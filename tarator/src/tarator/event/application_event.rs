/////////////////////////////////////////////////////////////////
/// # Implementation for:
/// # WindowResizeEvent,
/// # WindowCloseEvent,
/// # ApplicationTickEvent,
/// # ApplicationUpdateEvent,
/// # ApplicationRenderEvent
/// Most Of This Is Still Boilerplatecode

use crate::tarator::event::*;


/////////////////////////////////////////////////////////////////
/// # WindowResizeEvent
pub struct WindowResizeEvent {
    event_category: EventCategory,
    event_type: EventType,
    #[allow(unused)]
    size_x: u32,
    #[allow(unused)]
    size_y: u32,
    #[allow(unused)]
    handled: bool
}

impl Default for WindowResizeEvent {
    fn default() -> WindowResizeEvent {
        return WindowResizeEvent {
            event_category: EventCategory::APPLICATION,
            event_type: EventType::WINDOWRESIZE,
            size_x: 0,
            size_y: 0,
            handled: false
        };
    }
}

impl WindowResizeEvent {
    #[allow(unused)]
    fn get_size_x(&self) -> &u32 { return &self.size_x; }
    #[allow(unused)]
    fn get_size_y(&self) -> &u32 { return &self.size_y; }
}

crate::INTERN_IMPLEVENT!(WindowResizeEvent);

/////////////////////////////////////////////////////////////////
/// # WindowCloseEvent
pub struct WindowCloseEvent {
    #[allow(unused)]
    event_category: EventCategory,
    #[allow(unused)]
    event_type: EventType,
    #[allow(unused)]
    handled: bool
}

impl Default for WindowCloseEvent {
    fn default() -> WindowCloseEvent {
        return WindowCloseEvent {
            event_category: EventCategory::APPLICATION,
            event_type: EventType::WINDOWCLOSE,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(WindowCloseEvent);

/////////////////////////////////////////////////////////////////
/// # ApplicationTickEvent
pub struct ApplicationTickEvent {
    #[allow(unused)]
    event_category: EventCategory,
    #[allow(unused)]
    event_type: EventType,
    #[allow(unused)]
    handled: bool
}

impl Default for ApplicationTickEvent {
    fn default() -> ApplicationTickEvent {
        return ApplicationTickEvent {
            event_category: EventCategory::APPLICATION,
            event_type: EventType::APPLICATIONTICK,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(ApplicationTickEvent);

/////////////////////////////////////////////////////////////////
/// # ApplicationUpdateEvent
pub struct ApplicationUpdateEvent {
    #[allow(unused)]
    event_category: EventCategory,
    #[allow(unused)]
    event_type: EventType,
    #[allow(unused)]
    handled: bool
}

impl Default for ApplicationUpdateEvent {
    fn default() -> ApplicationUpdateEvent {
        return ApplicationUpdateEvent {
            event_category: EventCategory::APPLICATION,
            event_type: EventType::APPLICATIONUPDATE,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(ApplicationUpdateEvent);

/////////////////////////////////////////////////////////////////
/// # ApplicationRenderEvent
pub struct ApplicationRenderEvent {
    #[allow(unused)]
    event_category: EventCategory,
    #[allow(unused)]
    event_type: EventType,
    #[allow(unused)]
    handled: bool
}

impl Default for ApplicationRenderEvent {
    fn default() -> ApplicationRenderEvent {
        return ApplicationRenderEvent {
            event_category: EventCategory::APPLICATION,
            event_type: EventType::APPLICATIONRENDER,
            handled: false
        }
    }
}

crate::INTERN_IMPLEVENT!(ApplicationRenderEvent);
