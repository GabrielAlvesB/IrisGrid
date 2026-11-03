use super::Dialog;

/// Help ▸ About IrisGrid: the app, its contributors and the AI models that helped. The credits
/// are compiled in (`crate::credits`, docs/contributors.md).
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            ui.heading("IrisGrid");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.add_space(4.0);
            ui.label("A clean-room, open-source, Rust-native spreadsheet. Part of the Iris ecosystem.");
            ui.label("Based on GridCraft by the ArtCraft team.");
            ui.hyperlink_to("Source code: github.com/GabrielAlvesB/IrisGrid", "https://github.com/GabrielAlvesB/IrisGrid");
            ui.hyperlink_to("Original project: github.com/storytold/gridcraft", "https://github.com/storytold/gridcraft");
            ui.label(egui::RichText::new("MIT OR Apache-2.0. Copyright (c) 2026 ArtCraft Team and the GridCraft contributors.").small().weak());
        }
    }
}
