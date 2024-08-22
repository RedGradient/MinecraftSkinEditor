use std::error::Error;
use std::io::{Read, Write};
use std::ops::{Add, Deref, DerefMut};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use bytes::BufMut;
use gtk::{gio, glib, Orientation};
use gtk::gdk::Texture;
use gtk::glib::clone;
use gtk::prelude::{BoxExt, BufferedInputStreamExt, ButtonExt, EditableExt, WidgetExt};
use gtk::prelude::TextureExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use image::{DynamicImage, EncodableLayout, GenericImage, GenericImageView};
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, oneshot};

use crate::glium_area::skin_parser::TextureType;
use crate::utils::guess_model_type;
use crate::window::Window;

mod imp {
    use std::cell::Cell;
    use std::sync::Arc;

    use gtk::{CompositeTemplate, glib, TemplateChild};
    use gtk::subclass::popover::PopoverImpl;
    use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, WidgetImpl};
    use gtk::subclass::widget::WidgetClassExt;

    use crate::skin_loader_popover::SkinClient;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/skin-loader-popover.ui")]
    pub struct SkinLoaderPopover {
        #[template_child]
        pub nickname_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub search_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub popover_content: TemplateChild<gtk::Box>,

        pub client: Arc<SkinClient>,
        pub is_searching: Cell<bool>,
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
    RUNTIME.get_or_init(|| Runtime::new()
        .expect("Setting up tokio runtime needs to succeed."))
}

#[derive(Default)]
struct SkinClient {
    last_request_time: Arc<Mutex<Option<Instant>>>,
    cooldown_duration: Arc<Mutex<Duration>>,
}
impl SkinClient {
    const BASE_URL: &'static str = "https://mc-heads.net/skin";

    pub async fn set_cooldown(&self, secs: f32) {
        *self.cooldown_duration.lock().await = Duration::from_secs_f32(secs);
    }

    pub async fn cooldown(&self) {
        let mut last_request_time = self.last_request_time.lock().await;
        let cooldown_duration = self.cooldown_duration.lock().await.clone();

        if let Some(last_time) = *last_request_time {
            let elapsed = last_time.elapsed();
            if elapsed < cooldown_duration {
                let remained_cooldown = cooldown_duration - elapsed;
                tokio::time::sleep(remained_cooldown).await;
            }
        }

        // Update last request time
        *last_request_time = Some(Instant::now());
    }

    pub async fn get_skin(&self, nickname: &str) -> Result<DynamicImage, Box<dyn Error>> {
        self.cooldown().await;
        let uri = format!("{}/{}", Self::BASE_URL, nickname);
        let url = reqwest::Url::parse(uri.as_str()).unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;
        let mut skin = client.get(url).send().await?.bytes().await?;
        let image = image::load_from_memory(skin.as_bytes())?;
        Ok(image)
    }
}

impl SkinLoaderPopover {
    pub fn new(win: &Window) -> Self {
        let popover: SkinLoaderPopover = glib::Object::new();
        runtime().block_on(async {
            popover.imp().client.set_cooldown(1.5).await
        });
        popover.connect_signals(win);
        popover
    }

    pub fn connect_signals(&self, win: &Window) {
        self.imp().search_button.connect_clicked(self.get_search_skin_button_handler(win.clone()));
    }

    fn create_texture_button(win: Window, texture: DynamicImage, title: &str) -> gtk::Button {
        let texture_button = gtk::Button::new();
        let paintable = {
            let temporary_file = "temporary_file.png";
            texture.save(temporary_file).unwrap();
            let f = gio::File::for_path(temporary_file);
            Texture::from_file(&f).unwrap()
        };
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
        texture_button.set_child(Some(&inner_box));

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

        texture_button
    }

    fn get_search_skin_button_handler(&self, win: Window) -> impl Fn(&gtk::Button) {
        let popover = self.clone();
        move |btn| {
            if popover.searching() {
                return
            }
            let nickname = match popover.get_nickname() {
                Some(nickname) => nickname,
                None => return
            };
            popover.set_searching(true);

            popover.clear();
            popover.add_spinner();

            let (tx, mut rx) = oneshot::channel::<Result<DynamicImage, ()>>();

            // Spawn a task to fetch the skin
            let client = popover.imp().client.clone();
            runtime().spawn(clone!(@strong nickname => async move {
                println!("Fetching the skin...");
                let response = client.get_skin(nickname.as_str()).await.map_err(|_| ());
                tx.send(response).expect("The receiver needs to be open");
            }));

            glib::spawn_future_local(clone!(@strong win, @strong popover => async move {
                if let Ok(response) = rx.await {
                    popover.set_searching(false);
                    if response.is_err() {
                        println!("Bad request");
                        popover.clear();
                        return
                    }
                    let texture = response.unwrap();
                    let texture_button = SkinLoaderPopover::create_texture_button(win.clone(), texture, nickname.as_str());
                    popover.clear();
                    popover.imp().popover_content.append(&texture_button);
                }
            }));
        }
    }

    fn get_nickname(&self) -> Option<String> {
        let text = self.imp().nickname_entry.text();
        if text.is_empty() {
            return None
        }
        Some(text.to_string())
    }

    fn searching(&self) -> bool {
        self.imp().is_searching.get()
    }

    fn set_searching(&self, searching: bool) {
        self.imp().is_searching.replace(searching);
    }

    fn clear(&self) {
        while let Some(spinner) = self.imp().popover_content.last_child() {
            self.imp().popover_content.remove(&spinner);
        }
    }

    fn add_spinner(&self) {
        let spinner = gtk::Spinner::new();
        spinner.set_height_request(40);
        spinner.set_width_request(40);
        spinner.set_spinning(true);
        self.imp().popover_content.append(&spinner);
    }
}