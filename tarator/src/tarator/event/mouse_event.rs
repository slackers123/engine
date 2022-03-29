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
    event_type: EventType,
    #[allow(unused)]
    move_x: f32,
    #[allow(unused)]
    move_y: f32,
    #[allow(unused)]
    handled: bool
}

impl Default for MouseMovedEvent {
    fn default() -> MouseMovedEvent {
        return MouseMovedEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventType::MOUSEMOVED,
            move_x: 0.0,
            move_y: 0.0,
            handled: false
        }
    }
}

impl MouseMovedEvent {
    #[allow(unused)]
    fn get_move_x(&self) -> &f32 { return &self.move_x; }
    #[allow(unused)]
    fn get_move_y(&self) -> &f32 { return &self.move_y; }
}

impl Event for MouseMovedEvent {
    fn get_category(&self) -> &EventCategory { return &self.event_category; }
    fn get_type(&self) -> &EventType { return &self.event_type; }
}


/////////////////////////////////////////////////////////////////
/// # MouseScrolledEvent
pub struct MouseScrolledEvent {
    event_category: EventCategory,
    event_type: EventType,
    #[allow(unused)]
    offset_x: f32,
    #[allow(unused)]
    offset_y: f32,
    #[allow(unused)]
    handled: bool
}

impl Default for MouseScrolledEvent {
    fn default() -> MouseScrolledEvent {
        return MouseScrolledEvent {
            event_category: EventCategory::MOUSE | EventCategory::INPUT,
            event_type: EventType::MOUSESCROLLED,
            offset_x: 0.0,
            offset_y: 0.0,
            handled: false
        }
    }
}

impl MouseScrolledEvent {
    #[allow(unused)]
    fn get_offset_x(&self) -> &f32 { return &self.offset_x; }
    #[allow(unused)]
    fn get_offset_y(&self) -> &f32 { return &self.offset_y; }
}

impl Event for MouseScrolledEvent {
    fn get_category(&self) -> &EventCategory { return &self.event_category; }
    fn get_type(&self) -> &EventType { return &self.event_type; }
}




/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/// # Mouse Keys
pub trait MouseKeyEvent {
    fn get_key_code(&self) -> &u32;    
}


/////////////////////////////////////////////////////////////////
/// # MouseKeyPressedEvent
pub struct MouseKeyPressedEvent {
    event_category: EventCategory,
    event_type: EventType,
    key_code: u32,
    #[allow(unused)]
    handled: bool
}

impl Default for MouseKeyPressedEvent {
    fn default() -> MouseKeyPressedEvent {
        return MouseKeyPressedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventType::KEYPRESSED,
            key_code: 0,
            handled: false
        }
    }
}

impl Event for MouseKeyPressedEvent {
    fn get_category(&self) -> &EventCategory { return &self.event_category; }
    fn get_type(&self) -> &EventType { return &self.event_type; }
}

impl MouseKeyEvent for MouseKeyPressedEvent {
    fn get_key_code(&self) -> &u32 { return &self.key_code; }
}


/////////////////////////////////////////////////////////////////
/// # KeyReleasedEvent
pub struct MouseKeyReleasedEvent {
    event_category: EventCategory,
    event_type: EventType,
    #[allow(unused)]
    key_code: u32,
    #[allow(unused)]
    handled: bool
}

impl Default for MouseKeyReleasedEvent {
    fn default() -> MouseKeyReleasedEvent {
        return MouseKeyReleasedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventType::KEYRELEASED,
            key_code: 0,
            handled: false
        }
    }
}

impl Event for MouseKeyReleasedEvent {
    fn get_category(&self) -> &EventCategory { return &self.event_category; }
    fn get_type(&self) -> &EventType { return &self.event_type; }
}

impl MouseKeyEvent for MouseKeyReleasedEvent {
    fn get_key_code(&self) -> &u32 { return &self.key_code; }
}
