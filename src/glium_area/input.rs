use gtk::prelude::{GestureExt, WidgetExt};

use crate::command::*;
use crate::command::Tool;
use crate::editor_host::EditorHost;
use crate::glium_area::GliumArea;
use crate::glium_area::hover::Hover;
use crate::glium_area::renderer::ModelCell;
use crate::utils::{random_brightness, rgba_to_f32};

impl GliumArea {
    pub(super) fn connect_signals<H: EditorHost + Clone + 'static>(&self, host: H) {
        self.connect_scroll();
        self.connect_click(host);
    }

    fn connect_click<H: EditorHost + Clone + 'static>(&self, host: H) {
        let click_handler = self.get_click_handler(host.clone());
        let click = gtk::GestureClick::new();
        click.connect_begin(move |gesture, seq| {
            let point = gesture
                .point(seq)
                .expect("Unable to get current point from drag gesture");
            click_handler(point.0 as f32, point.1 as f32, false);
        });

        let click_handler = self.get_click_handler(host);
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
                    gl_area.queue_draw();
                }
            }
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

    fn get_click_handler<H: EditorHost + Clone + 'static>(
        &self,
        host: H,
    ) -> impl Fn(f32, f32, bool) + 'static {
        let gl_area = self.clone();
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

            if !host.tools_enabled() {
                return;
            }

            match host.current_tool() {
                Tool::Pencil => Self::handle_pencil(gl_area.clone(), cell, &host),
                Tool::Rubber => Self::handle_rubber(gl_area.clone(), cell, &host),
                Tool::Fill => Self::handle_fill(gl_area.clone(), cell, &host),
                Tool::Random => Self::handle_random(gl_area.clone(), cell, &host),
                Tool::Replace => Self::handle_replace(gl_area.clone(), cell, &host),
                Tool::ColorPicker => Self::handle_color_picker(&gl_area, x, y, &host),
            }
        }
    }

    fn handle_color_picker<H: EditorHost>(gl_area: &GliumArea, x: f32, y: f32, host: &H) {
        let Some(renderer) = gl_area.renderer() else {
            return;
        };
        let mut renderer = renderer.borrow_mut();
        if let Some(cell) = renderer.get_cell(x, y, true) {
            let rgba = crate::utils::f32_to_rgba(cell.color);
            host.set_active_color(&rgba);
            host.select_pencil_tool();
        }
    }

    fn handle_pencil<H: EditorHost>(gl_area: GliumArea, cell: ModelCell, host: &H) {
        let color = rgba_to_f32(host.active_color());
        let trying_draw_same_cell = host
            .last_modified_cell()
            .is_some_and(|last| last.same_cell(cell));
        if !trying_draw_same_cell {
            host.add_command(Box::new(Draw::new(gl_area, cell, color)));
            host.set_last_modified(cell);
        }
    }

    fn handle_replace<H: EditorHost>(gl_area: GliumArea, cell: ModelCell, host: &H) {
        if cell.color[3] == 0.0 {
            return;
        }
        let rgba = host.active_color();
        let new_color = [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()];
        host.add_command(Box::new(Replace::new(gl_area, cell.color, new_color)));
    }

    fn handle_random<H: EditorHost>(gl_area: GliumArea, cell: ModelCell, host: &H) {
        let color = rgba_to_f32(host.active_color());
        let trying_draw_same_cell = host
            .last_modified_cell()
            .is_some_and(|last| last.same_cell(cell));
        if !trying_draw_same_cell {
            host.add_command(Box::new(Draw::new(
                gl_area,
                cell,
                random_brightness(color),
            )));
            host.set_last_modified(cell);
        }
    }

    fn handle_rubber<H: EditorHost>(gl_area: GliumArea, cell: ModelCell, host: &H) {
        host.add_command(Box::new(Draw::new(
            gl_area,
            cell,
            [0.0, 0.0, 0.0, 0.0],
        )));
    }

    fn handle_fill<H: EditorHost>(gl_area: GliumArea, cell: ModelCell, host: &H) {
        let Some(renderer) = gl_area.renderer() else {
            return;
        };
        let cells_to_fill = renderer
            .borrow()
            .get_side_cells(&cell.body_part, cell.cell_index)
            .unwrap();
        let new_color = rgba_to_f32(host.active_color());
        host.add_command(Box::new(Fill::new(
            gl_area,
            cell.body_part,
            new_color,
            cells_to_fill,
        )));
    }
}
