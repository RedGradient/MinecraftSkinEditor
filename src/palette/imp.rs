use gtk::CompositeTemplate;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, WidgetImpl};
use gtk::subclass::prelude::*;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/palette.ui")]
pub struct Palette {
    #[template_child]
    main_box: TemplateChild<gtk::Box>,
    #[template_child]
    row_box_1: TemplateChild<gtk::Box>,
    #[template_child]
    color_1: TemplateChild<gtk::ToggleButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for Palette {
    const NAME: &'static str = "Palette";
    type Type = super::Palette;
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

impl ObjectImpl for Palette {}

impl WidgetImpl for Palette {}