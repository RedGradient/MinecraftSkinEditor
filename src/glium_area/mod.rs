use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;

use glium::backend::Backend;
use gtk::{EventControllerScroll, EventControllerScrollFlags, gdk, GestureClick, glib};
use gtk::gdk::RGBA;
use gtk::glib::{clone, Propagation};
use gtk::prelude::{GestureExt, GLAreaExt, IsA, ToggleButtonExt, WidgetExt};
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};

use crate::command::*;
use crate::command::Tool;
use crate::glium_area::hover::Hover;
use crate::glium_area::renderer::{ModelCell, Renderer};
use crate::utils;
use crate::utils::{random_brightness, rgba_to_f32};
use crate::window::Window;

mod imp;
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
        @extends gtk::GLArea, gtk::Widget;
}

unsafe impl Backend for GliumArea {
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
        let rend = self.imp().renderer.clone();
        match rend.into_inner() {
            Some(renderer ) => Some(renderer.clone()),
            None => None
        }
    }

    pub fn connect_signals(&self, win: &Window) {
        self.show_fps();
        self.connect_scroll();
        self.connect_click(win);
    }

    fn connect_click(&self, win: &Window) {
        let click_handler = self.get_click_handler(win);
        let click = GestureClick::new();
        click.connect_begin(move |gesture, seq| {
            let point = gesture.point(seq).expect("Unable to get current point from drag gesture");
            let (x, y) = (point.0 as f32, point.1 as f32);
            click_handler(x, y, false);
        });

        let click_handler = self.get_click_handler(win);
        let g = self.clone();
        click.connect_update(move |gesture, seq| {
            let point = gesture.point(seq).expect("Unable to get current point from drag gesture");
            let (x, y) = (point.0 as f32, point.1 as f32);
            let renderer = g.renderer().unwrap();

            let mouse_hover_opt = renderer.borrow().get_mouse_hover();

            if let Some(mouse_hover) = mouse_hover_opt {
                match mouse_hover {
                    Hover::OnModel => {
                        click_handler(x, y, true);
                    },
                    Hover::OnEmptyArea => {
                        renderer.borrow_mut().mouse_move(x, y);
                        renderer.borrow_mut().update_camera();
                    },
                }
                g.queue_draw();
            }
        });

        let g = self.clone();
        click.connect_end(move |_, _| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            renderer.stop_motion();
            renderer.set_mouse_hover(None);
        });

        self.add_controller(click);
    }

    fn connect_scroll(&self) {
        let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
        let g = self.clone();
        scroll.connect_scroll(move |_, _, y| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();

            let distance = (y as f32) * 0.025;
            renderer.update_scale(distance);

            g.queue_draw();
            Propagation::Proceed
        });

        self.add_controller(scroll);
    }

    fn show_fps(&self) {
        // --- CALCULATE FPS ---
        let frame_count = Rc::new(Cell::new(0));
        let current_time = Rc::new(Cell::new(Instant::now()));
        let previous_time = Rc::new(Cell::new(Instant::now()));
        self.connect_render(
            clone!(@strong frame_count, @strong current_time, @strong previous_time => move |_, _| {
                frame_count.set(frame_count.get() + 1);
                current_time.set(Instant::now());
                let elapsed_time = current_time.get().duration_since(previous_time.get());
                if elapsed_time.as_secs_f32() > 1.0 {
                    let fps = (frame_count.get() as f32) / elapsed_time.as_secs_f32();
                    // println!("FPS: {:.2}", fps);
                    frame_count.set(0);
                    previous_time.set(current_time.get());
                }
                Propagation::Proceed
            })
        );
    }

    fn get_click_handler(&self, win: &Window) -> impl Fn(f32, f32, bool) {
        let gl_area = self.clone();
        let win = win.clone();
        move |x, y, updating| {
            let renderer = gl_area.renderer().unwrap();
            let cell_opt = renderer.borrow().get_cell(x, y, false);
            if cell_opt.is_none() {
                if !updating { renderer.borrow_mut().set_mouse_hover(Some(Hover::OnEmptyArea)); }
                renderer.borrow_mut().start_motion(x, y);
                return
            }
            if !updating {
                renderer.borrow_mut().set_mouse_hover(Some(Hover::OnModel));
            }
            drop(renderer);
            if !win.is_tool_active() {
                gl_area.queue_draw();
                return
            }
            let cell = cell_opt.unwrap();

            match win.imp().current_tool.get() {
                Tool::Pencil => GliumArea::handle_pencil(gl_area.clone(), cell, win.clone()),
                Tool::Rubber => GliumArea::handle_rubber(gl_area.clone(), cell, win.clone()),
                Tool::Fill => GliumArea::handle_fill(gl_area.clone(), cell, win.clone()),
                Tool::Random => GliumArea::handle_random(gl_area.clone(), cell, win.clone()),
                Tool::Replace => GliumArea::handle_replace(gl_area.clone(), cell, win.clone()),
                Tool::ColorPicker => GliumArea::handle_color_picker(x, y, win.clone()),
            }

            gl_area.queue_draw();
        }
    }
    
    fn handle_color_picker(x: f32, y: f32, win: Window) {
        let renderer = win.imp().gl_area.renderer().unwrap();
        let clicked_cell = renderer.borrow().get_cell(x, y, true);
        if let Some(cell) = clicked_cell {
            let rgba = utils::f32_to_rgba(cell.color);
            win.imp().color_button.set_rgba(&rgba);
            win.imp().pencil.set_active(true);
        }
    }
    
    fn handle_pencil(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let color = rgba_to_f32(win.imp().color_button.rgba());
        let trying_draw_same_cell = match win.get_last_modified_cell() {
            Some(last) => last.same_cell(cell),
            None => false,
        };
        if !trying_draw_same_cell {
            let draw_command = Box::new(Draw::new(gl_area.clone(), cell, color));
            win.add_command_to_history(draw_command);
            win.set_last_modified(cell);
        }
    }

    fn handle_replace(gl_area: GliumArea, cell: ModelCell, win: Window) {
        if cell.color[3] == 0.0 {
            return
        }
        let rgba = win.imp().color_button.rgba();
        let new_color: [f32; 4] = [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()];
        let command = Box::new(Replace::new(gl_area.clone(), cell.color, new_color));
        win.add_command_to_history(command);
    }
    
    fn handle_random(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let color = rgba_to_f32(win.imp().color_button.rgba());
        let trying_draw_same_cell = match win.get_last_modified_cell() {
            Some(last) => last.same_cell(cell),
            None => false,
        };
        if !trying_draw_same_cell {
            let draw_command = Box::new(Draw::new(gl_area.clone(), cell, random_brightness(color)));
            win.add_command_to_history(draw_command);
            win.set_last_modified(cell);
        }
    }
    
    fn handle_rubber(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let command = Box::new(Draw::new(gl_area.clone(), cell, [0.0, 0.0, 0.0, 0.0]));
        win.add_command_to_history(command);
    }
    
    fn handle_fill(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let renderer = gl_area.renderer().unwrap();
        let cells_to_fill = renderer.borrow().get_side_cells(&cell.body_part, cell.cell_index).unwrap();
        let new_color = rgba_to_f32(win.imp().color_button.rgba());
        let fill_command = Fill::new(gl_area.clone(), cell.body_part, new_color, cells_to_fill.clone());
        win.add_command_to_history(Box::new(fill_command));
    }
}