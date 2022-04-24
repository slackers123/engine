use tarator::tarator::{
    layer::*,
    core::*
};

mod e {
    pub use eframe::{
        egui::{
            Context,
            CentralPanel,
            Window
        },
        run_native,
        NativeOptions,
        AppCreator
    };
}

pub struct EGUILayer {
    name: String,
    category: LayerCategory,
    ctx: UPtr<e::Context>,
    can_exit: bool,
    is_exiting: bool,
}
impl Layer for EGUILayer {
    fn new() -> EGUILayer {
        let egui_layer = EGUILayer{
            name: String::from("Egui Layer"),
            category: LayerCategory::GUI,
            ctx: UPtr::new(e::Context::default()),
            can_exit: false,
            is_exiting: false
        };
        return egui_layer;
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_category(&self) -> LayerCategory {
        return self.category;
    }
    fn update(&self) {
        
    }
    CASTIMPL!();
}
impl eframe::epi::App for EGUILayer {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }

    fn update(&mut self, ctx: &e::Context, frame: &mut eframe::Frame) {
        e::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
        });

        if self.is_exiting {
            e::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Not yet").clicked() {
                            self.is_exiting = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.can_exit = true;
                            frame.quit();
                        }
                    });
                });
        }
    }
}
