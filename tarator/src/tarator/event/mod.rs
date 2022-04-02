/// # Event
/// TODO: Event Dispatcher

pub mod application_event;
pub mod key_event;
pub mod mouse_event;

bitflags! {
    pub struct EventCategory: u8 {
        const NONE        = 0;
        const APPLICATION = BIT!(0);
        const INPUT       = BIT!(1);
        const KEYBOARD    = BIT!(2);
        const MOUSE       = BIT!(3);
        const MOUSEKEY    = BIT!(4);
    }

    pub struct EventType: u16 {
        const NONE                = 0;
        const WINDOWCLOSE         = BIT!(0); 
        const WINDOWRESIZE        = BIT!(1);
        const WINDOWFOCUS         = BIT!(2);
        const WINDOWLOSTFOCUS     = BIT!(3);
        const WINDOWMOVED         = BIT!(4);
        const APPLICATIONTICK     = BIT!(5);
        const APPLICATIONUPDATE   = BIT!(6);
        const APPLICATIONRENDER   = BIT!(7);
        const KEYPRESSED          = BIT!(8);
        const KEYRELEASED         = BIT!(9);
        const KEYTYPED            = BIT!(10);
        const MOUSEKEYPRESSED     = BIT!(11);
        const MOUSEKEYRELEASED    = BIT!(12);
        const MOUSEMOVED          = BIT!(13);
        const MOUSESCROLLED       = BIT!(14);
    }
}
/// # Event
pub trait Event {
    fn get_category(&self) -> &EventCategory;
    fn get_type(&self) -> &EventType;
    #[allow(unused)]
    fn is_in_category(&self, category: EventCategory) -> bool { return matches!(self.get_category(), category); }
    fn set_handled_callback(&mut self, func: &fn()->bool);
}

/// # Event Implementations
#[macro_export]
macro_rules! INTERN_IMPLEVENT {
    ($label:tt) => {
        impl Event for $label {
            fn get_category(&self) -> &EventCategory { return &self.event_category; }
            fn get_type(&self) -> &EventType { return &self.event_type; }
            fn set_handled_callback(&mut self, func: &fn()->bool) { self.handled = func(); }
        }
    };
}




/// # EventDispatcher
#[allow(unused)]
struct EventDispatcher {
    event: &'static mut dyn Event
}

impl EventDispatcher {
    #[allow(unused)]
    fn dispatch(&mut self, event_type: EventType, func: fn()->bool) -> bool {
        if *self.event.get_type() == event_type {
            self.event.set_handled_callback(&func);
            return true;
        }
        return false;
    }
}
