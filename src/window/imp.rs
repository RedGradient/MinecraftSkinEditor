use std::cell::{Cell, RefCell};
use std::hash::Hash;

use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::prelude::GtkWindowExt;
use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetImpl, WindowImpl};
use gtk::subclass::widget::WidgetClassExt;
use libadwaita as adw;
use libadwaita::subclass::application_window::AdwApplicationWindowImpl;

use crate::APP_ID;
use crate::glium_area::body_part::BodyPart;
use crate::glium_area::GliumArea;
use crate::glium_area::renderer::ModelCell;
use crate::model_switcher::ModelSwitcher;
use crate::window::drawing_history::DrawingHistory;
use crate::window::Tool;

#[derive(Debug)]
pub enum Command {
    Pencil { prev: ModelCell, new: ModelCell },
    Fill { body_part: BodyPart, fill_color: [f32; 4], prev_colors: Vec<ModelCell> },
}
impl Command {
    pub fn pencil(target_cell: ModelCell, new_color: [f32; 4]) -> Command {
        let new = ModelCell {
            body_part: target_cell.body_part.clone(),
            cell_index: target_cell.cell_index,
            color: new_color
        };

        Command::Pencil {
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

    pub fn execute(&self) {

    }
    pub fn undo(&self) {

    }
}

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/window.ui")]
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
    pub fill: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub gl_area: TemplateChild<GliumArea>,
    #[template_child]
    pub model_switcher: TemplateChild<ModelSwitcher>,

    pub current_tool: Cell<Tool>,
    pub drawing_history: RefCell<Option<RefCell<DrawingHistory>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MCSkinEditorWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.install_action("win.undo", None, move |win, _, _| {
            // let gl_area = win.imp().gl_area.get();
            win.imp().drawing_history.borrow().as_ref().unwrap().borrow_mut().undo();
        });

        klass.install_action("win.redo", None, move |win, _, _| {
            // let gl_area = win.imp().gl_area.get();
            win.imp().drawing_history.borrow().as_ref().unwrap().borrow_mut().redo();
        });

        klass.install_action("win.about", None, move |win, _, _| {
            win.imp().show_about();
        });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {

}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}

impl Window {
    fn show_about(&self) {
        let about_window = adw::AboutWindow::builder()
            .application_name("Minecraft Skin Editor")
            .application_icon(APP_ID)
            .version("0.1.0")
            .website("https://github.com/RedGradient/MinecraftSkinEditor")
            .issue_url("https://github.com/RedGradient/MinecraftSkinEditor/issues")
            .copyright("© 2023 RedGradient")
            .developers(vec!["RedGradient"])
            .license_type(gtk::License::Gpl30)
            .build();

        about_window.present();
    }
}