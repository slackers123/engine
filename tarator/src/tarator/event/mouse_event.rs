/////////////////////////////////////////////////////////////////
/// # Implementation for:
/// # MouseMovedEvent,
/// # MouseScrolledEvent,
/// # MouseButtonEvent,
/// # MouseButtonPressedEvent,
/// # MouseButtonReleasedEvent
/// Most Of This Is Still Boilerplatecode


use crate::tarator::event::*;

/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/// # MouseMovedEvent
pub struct MouseMovedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    #[allow(unused)]
    move_x: f64,
    #[allow(unused)]
    move_y: f64,
    #[allow(unused)]
    handled: bool
}
impl Default for MouseMovedEvent {
    fn default() -> MouseMovedEvent {
        return MouseMovedEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventAction::MOUSEMOVED,
            move_x: 0.0,
            move_y: 0.0,
            handled: false
        }
    }
}
impl MouseMovedEvent {
    #[allow(unused)]
    pub fn get_move_x(&self) -> f64 { return self.move_x; }
    #[allow(unused)]
    pub fn get_move_y(&self) -> f64 { return self.move_y; }
    #[allow(unused)]
    pub fn new(x: f64, y: f64) -> MouseMovedEvent {
        return MouseMovedEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventAction::MOUSEMOVED,
            move_x: x,
            move_y: y,
            handled: false
        }
    }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseMovedEvent);

/////////////////////////////////////////////////////////////////
/// # MouseScrolledEvent
pub struct MouseScrolledEvent {
    event_category: EventCategory,
    event_type: EventAction,
    #[allow(unused)]
    offset_x: f64,
    #[allow(unused)]
    offset_y: f64,
    #[allow(unused)]
    handled: bool
}
impl Default for MouseScrolledEvent {
    fn default() -> MouseScrolledEvent {
        return MouseScrolledEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventAction::MOUSESCROLLED,
            offset_x: 0.0,
            offset_y: 0.0,
            handled: false
        }
    }
}
impl MouseScrolledEvent {
    #[allow(unused)]
    pub fn get_offset_x(&self) -> f64 { return self.offset_x; }
    #[allow(unused)]
    pub fn get_offset_y(&self) -> f64 { return self.offset_y; }
    #[allow(unused)]
    pub fn new(x: f64, y: f64) -> MouseScrolledEvent {
        return MouseScrolledEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventAction::MOUSESCROLLED,
            offset_x: x,
            offset_y: y,
            handled: false
        }
    }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseScrolledEvent);


/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/// # Mouse Keys
pub trait MouseKeyEvent {
    fn get_key_code(&self) -> u32;    
}

/////////////////////////////////////////////////////////////////
/// # MouseKeyPressedEvent
pub struct MouseKeyPressedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    key_code: u32,
    #[allow(unused)]
    repeat_count: u32,
    #[allow(unused)]
    handled: bool
}
impl Default for MouseKeyPressedEvent {
    fn default() -> MouseKeyPressedEvent {
        return MouseKeyPressedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventAction::MOUSEKEYPRESSED,
            key_code: 0,
            repeat_count: 0,
            handled: false
        }
    }
}
impl MouseKeyEvent for MouseKeyPressedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
impl MouseKeyPressedEvent {
    #[allow(unused)]
    pub fn new(keycode: u32, repeat_count: u32) -> MouseKeyPressedEvent {
        return MouseKeyPressedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventAction::MOUSEKEYPRESSED,
            key_code: keycode,
            repeat_count: repeat_count,
            handled: false
        }
    }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseKeyPressedEvent);

/////////////////////////////////////////////////////////////////
/// # KeyReleasedEvent
pub struct MouseKeyReleasedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    #[allow(unused)]
    key_code: u32,
    #[allow(unused)]
    handled: bool
}
impl Default for MouseKeyReleasedEvent {
    fn default() -> MouseKeyReleasedEvent {
        return MouseKeyReleasedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventAction::MOUSEKEYRELEASED,
            key_code: 0,
            handled: false
        }
    }
}
impl MouseKeyEvent for MouseKeyReleasedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
impl MouseKeyReleasedEvent {
    #[allow(unused)]
    pub fn new(keycode: u32) -> MouseKeyReleasedEvent {
        return MouseKeyReleasedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventAction::MOUSEKEYRELEASED,
            key_code: keycode,
            handled: false
        }
    }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseKeyReleasedEvent);
