pub mod application_event;
pub mod key_event;
pub mod mouse_event;

bitflags! {
    /// ## EventCategory
    /// Bitmask for which type of input it is
    pub struct EventCategory: u8 {
        const NONE        = 0;
        const APPLICATION = BIT!(0);
        const INPUT       = BIT!(1);
        const KEYBOARD    = BIT!(2);
        const MOUSE       = BIT!(3);
        const MOUSEKEY    = BIT!(4);
    }
    /// ## EventAction
    /// Bitmask for which input action it is
    pub struct EventAction: u16 {
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
/// ## Event
/// Event Base Trait, implemented in the event structs
pub trait Event {
    fn get_category(&self) -> EventCategory;
    fn get_action(&self) -> EventAction;
    fn get_handled(&self) -> bool;
    fn is_in_category(&self, category: EventCategory) -> bool { return self.get_category() == category; }
    fn is_in_action(&self, action: EventAction) -> bool { return self.get_action() == action; }
    fn set_handled_callback(&mut self, func: &fn()->bool);
    fn as_any(&self) -> &dyn std::any::Any;
}
/// ## Event Implementations
/// Gets rid of boilerplate code for the implementation of every single event struct
#[macro_export]
macro_rules! INTERN_EVENT_IMPLEMENT {
    ($label:tt) => {
        impl Event for $label {
            fn get_category(&self) -> EventCategory { return self.event_category; }
            fn get_action(&self) -> EventAction { return self.event_type; }
            fn get_handled(&self) -> bool { return self.handled; }
            fn set_handled_callback(&mut self, func: &fn()->bool) { self.handled = func(); }
            fn as_any(&self) -> &dyn std::any::Any {
                return self;
            }
        }
    };
}
