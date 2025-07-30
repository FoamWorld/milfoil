use crate::app::MyApp;
use eframe::egui::{self, Color32, Context, FontFamily, Stroke, Style, TextStyle};

const DF_FONT_ID: egui::FontId = egui::FontId::new(20.0, FontFamily::Proportional);
const FG_LIGHT_INACTIVE: Color32 = Color32::from_rgb(0xb9, 0x7c, 0x2c);

pub fn setup_custom_styles(ctx: &Context) {
    ctx.all_styles_mut(|style: &mut Style| {
        style.spacing.button_padding = egui::Vec2::new(12.0, 8.0);
        style.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);

        style.text_styles.insert(TextStyle::Body, DF_FONT_ID);
        style.text_styles.insert(TextStyle::Button, DF_FONT_ID);
        style.text_styles.insert(TextStyle::Heading, DF_FONT_ID);

        style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.active.bg_fill = Color32::TRANSPARENT;

        style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    });
}

pub fn add_sidebar(app: &mut MyApp, ctx: &Context) {
    let lc = app.locales.borrow();
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(480.0)
        .show(ctx, |ui| {
            let style = ui.style_mut();
            style.visuals.button_frame = false;

            ui.vertical(|ui| {
                ui.heading(lc.translate("app-menu-title", None));
                ui.separator();
                if ui.button(lc.translate("app-menu-info", None)).clicked() {
                    app.messages
                        .borrow_mut()
                        .push(lc.translate("app-info-about", None), 1, 1);
                }
                if ui
                    .button(lc.translate("app-menu-change_theme", None))
                    .clicked()
                {
                    let len = app.visuals.len();
                    app.selected_visual += 1;
                    if app.selected_visual == len {
                        app.selected_visual = 0;
                    };
                    ctx.set_visuals(app.visuals[app.selected_visual].clone());
                }
                if ui.button(lc.translate("app-menu-quit", None)).clicked() {
                    std::process::exit(0);
                }
            });
        });
}

pub fn add_central(app: &mut MyApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let style = ui.style_mut();
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, FG_LIGHT_INACTIVE);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let ml = app.messages.borrow();

                for message in &ml.messages {
                    ui.horizontal(|ui| {
                        message.display(ui);
                    });
                }

                ui.separator();

                let ab = app.actions_bundle.borrow();
                if !ab.is_empty() {
                    ui.horizontal(|ui| {
                        for (_, actions) in &ab.dict {
                            ui.menu_button(actions.name.clone(), |ui| {
                                for action in &actions.list {
                                    if ui.small_button(action.name.clone()).clicked() {
                                        let _: () = action.func.call(()).unwrap();
                                    }
                                }
                            });
                        }
                    });
                }

                if ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }
            });
    });
}
