use crate::error::AppResult;
use crate::models::{Action, EventRecord, MouseButton};
use crate::repositories::SessionRepository;
use rdev::{listen, EventType};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tauri::Emitter;
use tokio::time::{sleep, Duration};
use crossbeam_channel::unbounded;
use tokio::sync::Mutex as TokioMutex;

pub struct RecorderService;

impl RecorderService {
    /// Start the recorder. This is a synchronous function designed to be called from a
    /// plain thread (we spawn internal threads / runtimes as needed).
    pub fn start_recording(
        is_recording: Arc<Mutex<bool>>,
        app_handle: tauri::AppHandle,
        // pass the shared repository mutex so background async task can lock it
        repository: Arc<TokioMutex<Box<dyn SessionRepository>>>,
        session_id: i64,
        in_flight: Option<Arc<AtomicUsize>>,
    ) -> AppResult<()> {
        let start = Instant::now();
        let last_pos = Arc::new(Mutex::new((-1, -1)));

        // channel 用作生产者/消费者队列，callback 只 push 到 tx，异步任务从 rx 读取并批量保存
        let (tx, rx) = unbounded::<EventRecord>();

        // 启动一个独立的线程运行一个专用的 Tokio runtime 来定期批量写入数据库。
        {
            let repository = repository.clone();
            let in_flight_bg = in_flight.clone();

            std::thread::spawn(move || {
                // 创建一个单线程的 Tokio 运行时用于异步数据库操作
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build tokio runtime for recorder flusher");

                rt.block_on(async move {
                    loop {
                        sleep(Duration::from_millis(500)).await;

                        // 批量拉取所有当前在通道里的事件
                        let mut batch = Vec::new();
                        while let Ok(ev) = rx.try_recv() {
                            batch.push(ev);
                        }

                        if !batch.is_empty() {
                            // lock repository inside async context
                            let repo = repository.lock().await;
                            if let Err(e) = repo.save_events(session_id, &batch).await {
                                eprintln!("Failed to save events: {:?}", e);
                            } else {
                                if let Some(counter) = in_flight_bg.as_ref() {
                                    counter.fetch_sub(batch.len(), Ordering::SeqCst);
                                }
                            }
                        }
                    }
                });
            });
        }

        let is_recording_cb = is_recording.clone();
        let tx_cb = tx.clone();
        let last_pos_cb = last_pos.clone();
        let in_flight_thread = in_flight.clone();

        std::thread::spawn(move || {
            let callback = move |event: rdev::Event| {
                let is_rec = *is_recording_cb.lock().unwrap();
                if !is_rec {
                    return;
                }

                let elapsed = start.elapsed().as_millis();

                // 先读取位置信息（避免双锁交叉）
                let (x, y) = *last_pos_cb.lock().unwrap();

                match event.event_type {
                    EventType::MouseMove { x, y } => {
                        *last_pos_cb.lock().unwrap() = (x as i32, y as i32);
                        let ev = EventRecord::new(
                            elapsed,
                            Action::MouseMove {
                                x: x as i32,
                                y: y as i32,
                            },
                        );
                        if tx_cb.send(ev).is_ok() {
                            if let Some(counter) = in_flight_thread.as_ref() {
                                counter.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }

                    EventType::ButtonPress(btn) => {
                        let button = MouseButton::from_rdev(&btn);

                        let ev = EventRecord::new(elapsed, Action::MouseDown { button, x, y });
                        if tx_cb.send(ev).is_ok() {
                            if let Some(counter) = in_flight_thread.as_ref() {
                                counter.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }

                    EventType::ButtonRelease(btn) => {
                        let button = MouseButton::from_rdev(&btn);

                        let ev = EventRecord::new(elapsed, Action::MouseUp { button, x, y });
                        if tx_cb.send(ev).is_ok() {
                            if let Some(counter) = in_flight_thread.as_ref() {
                                counter.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }

                    EventType::Wheel { delta_x, delta_y } => {
                        let ev = EventRecord::new(
                            elapsed,
                            Action::Wheel {
                                delta_x: delta_x as i32,
                                delta_y: delta_y as i32,
                                x,
                                y,
                            },
                        );
                        if tx_cb.send(ev).is_ok() {
                            if let Some(counter) = in_flight_thread.as_ref() {
                                counter.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }

                    EventType::KeyPress(key) => {
                        let ev = EventRecord::new(
                            elapsed,
                            Action::KeyPress {
                                key: format!("{:?}", key),
                            },
                        );
                        if tx_cb.send(ev).is_ok() {
                            if let Some(counter) = in_flight_thread.as_ref() {
                                counter.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }

                    _ => {}
                }

                // 发当前未保存计数给前端
                let count = in_flight_thread.as_ref().map(|c| c.load(Ordering::SeqCst)).unwrap_or(0usize);
                let _ = app_handle.emit("event-count", count);
            };

            if let Err(err) = listen(callback) {
                eprintln!("Listening failed: {:?}", err);
            }
        });

        Ok(())
    }

    /// Save recording helper used by the command module when stopping.
    /// This delegates to the repository implementation.
    pub async fn save_recording(
        session_id: i64,
        repository: &dyn SessionRepository,
    ) -> AppResult<usize> {
        // After recording, the flusher should already have persisted events.
        // To report how many events were stored, load them and return the count.
        let events = repository.load_events(session_id).await?;
        Ok(events.len())
    }
}
