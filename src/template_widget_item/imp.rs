use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::{ButtonImpl, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass};
use gtk::subclass::widget::CompositeTemplate;
use gtk::subclass::widget::WidgetClassExt;
use gtk::subclass::widget::WidgetImpl;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../resources/ui/template-widget-item.ui")]
pub struct TemplateWidgetItem {
    #[template_child]
    pub widget_item_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub title: TemplateChild<gtk::Label>,
    #[template_child]
    pub cover: TemplateChild<gtk::Image>
}

#[glib::object_subclass]
impl ObjectSubclass for TemplateWidgetItem {
    const NAME: &'static str = "TemplateWidgetItem";
    type Type = super::TemplateWidgetItem;
    type ParentType = gtk::Button;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TemplateWidgetItem {}
impl WidgetImpl for TemplateWidgetItem {}
impl ButtonImpl for TemplateWidgetItem {}