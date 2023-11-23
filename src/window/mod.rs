use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gtk::{EventControllerMotion, EventControllerScroll, EventControllerScrollFlags, FileChooserAction, FileChooserNative, GestureClick, gio};
use gtk::gdk::RGBA;
use gtk::glib;
use gtk::glib::{clone, IsA, ObjectExt, Propagation};
use gtk::prelude::{ButtonExt, DialogExtManual, FileChooserExt, FileExt, GLAreaExt, GtkWindowExt, NativeDialogExtManual, ToggleButtonExt};
use gtk::prelude::ColorChooserExt;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita as adw;

use Tool::*;

use crate::application::Application;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::GliumArea;
use crate::glium_area::hover_state::HoverState;
use crate::glium_area::renderer::ModelCell;
use crate::glium_area::skin_dialog::SkinDialog;
use crate::glium_area::skin_parser::ModelType;
use crate::window::imp::Command;

mod imp;


#[derive(Copy, Clone)]
pub enum Tool {
    Pencil,
    Rubber,
    ColorPicker,
    Fill,
    Dither,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Pencil
    }
}


struct Pencil {
    prev_cell: ModelCell,
    new_cell: ModelCell,
}
impl Command for Pencil {
    fn execute(&self, gl_area: &GliumArea) {
        let renderer = gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.set_cell(&self.new_cell);
        gl_area.queue_draw();
    }
    fn undo(&self, gl_area: &GliumArea) {
        let renderer = gl_area.renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.set_cell(&self.prev_cell);
        gl_area.queue_draw();
    }
}
impl Pencil {
    pub fn new(target_cell: ModelCell, new_color: [f32; 4]) -> Pencil {
        let new_cell = ModelCell {
            body_part: target_cell.body_part.clone(),
            cell_index: target_cell.cell_index,
            color: new_color
        };

        Pencil {
            prev_cell: target_cell,
            new_cell
        }
    }
}


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let win = glib::Object::builder::<Window>()
            .property("application", app)
            .build();

        win.connect_signals();

        win
    }

    fn connect_signals(&self) {
        self.connect_open_button();
        self.connect_save_button();
        self.connect_model_switcher();
        self.connect_gl_area();

        // load icons
        let pencil_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/pencil.png");
        let rubber_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/eraser.png");
        let color_picker_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/color_picker.png");
        self.imp().pencil.set_child(Some(&pencil_ico));
        self.imp().rubber.set_child(Some(&rubber_ico));
        self.imp().color_picker.set_child(Some(&color_picker_ico));

        self.imp().pencil.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Pencil); }));
        self.imp().rubber.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Rubber); }));
        self.imp().color_picker.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::ColorPicker); }));

        self.imp().grid_toggle.connect_toggled(
            clone!(@weak self as win => move |btn| {
                let gl_area = win.imp().gl_area.get();
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                renderer.set_grid_show(btn.is_active());
                gl_area.queue_draw();
            })
        );
    }

    fn activate_pencil_toggle(&self) {
        self.imp().pencil.set_active(true);
    }

    fn connect_model_switcher(&self) {
        let model_switcher = self.imp().model_switcher.get();
        let gl_area = self.imp().gl_area.get();

        model_switcher.inner_layer_toggle().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let ms = model_switcher.clone();
                let inner_is_active = ms.inner_layer_toggle().is_active();
                renderer.set_body_part_active(&Head,     inner_is_active && ms.head().is_active());
                renderer.set_body_part_active(&Torso,    inner_is_active && ms.torso().is_active());
                renderer.set_body_part_active(&RightArm, inner_is_active && ms.right_arm().is_active());
                renderer.set_body_part_active(&LeftArm,  inner_is_active && ms.left_arm().is_active());
                renderer.set_body_part_active(&RightLeg, inner_is_active && ms.right_leg().is_active());
                renderer.set_body_part_active(&LeftLeg,  inner_is_active && ms.left_leg().is_active());
                gl_area.queue_draw();
            }
        ));

        model_switcher.outer_layer_toggle().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let ms = model_switcher.clone();

                let outer_is_active = model_switcher.outer_layer_toggle().is_active();
                renderer.set_body_part_active(&HeadOuter, outer_is_active && ms.head().is_active());
                renderer.set_body_part_active(&TorsoOuter, outer_is_active && ms.torso().is_active());
                renderer.set_body_part_active(&RightArmOuter, outer_is_active && ms.right_arm().is_active());
                renderer.set_body_part_active(&LeftArmOuter, outer_is_active && ms.left_arm().is_active());
                renderer.set_body_part_active(&RightLegOuter, outer_is_active && ms.right_leg().is_active());
                renderer.set_body_part_active(&LeftLegOuter, outer_is_active && ms.left_leg().is_active());

                gl_area.queue_draw();
            }
        ));

        model_switcher.head().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&Head, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&HeadOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.torso().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&Torso, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&TorsoOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.left_arm().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftArm, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftArmOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.right_arm().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightArm, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightArmOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.left_leg().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftLeg, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftLegOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.right_leg().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightLeg, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightLegOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
    }

    fn connect_open_button(&self) {
        let file_chooser_dialog = FileChooserNative::new(
            Some("Open a skin"),
            Some(self),
            FileChooserAction::Open,
            Some("Open"),
            Some("Cancel")
        );

        self.imp().open_button.connect_clicked(clone!(@weak self as win => move |btn: &gtk::Button| {
            let gl_area = win.imp().gl_area.get();
            let win_clone_2 = win.clone();
            file_chooser_dialog.run_async(clone!(@weak win => move |this, _response| {
                if this.file().is_none() {
                    return;
                }

                let file = this.file().unwrap();
                let texture_path = {
                    let p = file.path().unwrap();
                    let x = p.to_str().unwrap();
                    String::from(x)
                };

                let skin_dialog = SkinDialog::new(texture_path);
                skin_dialog.set_transient_for(Some(&win));
                skin_dialog.set_parent(&win);

                skin_dialog.imp().slim.connect_clicked(
                    clone!(@weak win, @weak skin_dialog => move |btn| {
                        println!("slim");
                        let renderer = win.imp().gl_area.renderer().unwrap();
                        let mut renderer = renderer.borrow_mut();

                        let texture_path = skin_dialog.imp().texture_path.borrow();
                        let texture_path = texture_path.as_ref().unwrap().as_str();

                        let model_type = ModelType::Slim;
                        renderer.reset_model_type(&model_type);
                        renderer.load_texture(texture_path, &model_type);
                        win.imp().drawing_history.borrow_mut().clear();
                        win.imp().gl_area.queue_draw();
                        skin_dialog.destroy();
                    })
                );

                skin_dialog.imp().classic.connect_clicked(
                    clone!(@weak win_clone_2 as win, @weak skin_dialog => move |btn| {
                        println!("classic");
                        let renderer = win.imp().gl_area.renderer().unwrap();
                        let mut renderer = renderer.borrow_mut();

                        let texture_path = skin_dialog.imp().texture_path.borrow();
                        let texture_path = texture_path.as_ref().unwrap().as_str();

                        let model_type = ModelType::Classic;
                        renderer.reset_model_type(&model_type);

                        renderer.load_texture(texture_path, &model_type);
                        win.imp().drawing_history.borrow_mut().clear();
                        win.imp().gl_area.queue_draw();
                        skin_dialog.destroy();
                    })
                );

                skin_dialog.present();
            }));
        }));
    }

    fn connect_save_button(&self) {
        let dialog = FileChooserNative::new(
            Some("Save a skin"),
            Some(self),
            FileChooserAction::Save,
            Some("Save"),
            Some("Cancel")
        );
        dialog.set_current_name("untitled.png");
        let win = self.clone();
        self.imp().save_button.connect_clicked(move |btn: &gtk::Button| {
            let gl_area = win.imp().gl_area.get();
            dialog.run_async(move |this, _response| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let file = match this.file() {
                    Some(file) => file,
                    None => {
                        println!("Saving rejected");
                        return;
                    },
                };

                let pathbuf = match file.path() {
                    Some(pathbuf) => pathbuf,
                    None => {
                        println!("File has no path");
                        return;
                    }
                };
                let path = match pathbuf.to_str() {
                    Some(path) => path,
                    None => {
                        println!("Unsupported encoding of path");
                        return;
                    }
                };

                let imgbuf = renderer.export_texture();
                match imgbuf.save(path) {
                    Ok(_) => println!("Saved at {}", path),
                    Err(error) => println!("{}", error),
                }
            });
        });
    }
    
    fn connect_gl_area(&self) {
        // --- CALCULATE FPS ---
        let frame_count = Rc::new(Cell::new(0));
        let current_time = Rc::new(Cell::new(Instant::now()));
        let previous_time = Rc::new(Cell::new(Instant::now()));
        self.imp().gl_area.connect_render(
            clone!(@strong frame_count, @strong current_time, @strong previous_time => move |_, _| {
                frame_count.set(frame_count.get() + 1);
                current_time.set(Instant::now());
                let elapsed_time = current_time.get().duration_since(previous_time.get());
                if elapsed_time.as_secs_f32() > 1.0 {
                    let fps = (frame_count.get() as f32) / elapsed_time.as_secs_f32();
                    println!("FPS: {:.2}", fps);
                    frame_count.set(0);
                    previous_time.set(current_time.get());
                }
                Propagation::Proceed
            })
        );

        // --- CLICK EVENTS ---
        let click = GestureClick::new();
        // --- PRESSED ---
        click.connect_pressed(
            clone!(@weak self as win => move |_, _, x, y| {
                let gl_area = &win.imp().gl_area;
                let renderer = gl_area.renderer().unwrap();
                let (x, y) = (x as f32, y as f32);

                let cell_opt = renderer.borrow().get_cell(x, y, false);
                if let Some(cell) = cell_opt {
                    match win.imp().current_tool.get() {
                        Pencil => {
                            let rgba = win.imp().color_button.rgba();
                            let new_color: [f32; 4] = [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()];
                            let target_cell = ModelCell {
                                body_part: cell.body_part.clone(),
                                cell_index: cell.cell_index,
                                color: cell.color
                            };
                            let command = Pencil::new(target_cell, new_color);
                            command.execute(gl_area);
                            win.imp().drawing_history.borrow_mut().add_command(Box::new(command));
                        },
                        Rubber => {
                            let new_color = [0.0, 0.0, 0.0, 0.0];
                            let target_cell = ModelCell {
                                body_part: cell.body_part.clone(),
                                cell_index: cell.cell_index,
                                color: cell.color
                            };
                            let command = Pencil::new(target_cell, new_color);
                            command.execute(gl_area);
                            win.imp().drawing_history.borrow_mut().add_command(Box::new(command));
                        },
                        ColorPicker => {
                            let clicked_cell = renderer.borrow().get_cell(x, y, true);
                            if let Some(cell) = clicked_cell {
                                let color = cell.color;
                                let rgba = RGBA::new(color[0], color[1], color[2], color[3]);
                                win.imp().color_button.set_rgba(&rgba);
                                win.imp().pencil.set_active(true);
                            }
                        }
                        _ => unimplemented!("This tool is unimplemented yet"),
                    }
                    renderer.borrow_mut().set_mouse_hover(Some(HoverState::OnModel));
                } else {
                    renderer.borrow_mut().set_mouse_hover(Some(HoverState::OnEmptyArea));
                }
                renderer.borrow_mut().start_motion(x, y);
                gl_area.queue_draw();
            })
        );
        // --- RELEASED ---
        let g = self.imp().gl_area.clone();
        click.connect_released(move |_, _, _, _| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            renderer.stop_motion();
            renderer.set_mouse_hover(None);
        });

        // --- MOUSE MOTION ---
        let mv = EventControllerMotion::new();
        let click_event = click.clone();
        let g = self.imp().gl_area.clone();
        mv.connect_motion(move |_, x, y| {
            let renderer = g.renderer().unwrap();
            let mouse_hover_opt = renderer.borrow().get_mouse_hover();
            if let Some(mouse_hover) = mouse_hover_opt {
                match mouse_hover {
                    HoverState::OnModel => {
                        click_event.emit_by_name_with_values("pressed", &[0.into(), x.into(), y.into()]);
                    },
                    HoverState::OnEmptyArea => {
                        renderer.borrow_mut().mouse_move(x as f32, y as f32);
                        renderer.borrow_mut().update_camera();
                    },
                }
                g.queue_draw();
            }
        });

        // --- SCROLL ---
        let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
        let g = self.imp().gl_area.clone();
        scroll.connect_scroll(move |_, _, y| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();

            let distance = (y as f32) * 0.025;
            renderer.update_scale(distance);

            g.queue_draw();
            Propagation::Proceed
        });

        self.imp().gl_area.add_controller(scroll);
        self.imp().gl_area.add_controller(click);
        self.imp().gl_area.add_controller(mv);
    }
}