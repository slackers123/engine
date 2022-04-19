pub trait Input {
    fn is_key_pressed(keycode: u32) -> bool;
    fn is_mouse_key_pressed(keycode: u32) -> bool;
    fn get_mouse_position() -> (f32, f32);
}