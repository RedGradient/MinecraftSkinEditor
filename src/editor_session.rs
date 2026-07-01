use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::WidgetExt;
use image::{DynamicImage, ImageBuffer, Rgba};

use crate::command::{Action, DrawingHistory, Tool};
use crate::glium_area::body_part::BodyPart;
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::{ModelCell, Renderer};
use crate::glium_area::skin_parser::{ModelType, TextureLoadError, TextureType};

pub struct EditorSession {
    viewport: GliumArea,
    history: DrawingHistory,
    tool: Tool,
    tools_enabled: bool,
    dirty: bool,
}

impl EditorSession {
    pub fn new(viewport: GliumArea) -> Self {
        let history = DrawingHistory::new(viewport.clone());
        Self {
            viewport,
            history,
            tool: Tool::default(),
            tools_enabled: true,
            dirty: false,
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
        self.mark_dirty();
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn last_modified_cell(&self) -> Option<ModelCell> {
        self.history.get_last_modified()
    }

    pub fn set_last_modified(&mut self, cell: ModelCell) {
        self.history.set_last_modified(cell);
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        self.clear_dirty();
    }

    pub fn request_redraw(&self) {
        self.viewport.queue_draw();
    }

    pub fn renderer(&self) -> Option<Rc<RefCell<Renderer>>> {
        self.viewport.renderer()
    }

    pub fn load_skin_from_path(
        &mut self,
        path: &str,
        model_type: &ModelType,
        ignore_transparent: bool,
    ) -> Result<(), TextureLoadError> {
        let renderer = self.renderer().expect("Renderer is not initialized");
        let mut renderer = renderer.borrow_mut();
        renderer.load_texture(path, model_type, ignore_transparent)?;
        self.mark_dirty();
        Ok(())
    }

    pub fn load_skin_from_image(
        &mut self,
        image: &DynamicImage,
        model_type: ModelType,
        texture_type: TextureType,
        ignore_transparent: bool,
    ) -> Result<(), TextureLoadError> {
        let renderer = self.renderer().expect("Renderer is not initialized");
        let mut renderer = renderer.borrow_mut();
        renderer.load_texture_from_bytes(
            image,
            model_type,
            texture_type,
            ignore_transparent,
        )?;
        self.mark_dirty();
        Ok(())
    }

    pub fn load_template(&mut self, path: &str) -> Result<(), TextureLoadError> {
        let renderer = self.renderer().expect("Renderer is not initialized");
        let model_type = renderer.borrow().get_model_type();
        let mut renderer = renderer.borrow_mut();
        renderer.load_texture(path, &model_type, true)?;
        self.mark_dirty();
        Ok(())
    }

    pub fn reset_skin(&mut self) {
        let renderer = self.renderer().expect("Renderer is not initialized");
        renderer.borrow_mut().reset_skin();
        self.mark_dirty();
    }

    pub fn set_grid_visible(&mut self, visible: bool) {
        let renderer = self.renderer().expect("Renderer is not initialized");
        renderer.borrow_mut().set_grid_show(visible);
    }

    pub fn set_body_part_active(&mut self, body_part: &BodyPart, visible: bool) {
        let renderer = self.renderer().expect("Renderer is not initialized");
        renderer.borrow_mut().set_body_part_active(body_part, visible);
    }

    pub fn set_body_parts_active(&mut self, updates: &[(&BodyPart, bool)]) {
        let renderer = self.renderer().expect("Renderer is not initialized");
        let mut renderer = renderer.borrow_mut();
        for (body_part, visible) in updates {
            renderer.set_body_part_active(body_part, *visible);
        }
    }

    pub fn reset_model_type(&mut self, model_type: &ModelType) {
        let renderer = self.renderer().expect("Renderer is not initialized");
        renderer.borrow_mut().reset_model_type(model_type);
        self.mark_dirty();
    }

    pub fn export_texture(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let renderer = self.renderer().expect("Renderer is not initialized");
        let exported = renderer.borrow().export_texture();
        exported
    }
}
