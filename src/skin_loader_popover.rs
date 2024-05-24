use std::io::{Read, Write};
use std::sync::OnceLock;

use gtk::{gio, glib, Orientation};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use image::EncodableLayout;
use tokio::runtime::Runtime;

use crate::utils::get_model_type;
use crate::window::Window;

mod imp {
    use gtk::{glib, TemplateChild};
    use gtk::CompositeTemplate;
    use gtk::subclass::popover::PopoverImpl;
    use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl};
    use gtk::subclass::widget::WidgetClassExt;

    use crate::skin_loader_popover::SkinApiClient;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/skin-loader-popover.ui")]
    pub struct SkinLoaderPopover {
        #[template_child]
        pub search_skin_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub search_skin_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub popover_content: TemplateChild<gtk::Box>,

        pub skin_loader_api_client: SkinApiClient,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SkinLoaderPopover {
        const NAME: &'static str = "SkinLoaderPopover";
        type Type = super::SkinLoaderPopover;
        type ParentType = gtk::Popover;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for SkinLoaderPopover {}
    impl WidgetImpl for SkinLoaderPopover {}
    impl PopoverImpl for SkinLoaderPopover {}
}

glib::wrapper! {
    pub struct SkinLoaderPopover(ObjectSubclass<imp::SkinLoaderPopover>)
        @extends gtk::Widget,
        @implements gtk::Popover;
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Setting up tokio runtime needs to succeed."))
}

#[derive(Default)]
struct SkinApiClient;
impl SkinApiClient {
    const URI: &'static str = "https://mc-heads.net/skin";

    pub fn new() -> SkinApiClient {
        SkinApiClient
    }
    
    pub async fn get_skin(&self, nickname: &str) -> Result<bytes::Bytes, reqwest::Error> {
        let uri = format!("{}/{}", Self::URI, nickname);
        let url = reqwest::Url::parse(uri.as_str()).unwrap();
        let skin = reqwest::get(url).await?.bytes().await?;
        
        // let skin = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        Ok(skin)
    }
}

impl SkinLoaderPopover {
    pub fn new(win: &Window) -> Self {
        let popover: SkinLoaderPopover = glib::Object::new();

        popover.connect_signals(win);

        popover
    }

    pub fn connect_signals(&self, win: &Window) {
        let popover = self.clone();
        let win = win.clone();
        self.imp().search_skin_button.connect_clicked(move |_| {
            let popover = popover.clone();
            let win = win.clone();
            runtime().block_on(clone!(@weak popover as p => async move {
                let nickname = popover.imp().search_skin_entry.text();
                let client = SkinApiClient::new();

                // --- get skin as bytes ---
                let bytes = client.get_skin(nickname.as_str()).await.unwrap();
                
                // let front_texture = ...(bytes);
                
                // --- make GtkImage from bytes ---
                let glib_bytes: glib::Bytes = glib::Bytes::from(bytes.as_bytes());
                let input_stream = MemoryInputStream::from_bytes(&glib_bytes);
                let pixbuf = Pixbuf::from_stream(&input_stream, None::<&gio::Cancellable>).unwrap();
                let image = gtk::Image::from_pixbuf(Some(&pixbuf));
                image.set_height_request(50);
                image.set_width_request(50);

                // --- construct list item ---
                let list_item = gtk::Box::builder().spacing(10).orientation(Orientation::Horizontal).build();
                let label = gtk::Label::new(Some(nickname.as_str()));
                list_item.append(&image);
                list_item.append(&label);
                let button = gtk::Button::new();
                button.set_child(Some(&list_item));
                
                let win = win.clone();
                button.connect_clicked(move |_| {
                    let renderer = win.imp().gl_area.renderer();
                    let mut renderer = renderer.as_ref().unwrap().borrow_mut();
                    let model_type = get_model_type(bytes.as_bytes());
                    if model_type.is_err() {
                        println!("model_type is err");
                        return
                    }
                    let load_result = renderer.load_texture_from_bytes(&bytes, model_type.unwrap(), false);
                    if load_result.is_err() {
                        println!("Error loading texture: {:?}", load_result.unwrap_err());
                        return
                    }
                    win.imp().gl_area.queue_draw();
                    println!("Texture loaded");
                });
                
                if let Some(child) = popover.imp().popover_content.last_child() {
                    popover.imp().popover_content.remove(&child); 
                }
                popover.imp().popover_content.append(&button);
            }));
        });
    }
}