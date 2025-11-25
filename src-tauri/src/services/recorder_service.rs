use crate::models::{Action, EventRecord, MouseButton};
use crate::repositories::SessionRepository;
use crate::error::AppResult;
use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use std::time::Instant;

lazy_static::lazy_static! {
    static ref GLOBAL_RECORDS: Arc<Mutex<Vec<EventRecord>>> = Arc::new(Mutex::new(Vec::new()));
}

pub struct RecorderService;

impl RecorderService {
    pub fn start_recording(
        is_recording: Arc<Mutex<bool>>,
        app_handle: tauri::AppHandle,
    ) -> AppResult<()> {
        // 清空之前的记录
        {
            let mut records = GLOBAL_RECORDS.lock().unwrap();
            records.clear();
        }
        
        let start = Instant::now();
        let last_pos = Arc::new(Mutex::new((-1, -1)));
        
        let callback = move |event: Event| {
            let is_rec = is_recording.lock().unwrap();
            if !*is_rec {
                return;
            }
            drop(is_rec);
            
            let elapsed = start.elapsed().as_millis();
            let mut recs = GLOBAL_RECORDS.lock().unwrap();
            
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    let mut pos = last_pos.lock().unwrap();
                    *pos = (x as i32, y as i32);
                    
                    recs.push(EventRecord::new(
                        elapsed,
                        Action::MouseMove { x: x as i32, y: y as i32 },
                    ));
                    
                    let _ = app_handle.emit("recording-event", 
                        format!("MouseMove: ({}, {})", x, y));
                }
                EventType::ButtonPress(btn) => {
                    let (x, y) = *last_pos.lock().unwrap();
                    let button = MouseButton::from_rdev(&btn);
                    
                    recs.push(EventRecord::new(
                        elapsed,
                        Action::MouseDown { button, x, y },
                    ));
                    
                    let _ = app_handle.emit("recording-event", 
                        format!("MouseDown: {:?}", btn));
                }
                EventType::ButtonRelease(btn) => {
                    let (x, y) = *last_pos.lock().unwrap();
                    let button = MouseButton::from_rdev(&btn);
                    
                    recs.push(EventRecord::new(
                        elapsed,
                        Action::MouseUp { button, x, y },
                    ));
                    
                    let _ = app_handle.emit("recording-event", 
                        format!("MouseUp: {:?}", btn));
                }
                EventType::Wheel { delta_x, delta_y } => {
                    let (x, y) = *last_pos.lock().unwrap();
                    
                    recs.push(EventRecord::new(
                        elapsed,
                        Action::Wheel {
                            delta_x: delta_x as i32,
                            delta_y: delta_y as i32,
                            x,
                            y,
                        },
                    ));
                    
                    let _ = app_handle.emit("recording-event", 
                        format!("Wheel: ({}, {})", delta_x, delta_y));
                }
                EventType::KeyPress(key) => {
                    recs.push(EventRecord::new(
                        elapsed,
                        Action::KeyPress {
                            key: format!("{:?}", key),
                        },
                    ));
                    
                    let _ = app_handle.emit("recording-event", 
                        format!("KeyPress: {:?}", key));
                }
                _ => {}
            };
            
            let _ = app_handle.emit("event-count", recs.len());
        };
        
        if let Err(err) = listen(callback) {
            eprintln!("Listening failed: {:?}", err);
        }
        
        Ok(())
    }
    
    pub async fn save_recording(
        session_id: i64,
        repository: &dyn SessionRepository,
    ) -> AppResult<usize> {
        // Clone records so we don't hold the MutexGuard across an await
        let records = {
            let r = GLOBAL_RECORDS.lock().unwrap();
            r.clone()
        };
        let count = records.len();

        if count > 0 {
            repository.save_events(session_id, &records).await?;
        }

        Ok(count)
    }
}