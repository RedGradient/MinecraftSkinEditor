use std::cell::RefCell;
use std::rc::Rc;

use glium::Surface;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::glium_area::renderer::Renderer;

#[derive(Default)]
pub struct GliumGLArea {
    pub renderer: RefCell<Option<Rc<RefCell<Renderer>>>>
    // 1. RefCell<...> - for assign Renderer by immutable reference in "WidgetImpl::realize" function
    // 2. Option<...> - for Default trait implementation and setting "None" in "WidgetImpl::unrealize" function
}

#[glib::object_subclass]
impl ObjectSubclass for GliumGLArea {
    const NAME: &'static str = "GliumGLArea";
    type Type = super::GliumArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for GliumGLArea {}

impl WidgetImpl for GliumGLArea {
    fn realize(&self) {
        self.parent_realize();
        let widget = self.obj();
        widget.set_has_depth_buffer(true);
        widget.set_has_stencil_buffer(true);

        if widget.error().is_some() {
            return;
        }

        let context = unsafe {
            glium::backend::Context::new(
                widget.clone(),
                true,
                Default::default()
            )
        }.unwrap();

        let renderer = Renderer::new(context);
        let renderer = Rc::new(RefCell::new(renderer));

        // *self.renderer.borrow_mut() = Some(Renderer::new(context));
        *self.renderer.borrow_mut() = Some(renderer);
    }

    fn unrealize(&self) {
        *self.renderer.borrow_mut() = None;
        self.parent_unrealize();
    }
}

impl GLAreaImpl for GliumGLArea {
    fn render(&self, _context: &gtk::gdk::GLContext) -> bool {
        // self.renderer.borrow_mut().as_mut().unwrap().draw();
        self.renderer.borrow().as_ref().unwrap().borrow_mut().draw();
        true
    }
}