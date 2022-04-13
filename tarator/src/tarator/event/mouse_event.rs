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
            event_type: EventAction::MOUSEMOVED,
            move_x: 0.0,
            move_y: 0.0,
            handled: false
        }
    }
}
impl MouseMovedEvent {
    #[allow(unused)]
    fn get_move_x(&self) -> f32 { return self.move_x; }
    #[allow(unused)]
    fn get_move_y(&self) -> f32 { return self.move_y; }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseMovedEvent);

/////////////////////////////////////////////////////////////////
/// # MouseScrolledEvent
pub struct MouseScrolledEvent {
    event_category: EventCategory,
    event_type: EventAction,
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
            event_type: EventAction::MOUSESCROLLED,
            offset_x: 0.0,
            offset_y: 0.0,
            handled: false
        }
    }
}
impl MouseScrolledEvent {
    #[allow(unused)]
    fn get_offset_x(&self) -> f32 { return self.offset_x; }
    #[allow(unused)]
    fn get_offset_y(&self) -> f32 { return self.offset_y; }
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
    handled: bool
}
impl Default for MouseKeyPressedEvent {
    fn default() -> MouseKeyPressedEvent {
        return MouseKeyPressedEvent {
            event_category: EventCategory::MOUSEKEY | EventCategory::INPUT,
            event_type: EventAction::KEYPRESSED,
            key_code: 0,
            handled: false
        }
    }
}
impl MouseKeyEvent for MouseKeyPressedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
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
            event_type: EventAction::KEYRELEASED,
            key_code: 0,
            handled: false
        }
    }
}
impl MouseKeyEvent for MouseKeyReleasedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
crate::INTERN_EVENT_IMPLEMENT!(MouseKeyReleasedEvent);
