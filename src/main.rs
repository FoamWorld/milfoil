use crate::config::*;
use app::MyApp;
use eframe::egui;
use font_kit::source::SystemSource;

mod app;
mod config;
mod message;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([1200.0, 675.0])
            .with_icon(egui::IconData::default()),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| {
            load_font(&cc.egui_ctx);
            Ok(Box::new(MyApp::new()))
        }),
    )
    .expect("Failed to launch application.");
}

fn load_font(ctx: &egui::Context) {
    // for font in system_source.all_fonts().unwrap() {
    //     let data = font.load().unwrap();
    //     println!(
    //         "{0} ({1})",
    //         data.full_name(),
    //         data.postscript_name().unwrap_or("???".to_string())
    //     );
    // }
    let registered_name = "system_font";
    let system_source = SystemSource::new();

    let handle = system_source
        .select_by_postscript_name(DEFAULT_POSTSCRIPT)
        .or(system_source.select_best_match(
            &[font_kit::family_name::FamilyName::SansSerif],
            &font_kit::properties::Properties::new(),
        ));
    if let Ok(handle) = handle {
        let font_data = handle.load().unwrap();
        eprintln!(
            "Loaded font: {0} ({1})",
            font_data.full_name(),
            font_data.postscript_name().unwrap_or("???".to_string())
        );

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            registered_name.to_string(),
            egui::FontData::from_owned(font_data.copy_font_data().unwrap().to_vec()).into(),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, registered_name.to_string());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, registered_name.to_string());

        ctx.set_fonts(fonts);
    };
}
