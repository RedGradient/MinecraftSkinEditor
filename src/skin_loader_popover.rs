use std::io::{Read, Write};
use std::sync::OnceLock;

use bytes::BufMut;
use gtk::{gio, glib, Orientation};
use gtk::gdk::Texture;
use gtk::glib::clone;
use gtk::prelude::{BoxExt, BufferedInputStreamExt, ButtonExt, EditableExt, WidgetExt};
use gtk::prelude::TextureExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use image::{DynamicImage, EncodableLayout, GenericImage, GenericImageView};
use tokio::runtime::Runtime;

use crate::glium_area::skin_parser::TextureType;
use crate::utils::guess_model_type;
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
    
    pub async fn get_skin(&self, nickname: &str) -> Result<DynamicImage, ()> {
        let uri = format!("{}/{}", Self::URI, nickname);
        let url = reqwest::Url::parse(uri.as_str()).unwrap();
        let mut skin = reqwest::get(url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        Ok(image::load_from_memory(skin.as_bytes()).unwrap())
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
                let client = SkinApiClient::new();
                let nickname = popover.imp().search_skin_entry.text();
                let texture = client.get_skin(nickname.as_str()).await.unwrap();
                let texture_button = SkinLoaderPopover::create_texture_button(&texture, nickname.as_str());
                let win = win.clone();
                texture_button.connect_clicked(move |_| {
                    let renderer = win.imp().gl_area.renderer();
                    let mut renderer = renderer.as_ref().unwrap().borrow_mut();
                    let model_type = guess_model_type(texture.as_bytes());
                    let texture_type = match texture.dimensions() {
                        (64, 64) => TextureType::Normal,
                        (64, 32) => TextureType::Legacy,
                        _ => panic!("Wrong texture dimensions")
                    };
                    let load_result = renderer.load_texture_from_bytes(
                        &texture, model_type.unwrap(), texture_type, false);
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
                popover.imp().popover_content.append(&texture_button);
            }));
        });
    }

    fn create_texture_button(img: &DynamicImage, title: &str) -> gtk::Button {
        let button = gtk::Button::new();
        let temp_file = "temp_image.png";
        img.save(temp_file).unwrap();
        let f = gio::File::for_path(temp_file);
        // let glib_bytes = glib::Bytes::from(img.as_bytes());
        // let paintable = Texture::from_bytes(&glib_bytes).unwrap();
        let paintable = Texture::from_file(&f).unwrap();
        let image = gtk::Image::builder()
            .paintable(&paintable)
            .height_request(50)
            .width_request(50)
            .build();
        let label = gtk::Label::new(Some(title));
        let inner_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .build();
        inner_box.append(&image);
        inner_box.append(&label);
        button.set_child(Some(&inner_box));

        button
    }
}


#[test]
fn test() {
    let bytes = include_bytes!("/Users/redgradient/RustroverProjects/MinecraftSkinEditor/resources/steve-front.png");

    // let glib_bytes = glib::Bytes::from(bytes);
    // let paintable = Texture::from_bytes(&glib_bytes).unwrap();

    let img = image::load_from_memory(bytes).unwrap();
    img.save("temp_image.png").unwrap();
    let f = gio::File::for_path("temp_image.png");
    Texture::from_file(&f).expect("");
    // let paintable = Texture::from_bytes(&glib_bytes).unwrap();
}

#[test]
fn test2() {
    let bytes = include_bytes!("/Users/redgradient/RustroverProjects/MinecraftSkinEditor/resources/steve-front.png");
    // let mut di = image::ImageBuffer::new(64, 64);
    // let x: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(64, 64, bytes.to_vec()).unwrap();

    // assert_eq!(bytes, x.as_bytes());

    const URI: &'static str = "https://mc-heads.net/skin/Lysias";
    let url = reqwest::Url::parse(URI).unwrap();
    let mut skin = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    
    let image = image::io::Reader::new(std::io::Cursor::new(skin.as_bytes()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    
    // assert_eq!(bytes, image.as_bytes());
    // assert_eq!(bytes, di.as_rgba8().unwrap().as_bytes());
    // assert_eq!(bytes, di.into_bytes().as_slice());
    // assert_eq!(bytes, di.as_bytes());

    assert_eq!(skin.as_bytes(), image.as_bytes());
}