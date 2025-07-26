use crate::message::MessageLog;
use crate::ui::*;
use eframe::egui;

pub struct MyApp {
    pub message_log: MessageLog,
    pub show_sidebar: bool,
}

impl MyApp {
    pub fn new() -> Self {
        let message_log = MessageLog::default();
        Self {
            message_log,
            show_sidebar: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_sidebar = !self.show_sidebar;
        }

        if self.show_sidebar {
            add_sidebar(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);
            });
            ui.horizontal(|ui| {
                if ui.button("Add Message").clicked() {
                    self.message_log.add_message("Message!".to_string());
                }
                if ui.button("Clear All").clicked() {
                    self.message_log.messages.lock().unwrap().clear();
                }
            });
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let messages = self.message_log.messages.lock().unwrap();

                    for msg in messages.iter() {
                        ui.horizontal(|ui| {
                            ui.label(msg);
                        });
                    }

                    if ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    }
                });
        });
    }
}
