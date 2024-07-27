use gtk::glib;
use gtk::prelude::MediaFileExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp {
    use gtk::{CompositeTemplate, glib, TemplateChild};
    use gtk::glib::subclass::InitializingObject;
    use gtk::subclass::prelude::{ButtonImpl, CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl};
    use gtk::subclass::widget::WidgetClassExt;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/template-widget-item.ui")]
    pub struct TemplateWidgetItem {
        #[template_child]
        pub widget_item_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub cover: TemplateChild<gtk::Image>,
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

}

glib::wrapper! {
    pub struct TemplateWidgetItem(ObjectSubclass<imp::TemplateWidgetItem>)
        @extends gtk::Widget,
        @implements gtk::Button;
}

impl TemplateWidgetItem {
    pub fn new(cover: &str, title: &str) -> Self {
        let template_widget_item = glib::Object::builder::<TemplateWidgetItem>().build();
        template_widget_item.set_cover(cover);
        template_widget_item.set_title(title);
        template_widget_item
    }

    pub fn set_title(&self, title: &str) {
        self.imp().title.set_label(title);
    }

    pub fn set_cover(&self, cover: &str) {
        self.imp().cover.set_from_file(Some(cover));
    }
}