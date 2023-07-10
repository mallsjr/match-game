use egui::{emath::RectTransform, Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1400.0, 825.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Match Game",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(25.0, 25.0);
                for i in 0..20 {
                    let (id, rect) = ui.allocate_space(egui::vec2(150.0, 250.0));

                    let response = ui.interact(rect, id, egui::Sense::click());

                    if response.clicked() {
                        println!("Clicked");
                    }

                    ui.painter().rect(
                        rect,
                        Rounding::from(5.0),
                        Color32::TRANSPARENT,
                        Stroke::new(1.0, Color32::WHITE),
                    );

                    // Get the relative position of our "canvas"
                    let to_screen =
                        RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, rect.size()), rect);

                    // Create an absolute point
                    let point = Pos2 { x: 75.0, y: 125.0 };
                    // Make the absolute point relative to the "canvas" container
                    let point_in_screen = to_screen.transform_pos(point);
                    // e.g. x: 338.0, y: 245.0

                    ui.painter_at(rect).text(
                        point_in_screen,
                        egui::Align2::CENTER_CENTER,
                        "test",
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    );
                }
            });
        });
    }
}
