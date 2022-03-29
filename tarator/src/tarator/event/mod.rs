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
}

pub enum EventType {
    NONE = 0,
    WINDOWCLOSE, WINDOWRESIZE, WINDOWFOCUS, WINDOWLOSTFOCUS, WINDOWMOVED,
    APPLICATIONTICK, APPLICATIONUPDATE, APPLICATIONRENDER,
    KEYPRESSED, KEYRELEASED, KEYTYPED,
    MOUSEKEYPRESSED, MOUSEKEYRELEASED, MOUSEMOVED, MOUSESCROLLED
}

/// # Event
pub trait Event {
    fn get_category(&self) -> &EventCategory;
    fn get_type(&self) -> &EventType;
    #[allow(unused)]
    fn is_in_category(&self, category: EventCategory) -> bool { return matches!(self.get_category(), category); }
}
