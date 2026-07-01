use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::WidgetExt;

use crate::command::{Action, DrawingHistory, Tool};
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::{ModelCell, Renderer};

pub struct EditorSession {
    viewport: GliumArea,
    history: DrawingHistory,
    tool: Tool,
    tools_enabled: bool,
}

impl EditorSession {
    pub fn new(viewport: GliumArea) -> Self {
        let history = DrawingHistory::new(viewport.clone());
        Self {
            viewport,
            history,
            tool: Tool::default(),
            tools_enabled: true,
        }
    }

    pub fn viewport(&self) -> &GliumArea {
        &self.viewport
    }

    pub fn tool(&self) -> Tool {
        self.tool
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.tool = tool;
    }

    pub fn tools_enabled(&self) -> bool {
        self.tools_enabled
    }

    pub fn set_tools_enabled(&mut self, enabled: bool) {
        self.tools_enabled = enabled;
    }

    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    pub fn add_command(&mut self, command: Box<dyn Action>) {
        self.history.add_command(command);
    }

    pub fn last_modified_cell(&self) -> Option<ModelCell> {
        self.history.get_last_modified()
    }

    pub fn set_last_modified(&mut self, cell: ModelCell) {
        self.history.set_last_modified(cell);
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn request_redraw(&self) {
        self.viewport.queue_draw();
    }

    pub fn renderer(&self) -> Option<Rc<RefCell<Renderer>>> {
        self.viewport.renderer()
    }
}
