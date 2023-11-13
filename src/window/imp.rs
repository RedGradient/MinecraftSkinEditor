use std::cell::{Cell, RefCell};

use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::prelude::GtkWindowExt;
use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetImpl, WindowImpl};
use gtk::subclass::widget::WidgetClassExt;
use libadwaita as adw;
use libadwaita::subclass::application_window::AdwApplicationWindowImpl;
use crate::APP_ID;

use crate::glium_area::GliumArea;
use crate::model_switcher::ModelSwitcher;
use crate::window::Tool;

pub trait Command {
    fn execute(&self, gl_area: &GliumArea);
    fn undo(&self, gl_area: &GliumArea);
}


#[derive(Default)]
pub struct DrawingHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}
impl DrawingHistory {
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, gl_area: &GliumArea) {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(gl_area);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, gl_area: &GliumArea) {
        if let Some(command) = self.redo_stack.pop() {
            command.execute(gl_area);
            self.undo_stack.push(command);
        }
    }
}

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/window.ui")]
// #[template(resource = "/io/redgradient/MCSkinEditor/window.ui")]
pub struct Window {
    #[template_child]
    pub open_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub save_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub undo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub redo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub grid_toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub color_button: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub content_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub left_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub right_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub pencil: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub rubber: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub color_picker: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub gl_area: TemplateChild<GliumArea>,
    #[template_child]
    pub model_switcher: TemplateChild<ModelSwitcher>,

    pub current_tool: Cell<Tool>,
    pub drawing_history: RefCell<DrawingHistory>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MCSkinEditorWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.install_action("win.undo", None, move |win, _, _| {
            let gl_area = win.imp().gl_area.get();
            win.imp().drawing_history.borrow_mut().undo(&gl_area);
        });

        klass.install_action("win.redo", None, move |win, _, _| {
            let gl_area = win.imp().gl_area.get();
            win.imp().drawing_history.borrow_mut().redo(&gl_area);
        });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}