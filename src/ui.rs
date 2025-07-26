use eframe::egui;

pub fn add_sidebar(ctx: &egui::Context) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(480.0)
        .frame(
            egui::Frame::NONE
                .outer_margin(egui::Margin::same(0))
                .inner_margin(egui::Margin::rightf(4.0.into())),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("菜单");
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Dark").clicked() {
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                    if ui.button("Light").clicked() {
                        ctx.set_visuals(egui::Visuals::light());
                    }
                });
            });
        });
}
