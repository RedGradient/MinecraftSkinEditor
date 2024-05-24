use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::{BoxImpl, CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl};
use gtk::subclass::widget::WidgetClassExt;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/templates-list.ui")]
pub struct TemplateList {
    #[template_child]
    pub list: TemplateChild<gtk::FlowBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for TemplateList {
    const NAME: &'static str = "TemplateList";
    type Type = super::TemplateList;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TemplateList {}
impl WidgetImpl for TemplateList {}
impl BoxImpl for TemplateList {}
