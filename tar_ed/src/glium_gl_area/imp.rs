use glium::{
    implement_vertex
};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use tarator::platform::glium::{
    render::*,
    buffer::*,
    vertex_array::*
};
use tarator::{
    core::*,
    math::*,
    render::{
        vertex_array::*,
        *
    }
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

#[derive(Default)]
pub struct GliumGLArea {
    renderer: RefCell<Option<GliumRenderAPI>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GliumGLArea {
    const NAME: &'static str = "GliumGLArea";
    type Type = super::GliumGLArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for GliumGLArea {}

impl WidgetImpl for GliumGLArea {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        if widget.error().is_some() {
            return;
        }

        // SAFETY: we know the GdkGLContext exists as we checked for errors above, and we haven't
        // done any operations on it which could lead to glium's state mismatch. (In theory, GTK
        // doesn't do any state-breaking operations on the context either.)
        //
        // We will also ensure glium's context does not outlive the GdkGLContext by destroying it in
        // `unrealize()`.
        let context =
            unsafe { glium::backend::Context::new(self.instance(), true, Default::default()) }
                .unwrap();
        *self.renderer.borrow_mut() = Some(GliumRenderAPI::new(context));
    }

    fn unrealize(&self, widget: &Self::Type) {
        *self.renderer.borrow_mut() = None;

        self.parent_unrealize(widget);
    }
}

impl GLAreaImpl for GliumGLArea {
    fn render(&self, _gl_area: &Self::Type, _context: &gtk::gdk::GLContext) -> bool {
        let mut vertex_array = GliumVertexArray::new(&self.renderer.borrow().as_ref().unwrap().ctx);
        vertex_array.set_index_buffer(SPtr::new(
            GliumIndexBuffer::new(
                &self.renderer.borrow().as_ref().unwrap().ctx,
                &[0, 1, 2])
        ));
        vertex_array.push_vertex_buffer(SPtr::new(
            GliumVertexBuffer::new(
                &self.renderer.borrow().as_ref().unwrap().ctx,
                &[GliumVertex{position: [-0.5, -0.5]},
                GliumVertex{position: [0.0, 0.5]},
                GliumVertex{position: [0.5, -0.5]}],
                0
            )
        ));
        self.renderer.borrow().as_ref().unwrap().draw_indexed(SPtr::new(vertex_array));
        return true;
    }
}