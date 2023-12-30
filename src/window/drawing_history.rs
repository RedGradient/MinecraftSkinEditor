use gtk::prelude::WidgetExt;

use crate::glium_area::GliumArea;
use crate::glium_area::renderer::ModelCell;
use crate::window::imp::Command;

pub struct DrawingHistory {
    gl_area: GliumArea,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

impl DrawingHistory {
    pub fn new(gl_area: GliumArea) -> DrawingHistory {
        DrawingHistory { gl_area, undo_stack: vec![], redo_stack: vec![] }
    }

    pub fn add_command(&mut self, command: Command) -> bool {
        self._execute(&command);
        self.undo_stack.push(command);
        self.redo_stack.clear();
        true
    }

    pub fn undo(&mut self) {
        if self.undo_stack.is_empty() { return }
        let command = self.undo_stack.pop()
            .expect("Error popping a command from undo_stack.");

        self._undo(&command);

        self.redo_stack.push(command);
    }

    pub fn redo(&mut self) {
        if self.redo_stack.is_empty() { return }
        let command = self.redo_stack.pop()
            .expect("Error popping a command from redo_stack.");

        self._execute(&command);

        self.undo_stack.push(command);
    }

    fn _execute(&self, command: &Command) {
        match command {
            Command::Pencil { prev, new } => {
                // do redo for Pencil
                let renderer = self.gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                renderer.set_cell(&new);
                self.gl_area.queue_draw();
            }
            Command::Fill { body_part, fill_color, prev_colors } => {
                // do redo for Fill
                let renderer = self.gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();

                for cell in prev_colors {
                    let new_cell = ModelCell {
                        body_part: cell.body_part.clone(),
                        cell_index: cell.cell_index,
                        color: *fill_color,
                    };
                    renderer.set_cell(&new_cell);
                }

                self.gl_area.queue_draw();
            }
        }
    }

    fn _undo(&self, command: &Command) {
        match command {
            Command::Pencil { prev, new } => {
                // do undo for Pencil
                let renderer = self.gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                renderer.set_cell(&prev);
                self.gl_area.queue_draw();
            }
            Command::Fill { body_part, fill_color, prev_colors} => {
                // do undo for Fill
                let renderer = self.gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();

                for cell in prev_colors {
                    renderer.set_cell(cell);
                }

                self.gl_area.queue_draw();
            }
        }
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}