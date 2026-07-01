use std::cell::{Ref, RefCell, RefMut};

use gtk::gio;
use gtk::glib;
use gtk::prelude::{ButtonExt, ColorChooserExt, ToggleButtonExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita as adw;

use crate::application::Application;
use crate::command::{Action, Tool};
use crate::editor_host::EditorHost;
use crate::editor_session::EditorSession;
use crate::glium_area::body_part::BodyPart;
use crate::glium_area::renderer::ModelCell;
use crate::glium_area::skin_parser::{ModelType, TextureLoadError, TextureType};
use crate::skin_loader_popover::SkinLoaderPopover;

mod imp;
mod signals;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let win = glib::Object::builder::<Window>()
            .property("application", app)
            .build();

        win.setup();
        win.set_icons();
        signals::connect(&win);

        win
    }

    fn setup(&self) {
        self.add_css_class("devel");
        self.imp().header_bar.set_show_title(false);

        let popover = SkinLoaderPopover::new(self);
        self.imp().open_button.set_popover(Some(&popover));

        let color_dialog = gtk::ColorDialog::builder().with_alpha(false).build();
        self.imp().color_button.set_dialog(&color_dialog);

        let gl_area = self.imp().gl_area.get();
        gl_area.setup(self.clone());
        self.imp().editor.replace(Some(EditorSession::new(gl_area)));
        self.set_tool_active(true);
    }

    fn set_icons(&self) {
        let pencil_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/pencil.png");
        let rubber_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/eraser.png");
        let color_picker_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/color_picker.png");
        let grid_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/grid.png");
        let fill_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/fill.png");
        let replace_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/replace.png");
        self.imp().pencil.set_child(Some(&pencil_ico));
        self.imp().rubber.set_child(Some(&rubber_ico));
        self.imp().color_picker.set_child(Some(&color_picker_ico));
        self.imp().grid_toggle.set_child(Some(&grid_ico));
        self.imp().fill.set_child(Some(&fill_ico));
        self.imp().replace_color.set_child(Some(&replace_ico));
    }

    pub fn editor(&self) -> Ref<'_, EditorSession> {
        Ref::map(self.imp().editor.borrow(), |editor| {
            editor.as_ref().expect("EditorSession is not initialized")
        })
    }

    pub fn editor_mut(&self) -> RefMut<'_, EditorSession> {
        RefMut::map(self.imp().editor.borrow_mut(), |editor| {
            editor.as_mut().expect("EditorSession is not initialized")
        })
    }

    pub fn gl_area(&self) -> crate::glium_area::GliumArea {
        self.editor().viewport().clone()
    }

    pub fn begin_skin_import(&self, model_type_index: u32) {
        self.imp().opening_new_skin.replace(true);
        self.imp()
            .model_switcher
            .imp()
            .model_type_selector
            .set_selected(model_type_index);
    }

    pub fn clear_drawing_history(&self) {
        self.editor_mut().clear_history();
    }

    pub fn request_viewport_redraw(&self) {
        self.editor().request_redraw();
    }

    pub fn open_skin_file(&self, path: &str, model_type: ModelType) -> Result<(), TextureLoadError> {
        let item_num = match model_type {
            ModelType::Slim => 0,
            ModelType::Classic => 1,
        };
        self.begin_skin_import(item_num);
        self.editor_mut()
            .load_skin_from_path(path, &model_type, false)?;
        self.clear_drawing_history();
        self.request_viewport_redraw();
        Ok(())
    }

    pub fn load_skin_from_image(
        &self,
        image: &image::DynamicImage,
        model_type: ModelType,
        texture_type: TextureType,
    ) -> Result<(), TextureLoadError> {
        self.editor_mut()
            .load_skin_from_image(image, model_type, texture_type, false)?;
        self.request_viewport_redraw();
        Ok(())
    }

    pub fn load_template(&self, path: &str) -> Result<(), TextureLoadError> {
        self.editor_mut().load_template(path)?;
        self.request_viewport_redraw();
        Ok(())
    }

    pub fn reset_skin(&self) {
        self.editor_mut().reset_skin();
        self.set_grid_visible(true);
    }

    pub fn set_grid_visible(&self, active: bool) {
        self.editor_mut().set_grid_visible(active);
        self.imp().grid_toggle.set_active(active);
        self.request_viewport_redraw();
    }

    pub fn export_texture(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        self.editor().export_texture()
    }

    pub fn refresh_template_list(&self) {
        self.imp().template_list.load_list(self);
    }

    pub fn set_body_part_visible(&self, body_part: &BodyPart, visible: bool) {
        self.editor_mut().set_body_part_active(body_part, visible);
        self.request_viewport_redraw();
    }

    pub fn set_body_parts_visible(&self, updates: &[(&BodyPart, bool)]) {
        self.editor_mut().set_body_parts_active(updates);
        self.request_viewport_redraw();
    }

    pub fn change_model_type(&self, model_type: ModelType) {
        self.editor_mut().reset_model_type(&model_type);
        self.request_viewport_redraw();
    }

    pub(super) fn consume_skin_import_model_change(&self) -> bool {
        self.imp().opening_new_skin.take()
    }

    pub fn is_dirty(&self) -> bool {
        self.editor().is_dirty()
    }

    pub fn save_skin_to_path(&self, path: &str) -> Result<(), image::ImageError> {
        self.export_texture().save(path)?;
        self.editor_mut().clear_dirty();
        Ok(())
    }

    pub(super) fn set_tool_active(&self, active: bool) {
        self.editor_mut().set_tools_enabled(active);
    }

    pub fn is_tool_active(&self) -> bool {
        self.editor().tools_enabled()
    }
}

impl EditorHost for Window {
    fn tools_enabled(&self) -> bool {
        self.editor().tools_enabled()
    }

    fn current_tool(&self) -> Tool {
        self.editor().tool()
    }

    fn active_color(&self) -> gtk::gdk::RGBA {
        self.imp().color_button.rgba()
    }

    fn set_active_color(&self, rgba: &gtk::gdk::RGBA) {
        self.imp().color_button.set_rgba(rgba);
    }

    fn select_pencil_tool(&self) {
        self.imp().pencil.set_active(true);
        self.editor_mut().set_tool(Tool::Pencil);
    }

    fn last_modified_cell(&self) -> Option<ModelCell> {
        self.editor().last_modified_cell()
    }

    fn set_last_modified(&self, cell: ModelCell) {
        self.editor_mut().set_last_modified(cell);
    }

    fn add_command(&self, command: Box<dyn Action>) {
        self.editor_mut().add_command(command);
    }
}
