use crate::models::{Action, MouseButton};
use crate::repositories::SessionRepository;
use crate::error::AppResult;
use enigo::*;
use std::time::{Duration, Instant};

pub struct PlayerService;

impl PlayerService {
    pub async fn play_session(
        session_id: i64,
        repository: &dyn SessionRepository,
    ) -> AppResult<()> {
        let records = repository.load_events(session_id).await?;
        
        if records.is_empty() {
            println!("No events in this session.");
            return Ok(());
        }

        println!("Loading {} events, playback will begin in 3 seconds...", records.len());
        std::thread::sleep(Duration::from_secs(3));

        let mut enigo = Enigo::new();
        let mut last_ts = records[0].timestamp_ms;
        let start = Instant::now();

        for rec in records {
            let wait_ms = if rec.timestamp_ms >= last_ts {
                rec.timestamp_ms - last_ts
            } else {
                0
            };
            std::thread::sleep(Duration::from_millis(wait_ms as u64));
            
            Self::execute_action(&mut enigo, &rec.action);
            last_ts = rec.timestamp_ms;
        }

        println!("Playback complete, duration: {:?}", start.elapsed());
        Ok(())
    }
    
    fn execute_action(enigo: &mut Enigo, action: &Action) {
        match action {
            Action::MouseMove { x, y } => {
                enigo.mouse_move_to(*x, *y);
            }
            Action::MouseDown { button, .. } => {
                let btn = Self::convert_button(button);
                enigo.mouse_down(btn);
            }
            Action::MouseUp { button, .. } => {
                let btn = Self::convert_button(button);
                enigo.mouse_up(btn);
            }
            Action::Wheel { delta_x, delta_y, .. } => {
                if *delta_y != 0 {
                    enigo.mouse_scroll_y(*delta_y);
                }
                if *delta_x != 0 {
                    enigo.mouse_scroll_x(*delta_x);
                }
            }
            Action::KeyPress { key } => {
                if let Some(ch) = Self::extract_char(key) {
                    enigo.key_sequence(&ch.to_string());
                }
            }
        }
    }
    
    fn convert_button(button: &MouseButton) -> enigo::MouseButton {
        match button {
            MouseButton::Left => enigo::MouseButton::Left,
            MouseButton::Right => enigo::MouseButton::Right,
            MouseButton::Middle => enigo::MouseButton::Middle,
        }
    }
    
    fn extract_char(s: &str) -> Option<char> {
        if let Some(pos) = s.find("Layout(") {
            let part = &s[pos..];
            for ch in part.chars() {
                if ch.is_ascii_alphabetic() {
                    return Some(ch);
                }
            }
        }
        s.chars().rev().find(|c| c.is_ascii_alphanumeric())
    }
}