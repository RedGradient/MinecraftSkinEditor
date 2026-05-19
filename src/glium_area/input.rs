use gtk::prelude::{GestureExt, ToggleButtonExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::command::*;
use crate::command::Tool;
use crate::glium_area::GliumArea;
use crate::glium_area::hover::Hover;
use crate::glium_area::renderer::ModelCell;
use crate::utils::{random_brightness, rgba_to_f32};
use crate::window::Window;

impl GliumArea {
    pub(super) fn connect_signals(&self, win: &Window) {
        self.connect_scroll();
        self.connect_click(win);
    }

    fn connect_click(&self, win: &Window) {
        let click_handler = self.get_click_handler(win);
        let click = gtk::GestureClick::new();
        click.connect_begin(move |gesture, seq| {
            let point = gesture
                .point(seq)
                .expect("Unable to get current point from drag gesture");
            click_handler(point.0 as f32, point.1 as f32, false);
        });

        let click_handler = self.get_click_handler(win);
        let gl_area = self.clone();
        click.connect_update(move |gesture, seq| {
            let point = gesture
                .point(seq)
                .expect("Unable to get current point from drag gesture");
            let (x, y) = (point.0 as f32, point.1 as f32);

            let Some(renderer) = gl_area.renderer() else {
                return;
            };
            let mut renderer = renderer.borrow_mut();

            let Some(mouse_hover) = renderer.get_mouse_hover() else {
                return;
            };

            match mouse_hover {
                Hover::OnModel => {
                    drop(renderer);
                    click_handler(x, y, true);
                }
                Hover::OnEmptyArea => {
                    renderer.mouse_move(x, y);
                    renderer.update_camera();
                    drop(renderer);
                }
            }
            gl_area.queue_draw();
        });

        let gl_area = self.clone();
        click.connect_end(move |_, _| {
            if let Some(renderer) = gl_area.renderer() {
                let mut renderer = renderer.borrow_mut();
                renderer.stop_motion();
                renderer.set_mouse_hover(None);
            }
        });

        self.add_controller(click);
    }

    fn connect_scroll(&self) {
        let scroll =
            gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        let gl_area = self.clone();
        scroll.connect_scroll(move |_, _, y| {
            if let Some(renderer) = gl_area.renderer() {
                renderer.borrow_mut().update_scale(y as f32 * 0.025);
                gl_area.queue_draw();
            }
            gtk::glib::Propagation::Proceed
        });
        self.add_controller(scroll);
    }

    fn get_click_handler(&self, win: &Window) -> impl Fn(f32, f32, bool) + 'static {
        let gl_area = self.clone();
        let win = win.clone();
        move |x, y, updating| {
            let Some(renderer_rc) = gl_area.renderer() else {
                return;
            };
            let mut renderer = renderer_rc.borrow_mut();
            let cell_opt = renderer.get_cell(x, y, false);
            if cell_opt.is_none() {
                if !updating {
                    renderer.set_mouse_hover(Some(Hover::OnEmptyArea));
                }
                renderer.start_motion(x, y);
                return;
            }
            if !updating {
                renderer.set_mouse_hover(Some(Hover::OnModel));
            }
            let cell = cell_opt.unwrap();
            drop(renderer);

            if !win.is_tool_active() {
                gl_area.queue_draw();
                return;
            }

            match win.imp().current_tool.get() {
                Tool::Pencil => Self::handle_pencil(gl_area.clone(), cell, win.clone()),
                Tool::Rubber => Self::handle_rubber(gl_area.clone(), cell, win.clone()),
                Tool::Fill => Self::handle_fill(gl_area.clone(), cell, win.clone()),
                Tool::Random => Self::handle_random(gl_area.clone(), cell, win.clone()),
                Tool::Replace => Self::handle_replace(gl_area.clone(), cell, win.clone()),
                Tool::ColorPicker => Self::handle_color_picker(&gl_area, x, y, &win),
            }

            gl_area.queue_draw();
        }
    }

    fn handle_color_picker(gl_area: &GliumArea, x: f32, y: f32, win: &Window) {
        let Some(renderer) = gl_area.renderer() else {
            return;
        };
        let mut renderer = renderer.borrow_mut();
        if let Some(cell) = renderer.get_cell(x, y, true) {
            let rgba = crate::utils::f32_to_rgba(cell.color);
            win.imp().color_button.set_rgba(&rgba);
            win.imp().pencil.set_active(true);
        }
    }

    fn handle_pencil(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let color = rgba_to_f32(win.imp().color_button.rgba());
        let trying_draw_same_cell = win
            .get_last_modified_cell()
            .is_some_and(|last| last.same_cell(cell));
        if !trying_draw_same_cell {
            win.add_command_to_history(Box::new(Draw::new(gl_area, cell, color)));
            win.set_last_modified(cell);
        }
    }

    fn handle_replace(gl_area: GliumArea, cell: ModelCell, win: Window) {
        if cell.color[3] == 0.0 {
            return;
        }
        let rgba = win.imp().color_button.rgba();
        let new_color = [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()];
        win.add_command_to_history(Box::new(Replace::new(gl_area, cell.color, new_color)));
    }

    fn handle_random(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let color = rgba_to_f32(win.imp().color_button.rgba());
        let trying_draw_same_cell = win
            .get_last_modified_cell()
            .is_some_and(|last| last.same_cell(cell));
        if !trying_draw_same_cell {
            win.add_command_to_history(Box::new(Draw::new(
                gl_area,
                cell,
                random_brightness(color),
            )));
            win.set_last_modified(cell);
        }
    }

    fn handle_rubber(gl_area: GliumArea, cell: ModelCell, win: Window) {
        win.add_command_to_history(Box::new(Draw::new(
            gl_area,
            cell,
            [0.0, 0.0, 0.0, 0.0],
        )));
    }

    fn handle_fill(gl_area: GliumArea, cell: ModelCell, win: Window) {
        let Some(renderer) = gl_area.renderer() else {
            return;
        };
        let cells_to_fill = renderer
            .borrow()
            .get_side_cells(&cell.body_part, cell.cell_index)
            .unwrap();
        let new_color = rgba_to_f32(win.imp().color_button.rgba());
        win.add_command_to_history(Box::new(Fill::new(
            gl_area,
            cell.body_part,
            new_color,
            cells_to_fill,
        )));
    }
}
