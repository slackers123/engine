use crate::{
    tarator::{
        layer::*,
        window::*,
    },
    platform::glfw::glfw_window::*
};
pub struct GLFWEGUILayer {
    name: String,
    category: LayerCategory,
    #[allow(unused)]
    ctx: egui_glfw_gl::egui::CtxRef,
    pub input: egui_glfw_gl::EguiInputState,
    painter: egui_glfw_gl::Painter,
    func: fn(&mut egui_glfw_gl::egui::Ui),
    nativeppp: f32
}
impl Layer for GLFWEGUILayer {
    fn new() -> GLFWEGUILayer {
        return GLFWEGUILayer::new_glfw_egui(&mut GLFWWindow::new(&WindowProps::default()), |ui|{});
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_category(&self) -> LayerCategory {
        return self.category;
    }
    fn update_mut(&mut self, delta: f64) {
        self.input.input.time = Some(delta);
        self.ctx.begin_frame(self.input.input.take());
        self.input.input.pixels_per_point = Some(self.nativeppp);
        unsafe{gl::Clear(gl::COLOR_BUFFER_BIT);}
        egui_glfw_gl::egui::Window::new("Egui with GLFW").show(&self.ctx, |ui| { (self.func)(ui); });
        let (output, cmds) = self.ctx.end_frame();
        let paint_jobs = self.ctx.tessellate(cmds);
        self.painter.paint_jobs(None, paint_jobs, &self.ctx.texture(), self.nativeppp);
    }
    CASTIMPL!();
}
impl GLFWEGUILayer {
    /// ## new_glfw_egui
    pub fn new_glfw_egui(window: &mut GLFWWindow, add_contents: fn(&mut egui_glfw_gl::egui::Ui)) -> GLFWEGUILayer {
        let width = window.get_width();
        let height = window.get_height();
        let ppp = window.get_native().get_content_scale().0;
        return GLFWEGUILayer {
            name: String::from("GLFWEGUILayer"),
            category: LayerCategory::GUI,
            ctx: egui_glfw_gl::egui::CtxRef::default(),
            input: egui_glfw_gl::EguiInputState::new(egui_glfw_gl::egui::RawInput {
                screen_rect: Some(egui_glfw_gl::egui::Rect::from_min_size(
                    egui_glfw_gl::egui::Pos2::new(0f32, 0f32),
                    egui_glfw_gl::egui::vec2(window.get_width() as f32, window.get_height() as f32))),
                    pixels_per_point: Some(ppp),
                    ..Default::default()
                }),
            painter: egui_glfw_gl::Painter::new(window.get_native(), width, height),
            func: add_contents,
            nativeppp: ppp
        };
    }
}

