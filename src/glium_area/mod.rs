use std::cell::RefCell;
use std::rc::Rc;

use glium::backend::Backend;
use gtk::{gdk, glib};
use gtk::prelude::{GLAreaExt, IsA, WidgetExt};
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};

use crate::glium_area::renderer::Renderer;
use crate::window::Window;

mod imp;
mod input;
pub mod camera;
pub mod renderer;
mod vertex;
pub(crate) mod model_object;
mod model;
pub mod skin_parser;
mod ray;
pub mod hover;
mod mouse_move;
pub mod body_part;
mod cross_info;
pub mod cube_side;

glib::wrapper! {
    pub struct GliumArea(ObjectSubclass<imp::GliumGLArea>)
        @extends gtk::Widget, gtk::GLArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

unsafe impl Backend for GliumArea {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        // GLArea swaps buffers in its render vfunc.
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::ffi::c_void {
        epoxy::get_proc_addr(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let scale = self.scale_factor();
        let width = self.width();
        let height = self.height();
        ((width * scale) as u32, (height * scale) as u32)
    }

    fn resize(&self, _new_size: (u32, u32)) {}

    fn is_current(&self) -> bool {
        match self.context() {
            Some(context) => gdk::GLContext::current() == Some(context),
            None => false,
        }
    }

    unsafe fn make_current(&self) {
        GLAreaExt::make_current(self);
    }
}

impl GliumArea {
    pub fn new() -> GliumArea {
        glib::Object::new()
    }

    pub fn setup(&self, win: &Window) {
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.connect_signals(win);
    }

    pub fn renderer(&self) -> Option<Rc<RefCell<Renderer>>> {
        self.imp().renderer.borrow().clone()
    }
}
