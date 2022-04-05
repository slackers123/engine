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
    event_type: EventType,
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
            event_type: EventType::KEYPRESSED,
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

crate::INTERN_IMPLEVENT!(KeyPressedEvent);

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}


/////////////////////////////////////////////////////////////////
/// # KeyReleasedEvent
pub struct KeyReleasedEvent {
    event_category: EventCategory,
    event_type: EventType,
    key_code: u32,
    #[allow(unused)]
    handled: bool
}

impl Default for KeyReleasedEvent {
    fn default() -> KeyReleasedEvent {
        return KeyReleasedEvent {
            event_category: EventCategory::KEYBOARD | EventCategory::INPUT,
            event_type: EventType::KEYRELEASED,
            key_code: 0,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(KeyReleasedEvent);

impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}


/////////////////////////////////////////////////////////////////
/// # KeyTypedEvent
/// This is for when you're writing inside input-fields or similar.
pub struct KeyTypedEvent {
    event_category: EventCategory,
    event_type: EventType,
    key_code: u32,
    #[allow(unused)]
    handled: bool
}

impl Default for KeyTypedEvent {
    fn default() -> KeyTypedEvent {
        return KeyTypedEvent {
            event_category: EventCategory::KEYBOARD | EventCategory::INPUT,
            event_type: EventType::KEYTYPED,
            key_code: 0,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(KeyTypedEvent);

impl KeyEvent for KeyTypedEvent {
    fn get_key_code(&self) -> u32 { return self.key_code; }
}
