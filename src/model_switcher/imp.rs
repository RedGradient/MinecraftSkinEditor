use gtk::CompositeTemplate;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, WidgetImpl};
use gtk::subclass::prelude::*;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/model-switcher.ui")]
// #[template(resource = "model-switcher.ui")]
pub struct ModelSwitcher {
    #[template_child]
    pub head: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub body: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub right_arm: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub left_arm: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub right_leg: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub left_leg: TemplateChild<gtk::ToggleButton>,

    #[template_child]
    pub inner_layer_toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub outer_layer_toggle: TemplateChild<gtk::ToggleButton>,

    // #[template_child]
    // pub model_button: TemplateChild<ModelButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for ModelSwitcher {
    const NAME: &'static str = "ModelSwitcher";
    type Type = super::ModelSwitcher;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.set_layout_manager_type::<gtk::BinLayout>();
        // klass.set_accessible_role(gtk::AccessibleRole::Group);
    }
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ModelSwitcher {}

impl WidgetImpl for ModelSwitcher {}