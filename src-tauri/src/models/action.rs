use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "mouse_move")]
    MouseMove { 
        x: i32, 
        y: i32 
    },
    #[serde(rename = "mouse_down")]
    MouseDown { 
        button: MouseButton, 
        x: i32, 
        y: i32 
    },
    #[serde(rename = "mouse_up")]
    MouseUp { 
        button: MouseButton, 
        x: i32, 
        y: i32 
    },
    #[serde(rename = "wheel")]
    Wheel { 
        delta_x: i32, 
        delta_y: i32, 
        x: i32, 
        y: i32 
    },
    #[serde(rename = "key_press")]
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

impl MouseButton {
    pub fn from_rdev(btn: &rdev::Button) -> Self {
        match btn {
            rdev::Button::Left => MouseButton::Left,
            rdev::Button::Right => MouseButton::Right,
            rdev::Button::Middle => MouseButton::Middle,
            _ => MouseButton::Left,
        }
    }
    
    pub fn to_enigo(&self) -> enigo::Button {
        match self {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
        }
    }
}