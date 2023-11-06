use std::cell::Cell;

use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl, WindowImpl};
use gtk::subclass::widget::WidgetClassExt;
use libadwaita as adw;
use libadwaita::subclass::application_window::AdwApplicationWindowImpl;

use crate::glium_area::GliumArea;
use crate::model_switcher::ModelSwitcher;
use crate::window::Tool;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/window.ui")]
pub struct Window {
    // #[template_child]
    // pub header: gtk::HeaderBar,
    #[template_child]
    pub open_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub save_button: TemplateChild<gtk::Button>,
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
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MCSkinEditorWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
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