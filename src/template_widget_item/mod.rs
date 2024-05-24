use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp;

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
        self.imp().cover.set_file(Some(cover));
    }
}