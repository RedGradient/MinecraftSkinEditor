use std::cell::RefCell;
use std::rc::Rc;

use glium::Surface;
use gtk::gdk;
use gtk::glib;
use gtk::glib::Propagation;
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
        let widget = self.obj();
        widget.set_has_depth_buffer(true);
        widget.set_has_stencil_buffer(true);
        widget.set_allowed_apis(gdk::GLAPI::GL);
        widget.set_required_version(3, 3);
        self.parent_realize();

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

        let mut renderer = Renderer::new(context);
        renderer.set_viewport_size(widget.width(), widget.height());
        *self.renderer.borrow_mut() = Some(Rc::new(RefCell::new(renderer)));
    }

    fn unrealize(&self) {
        *self.renderer.borrow_mut() = None;
        self.parent_unrealize();
    }
}

impl GLAreaImpl for GliumGLArea {
    fn render(&self, _context: &gtk::gdk::GLContext) -> Propagation {
        let widget = self.obj();
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            let mut renderer = renderer.borrow_mut();
            renderer.set_viewport_size(widget.width(), widget.height());
            renderer.draw();
        }
        Propagation::Proceed
    }
}