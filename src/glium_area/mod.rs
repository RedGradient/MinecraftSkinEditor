use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use glium::backend::Backend;
use gtk::{gdk, glib};
use gtk::prelude::{GLAreaExt, WidgetExt};
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use gtk::traits::NativeExt;

use crate::glium_area::renderer::Renderer;

use self::imp::GliumGLArea;

mod imp;
pub mod camera;
pub mod renderer;
mod vertex;
mod model_object;
mod model;
pub mod skin_parser;
mod ray;
pub mod hover_state;
mod mouse_move;
pub mod body_part;
mod cross_info;
pub mod cube_side;
pub mod skin_dialog;

glib::wrapper! {
    pub struct GliumArea(ObjectSubclass<imp::GliumGLArea>)
        @extends gtk::GLArea, gtk::Widget;
}

impl GliumArea {
    pub fn inner(&self) -> &GliumGLArea {
        GliumGLArea::from_obj(self)
    }

    pub fn renderer(&self) -> Option<Rc<RefCell<Renderer>>> {
        let rend = self.imp().renderer.clone();
        match rend.into_inner() {
            Some(renderer ) => Some(renderer.clone()),
            None => None
        }
    }
}

unsafe impl glium::backend::Backend for GliumArea {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        // We're supposed to draw (and hence swap buffers) only inside the `render()` vfunc or
        // signal, which means that GLArea will handle buffer swaps for us.
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

    fn resize(&self, new_size: (u32, u32)) { }

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
