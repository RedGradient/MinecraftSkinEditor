use crate::command::{Action, Tool};
use crate::glium_area::renderer::ModelCell;

pub trait EditorHost {
    fn tools_enabled(&self) -> bool;
    fn current_tool(&self) -> Tool;
    fn active_color(&self) -> gtk::gdk::RGBA;
    fn set_active_color(&self, rgba: &gtk::gdk::RGBA);
    fn select_pencil_tool(&self);
    fn last_modified_cell(&self) -> Option<ModelCell>;
    fn set_last_modified(&self, cell: ModelCell);
    fn add_command(&self, command: Box<dyn Action>);
}
