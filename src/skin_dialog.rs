use std::io::{Read, Write};
use std::path::PathBuf;

use gtk::glib;
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use image::EncodableLayout;
use libadwaita as adw;
use libadwaita::prelude::AdwDialogExt;

use crate::glium_area::skin_parser::ModelType;
use crate::window::Window;

mod imp {
    use std::cell::Cell;
    use std::path::PathBuf;

    use gtk::{glib, TemplateChild};
    use gtk::CompositeTemplate;
    use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl};
    use gtk::subclass::widget::WidgetClassExt;
    use libadwaita as adw;
    use libadwaita::prelude::AdwDialogExt;
    use libadwaita::subclass::dialog::AdwDialogImpl;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/skin-dialog.ui")]
    pub struct SkinDialog {
        pub texture_path: Cell<Option<PathBuf>>,
        #[template_child]
        pub slim_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub classic_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SkinDialog {
        const NAME: &'static str = "SkinDialog2";
        type Type = super::SkinDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.install_action(
                "skin_dialog.discard",
                None,
                move |dialog, _, _| { dialog.close(); }
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for SkinDialog {}
    impl WidgetImpl for SkinDialog {}
    impl AdwDialogImpl for SkinDialog {}
}

glib::wrapper! {
    pub struct SkinDialog(ObjectSubclass<imp::SkinDialog>)
        @extends gtk::Widget,
        @implements adw::Dialog;
}

impl SkinDialog {
    pub fn new(texture_path: PathBuf, window: Window) -> Self {
        let dialog: SkinDialog = glib::Object::new();
        dialog.imp().texture_path.replace(Some(texture_path));
        dialog.connect_signals(window.clone());
        dialog
    }
    
    fn connect_signals(&self, window: Window) {
        let handler = self.get_handler(window.clone(), ModelType::Slim);
        self.imp().slim_button.connect_clicked(handler);
        
        let handler = self.get_handler(window.clone(), ModelType::Classic);
        self.imp().classic_button.connect_clicked(handler);
    }
    
    fn get_handler(&self, window: Window, model_type: ModelType) -> impl Fn(&gtk::Button) {
        let dialog = self.clone();
        let item_num = match model_type {
            ModelType::Slim => 0,
            ModelType::Classic => 1,
        };
        move |btn| {
            window.imp().opening_new_skin.replace(true);
            window.imp().model_switcher.imp().model_type_selector.set_selected(item_num);

            let renderer = window.imp().gl_area.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();

            let texture_path = dialog.imp().texture_path.take()
                .expect("Texture path is not set. This can happen if the dialog was not created using 'new()' method");
            let texture_path = texture_path.to_str().unwrap();

            let _ = renderer.load_texture(texture_path, &model_type, false);
            window.imp().drawing_history.borrow()
                .as_ref()
                .expect("Drawing history is not initialized")
                .borrow_mut()
                .clear();
            window.imp().gl_area.queue_draw();
            dialog.close();
        }
    }
}