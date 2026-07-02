use std::collections::BTreeMap;

use gtk::prelude::WidgetExt;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::ModelCell;

pub trait Action {
    fn execute(&self);
}

#[derive(Clone)]
struct CellChange {
    body_part: BodyPart,
    cell_index: usize,
    before: [f32; 4],
    after: [f32; 4],
}

type ModelSnapshot = BTreeMap<(BodyPart, usize), [f32; 4]>;

fn snapshot_model(gl_area: &GliumArea) -> ModelSnapshot {
    let renderer = gl_area.renderer().expect("Renderer is not initialized");
    let snapshot = renderer.borrow().snapshot_cells();
    snapshot
}

fn colors_differ(a: [f32; 4], b: [f32; 4]) -> bool {
    a != b
}

fn diff_snapshots(before: &ModelSnapshot, after: &ModelSnapshot) -> Vec<CellChange> {
    let mut changes = Vec::new();

    for ((body_part, cell_index), &after_color) in after {
        let before_color = before
            .get(&(*body_part, *cell_index))
            .copied()
            .unwrap_or([0.0, 0.0, 0.0, 0.0]);
        if colors_differ(before_color, after_color) {
            changes.push(CellChange {
                body_part: *body_part,
                cell_index: *cell_index,
                before: before_color,
                after: after_color,
            });
        }
    }

    for ((body_part, cell_index), &before_color) in before {
        if !after.contains_key(&(*body_part, *cell_index)) {
            changes.push(CellChange {
                body_part: *body_part,
                cell_index: *cell_index,
                before: before_color,
                after: [0.0, 0.0, 0.0, 0.0],
            });
        }
    }

    changes
}

fn apply_diff(gl_area: &GliumArea, diff: &[CellChange], undo: bool) {
    let renderer = gl_area.renderer().expect("Renderer is not initialized");
    let mut renderer = renderer.borrow_mut();

    for change in diff {
        let color = if undo { change.before } else { change.after };
        renderer.set_cell(&ModelCell {
            body_part: change.body_part,
            cell_index: change.cell_index,
            color,
        });
    }

    gl_area.queue_draw();
}

pub struct Draw {
    gl_area: GliumArea,
    new_cell: ModelCell,
}
impl Draw {
    pub fn new(gl_area: GliumArea, cell: ModelCell, color: [f32; 4]) -> Draw {
        Draw {
            gl_area,
            new_cell: ModelCell {
                body_part: cell.body_part,
                cell_index: cell.cell_index,
                color,
            },
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
}

pub struct Fill {
    gl_area: GliumArea,
    fill_color: [f32; 4],
    cells: Vec<ModelCell>,
}
impl Fill {
    pub fn new(gl_area: GliumArea, _body_part: BodyPart, fill_color: [f32; 4], cells: Vec<ModelCell>) -> Fill {
        Fill {
            gl_area,
            fill_color,
            cells,
        }
    }
}
impl Action for Fill {
    fn execute(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();

        for cell in &self.cells {
            let new_cell = ModelCell {
                body_part: cell.body_part,
                cell_index: cell.cell_index,
                color: self.fill_color,
            };
            renderer.set_cell(&new_cell);
        }

        self.gl_area.queue_draw();
    }
}

pub struct Replace {
    gl_area: GliumArea,
    old_color: [f32; 4],
    new_color: [f32; 4],
}
impl Replace {
    pub fn new(gl_area: GliumArea, old_color: [f32; 4], new_color: [f32; 4]) -> Replace {
        Replace {
            gl_area,
            old_color,
            new_color,
        }
    }
}
impl Action for Replace {
    fn execute(&self) {
        let renderer = self.gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.replace(self.old_color, self.new_color);
        self.gl_area.queue_draw();
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
    undo_stack: Vec<Vec<CellChange>>,
    redo_stack: Vec<Vec<CellChange>>,
    last_modified_cell: Option<ModelCell>,
}

impl DrawingHistory {
    pub fn new(gl_area: GliumArea) -> DrawingHistory {
        DrawingHistory { gl_area, undo_stack: vec![], redo_stack: vec![], last_modified_cell: None }
    }

    pub fn add_command(&mut self, command: Box<dyn Action>) -> bool {
        let before = snapshot_model(&self.gl_area);
        command.execute();
        let after = snapshot_model(&self.gl_area);
        let diff = diff_snapshots(&before, &after);

        if !diff.is_empty() {
            self.undo_stack.push(diff);
            self.redo_stack.clear();
        }

        true
    }

    pub fn undo(&mut self) {
        if self.undo_stack.is_empty() { return }
        let diff = self.undo_stack.pop()
            .expect("Error popping a diff from undo_stack");

        apply_diff(&self.gl_area, &diff, true);
        self.redo_stack.push(diff);

        self.last_modified_cell.take();
    }

    pub fn redo(&mut self) {
        if self.redo_stack.is_empty() { return }
        let diff = self.redo_stack.pop()
            .expect("Error popping a diff from redo_stack.");

        apply_diff(&self.gl_area, &diff, false);
        self.undo_stack.push(diff);

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
