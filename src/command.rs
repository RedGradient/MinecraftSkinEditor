use gtk::prelude::WidgetExt;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::ModelCell;
use crate::utils::random_brightness;

#[derive(Debug, Clone)]
pub enum Command {
    Draw { prev: ModelCell, new: ModelCell },
    Fill { body_part: BodyPart, fill_color: [f32; 4], prev_colors: Vec<ModelCell> },
}

impl Command {
    pub fn draw(target_cell: ModelCell, new_color: [f32; 4]) -> Command {
        let new = ModelCell {
            body_part: target_cell.body_part.clone(),
            cell_index: target_cell.cell_index,
            color: new_color
        };

        Command::Draw {
            prev: target_cell,
            new
        }
    }

    pub fn fill(body_part: &BodyPart, fill_color: &[f32; 4], prev_colors: Vec<ModelCell>) -> Command {
        Command::Fill {
            body_part: body_part.clone(),
            fill_color: fill_color.clone(),
            prev_colors
        }
    }

    pub fn random_draw(target_cell: ModelCell, color: [f32; 4]) -> Command {
        // Command::draw(target_cell, Command::adjust_color(color, 0.1))
        // random_saturation_adjustment
        // Command::draw(target_cell, random_saturation_adjustment(color, 0.2))
        // random_brightness
        Command::draw(target_cell, random_brightness(color))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tool {
    Pencil,
    Rubber,
    ColorPicker,
    Fill,
    Random,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Pencil
    }
}

pub struct DrawingHistory {
    gl_area: GliumArea,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    last_modified_cell: Option<ModelCell>,
}

impl DrawingHistory {
    pub fn new(gl_area: GliumArea) -> DrawingHistory {
        DrawingHistory { gl_area, undo_stack: vec![], redo_stack: vec![], last_modified_cell: None }
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
            .expect("Error popping a command from undo_stack");

        self._undo(&command);
        self.redo_stack.push(command);

        self.last_modified_cell.take();
    }

    pub fn redo(&mut self) {
        if self.redo_stack.is_empty() { return }
        let command = self.redo_stack.pop()
            .expect("Error popping a command from redo_stack.");

        self._execute(&command);
        self.undo_stack.push(command);

        self.last_modified_cell.take();
    }

    pub fn get_last_modified(&self) -> Option<ModelCell> {
        self.last_modified_cell
    }
    pub fn set_last_modified(&mut self, cell: ModelCell) {
        self.last_modified_cell.replace(cell);
    }
    
    fn _execute(&self, command: &Command) {
        match command {
            Command::Draw { prev, new } => {
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

    fn _undo(&mut self, command: &Command) {
        match command {
            Command::Draw { prev, new } => {
                let renderer = self.gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                renderer.set_cell(&prev);
                self.gl_area.queue_draw();
                self.last_modified_cell.replace(*new);
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
