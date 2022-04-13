//////////////////////////////////////////////////////////////////// /////////////////
/// # Implementation for:
/// # KeyEvent,
/// # KeyPressedEvent,
/// # KeyReleasedEvent,
/// # KeyTypedEvent 
/// Most Of This Is Still Boilerplatecode

use crate::tarator::event::*;

/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/// # KeyEvent
pub trait KeyEvent {
    fn get_key_code(&self) -> u32;
}


/// # KeyPressedEvent
pub struct KeyPressedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    key_code: u32,
    #[allow(unused)]
    repeat_count: u32,
    #[allow(unused)]
    handled: bool
}
impl Default for KeyPressedEvent {
    fn default() -> KeyPressedEvent {
        return KeyPressedEvent {
            event_category: EventCategory::KEYBOARD | EventCategory::INPUT,
            event_type: EventAction::KEYPRESSED,
            key_code: 0,
            repeat_count: 0,
            handled: false
        }
    }
}
impl KeyPressedEvent {
    #[allow(unused)]
    fn get_repeat_count(&self) -> u32 { return self.repeat_count; }
}
impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
crate::INTERN_EVENT_IMPLEMENT!(KeyPressedEvent);

/////////////////////////////////////////////////////////////////
/// # KeyReleasedEvent
pub struct KeyReleasedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    key_code: u32,
    #[allow(unused)]
    handled: bool
}
impl Default for KeyReleasedEvent {
    fn default() -> KeyReleasedEvent {
        return KeyReleasedEvent {
            event_category: EventCategory::KEYBOARD | EventCategory::INPUT,
            event_type: EventAction::KEYRELEASED,
            key_code: 0,
            handled: false
        }
    }
}
impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
crate::INTERN_EVENT_IMPLEMENT!(KeyReleasedEvent);

/////////////////////////////////////////////////////////////////
/// # KeyTypedEvent
/// This is for when you're writing inside input-fields or similar.
pub struct KeyTypedEvent {
    event_category: EventCategory,
    event_type: EventAction,
    key_code: u32,
    #[allow(unused)]
    handled: bool
}
impl Default for KeyTypedEvent {
    fn default() -> KeyTypedEvent {
        return KeyTypedEvent {
            event_category: EventCategory::KEYBOARD | EventCategory::INPUT,
            event_type: EventAction::KEYTYPED,
            key_code: 0,
            handled: false
        }
    }
}
impl KeyEvent for KeyTypedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
crate::INTERN_EVENT_IMPLEMENT!(KeyTypedEvent);
