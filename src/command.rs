use std::cell::RefCell;

use gtk::prelude::WidgetExt;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::ModelCell;

pub trait Action {
    fn execute(&self);
    fn undo(&self);
    fn redo(&self);
}

pub struct Draw {
    gl_area: GliumArea,
    old_cell: ModelCell,
    new_cell: ModelCell,
}
impl Draw {
    pub fn new(gl_area: GliumArea, cell: ModelCell, color: [f32; 4]) -> Draw {
        let new_cell = ModelCell {
            body_part: cell.body_part,
            cell_index: cell.cell_index,
            color
        };
        
        Draw {
            gl_area,
            old_cell: cell,
            new_cell,
        }
    }
}
impl Action for Draw {
    fn execute(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.set_cell(&self.new_cell);
        self.gl_area.queue_draw();
    }
    fn undo(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.set_cell(&self.old_cell);
        self.gl_area.queue_draw();
    }
    fn redo(&self) {
        self.execute();
    }
}


pub struct Fill {
    gl_area: GliumArea,
    body_part: BodyPart,
    fill_color: [f32; 4],
    prev_colors: Vec<ModelCell>,
}
impl Fill {
    pub fn new(gl_area: GliumArea, body_part: BodyPart, fill_color: [f32; 4], prev_colors: Vec<ModelCell>) -> Fill {
        Fill {
            gl_area,
            body_part,
            fill_color,
            prev_colors,
        }
    }
}
impl Action for Fill {
    fn execute(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();

        for cell in &self.prev_colors {
            let new_cell = ModelCell {
                body_part: cell.body_part.clone(),
                cell_index: cell.cell_index,
                color: self.fill_color,
            };
            renderer.set_cell(&new_cell);
        }

        self.gl_area.queue_draw();
    }

    fn undo(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();

        for cell in &self.prev_colors {
            let new_cell = ModelCell {
                body_part: cell.body_part.clone(),
                cell_index: cell.cell_index,
                color: cell.color,
            };
            renderer.set_cell(&new_cell);
        }
        self.gl_area.queue_draw();
    }
    fn redo(&self) {
        self.execute();
    }
}


pub struct Replace {
    gl_area: GliumArea,
    old_color: [f32; 4],
    new_color: [f32; 4],
    replaced_cells: RefCell<Vec<ModelCell>>,
}
impl Replace {
    pub fn new(gl_area: GliumArea, old_color: [f32; 4], new_color: [f32; 4]) -> Replace {
        Replace {
            gl_area,
            old_color,
            new_color,
            replaced_cells: RefCell::new(vec![]),
        }
    }
}
impl Action for Replace {
    fn execute(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        let replaced = renderer.replace(self.old_color, self.new_color);
        self.replaced_cells.replace(replaced);
        self.gl_area.queue_draw();
    }

    fn undo(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        for model_cell in self.replaced_cells.borrow().iter() {
            renderer.set_cell(model_cell);
        }
        self.gl_area.queue_draw();
    }

    fn redo(&self) {
        self.execute();
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tool {
    Pencil,
    Rubber,
    ColorPicker,
    Fill,
    Random,
    Replace
}

impl Default for Tool {
    fn default() -> Self {
        Self::Pencil
    }
}

pub struct DrawingHistory {
    gl_area: GliumArea,
    undo_stack: Vec<Box<dyn Action>>,
    redo_stack: Vec<Box<dyn Action>>,
    last_modified_cell: Option<ModelCell>,
}

impl DrawingHistory {
    pub fn new(gl_area: GliumArea) -> DrawingHistory {
        DrawingHistory { gl_area, undo_stack: vec![], redo_stack: vec![], last_modified_cell: None }
    }

    pub fn add_command(&mut self, command: Box<dyn Action>) -> bool {
        command.execute();
        self.undo_stack.push(command);
        self.redo_stack.clear();
        true
    }

    pub fn undo(&mut self) {
        if self.undo_stack.is_empty() { return }
        let command = self.undo_stack.pop()
            .expect("Error popping a command from undo_stack");

        command.undo();
        self.redo_stack.push(command);

        self.last_modified_cell.take();
    }

    pub fn redo(&mut self) {
        if self.redo_stack.is_empty() { return }
        let command = self.redo_stack.pop()
            .expect("Error popping a command from redo_stack.");

        command.redo();
        self.undo_stack.push(command);

        self.last_modified_cell.take();
    }

    pub fn get_last_modified(&self) -> Option<ModelCell> {
        self.last_modified_cell
    }
    pub fn set_last_modified(&mut self, cell: ModelCell) {
        self.last_modified_cell.replace(cell);
    }
    
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}