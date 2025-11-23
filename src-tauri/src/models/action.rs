use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Action {
    MouseMove { 
        x: i32, 
        y: i32 
    },
    MouseDown { 
        button: MouseButton, 
        x: i32, 
        y: i32 
    },
    MouseUp { 
        button: MouseButton, 
        x: i32, 
        y: i32 
    },
    Wheel { 
        delta_x: i32, 
        delta_y: i32, 
        x: i32, 
        y: i32 
    },
    KeyPress { 
        key: String 
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Action {
    pub fn action_type(&self) -> &'static str {
        match self {
            Action::MouseMove { .. } => "MouseMove",
            Action::MouseDown { .. } => "MouseDown",
            Action::MouseUp { .. } => "MouseUp",
            Action::Wheel { .. } => "Wheel",
            Action::KeyPress { .. } => "KeyPress",
        }
    }
}