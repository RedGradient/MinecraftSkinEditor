mod imp {
    use gtk::CompositeTemplate;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, WidgetImpl};
    use gtk::subclass::prelude::*;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/model-switcher.ui")]
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
        pub model_type_selector: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub inner_layer_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub outer_layer_toggle: TemplateChild<gtk::ToggleButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModelSwitcher {
        const NAME: &'static str = "ModelSwitcher";
        type Type = super::ModelSwitcher;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ModelSwitcher {}

    impl WidgetImpl for ModelSwitcher {}
}

use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::ToggleButtonExt;
use gtk::subclass::prelude::*;


glib::wrapper! {
    pub struct ModelSwitcher(ObjectSubclass<imp::ModelSwitcher>)
        @extends gtk::Widget;
}

impl ModelSwitcher {
    pub fn new() -> ModelSwitcher {
        Object::builder().build()
    }

    pub fn head(&self) -> gtk::ToggleButton {
        self.imp().head.get()
    }

    pub fn torso(&self) -> gtk::ToggleButton {
        self.imp().body.get()
    }

    pub fn right_arm(&self) -> gtk::ToggleButton {
        self.imp().right_arm.get()
    }

    pub fn left_arm(&self) -> gtk::ToggleButton {
        self.imp().left_arm.get()
    }

    pub fn right_leg(&self) -> gtk::ToggleButton {
        self.imp().right_leg.get()
    }

    pub fn left_leg(&self) -> gtk::ToggleButton {
        self.imp().left_leg.get()
    }

    pub fn inner_layer_toggle(&self) -> gtk::ToggleButton {
        self.imp().inner_layer_toggle.get()
    }

    pub fn outer_layer_toggle(&self) -> gtk::ToggleButton {
        self.imp().outer_layer_toggle.get()
    }
}