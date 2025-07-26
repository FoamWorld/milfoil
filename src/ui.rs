use crate::app::MyApp;
use eframe::egui;

pub fn setup_custom_styles(ctx: &egui::Context) {
    ctx.all_styles_mut(|style: &mut egui::Style| {
        style.spacing.button_padding = egui::Vec2::new(12.0, 8.0);
        style.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);

        style.text_styles = [
            (
                egui::TextStyle::Body,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(40.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        style.visuals.button_frame = false;

        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;

        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

        style.visuals.widgets.inactive.fg_stroke =
            egui::Stroke::new(1.0, egui::Color32::from_hex("#b97c2c").unwrap());
    });
}

pub fn add_sidebar(app: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(480.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("菜单");
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("更改主题").clicked() {
                        let len = app.visuals.len();
                        app.selected_visual += 1;
                        if app.selected_visual == len {
                            app.selected_visual = 0;
                        };
                        ctx.set_visuals(app.visuals[app.selected_visual].clone());
                    }
                });
                if ui.button("退出").clicked() {
                    std::process::exit(0);
                }
            });
        });
}

pub fn add_central(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for msg in app.message_log.borrow().messages.iter() {
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
