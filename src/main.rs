use egui::{emath::RectTransform, Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1400.0, 825.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Match Game",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, 10))),
    )
}

struct Card {
    face_value: String,
    flipped: CardState,
}

impl Card {
    fn new() -> Self {
        Self {
            face_value: "init".to_string(),
            flipped: CardState::NotFlipped(Color32::TRANSPARENT),
        }
    }
}

enum CardState {
    Flipped(Color32),
    NotFlipped(Color32),
}

struct App {
    game_state: Vec<Card>,
    num_matches: usize,
    cards_flipped: usize,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>, num_matches: usize) -> Self {
        let mut init_cards = Vec::new();

        for _ in 0..num_matches * 2 {
            let mut card: Card = Card::new();
            card.face_value = "Test".to_string();
            card.flipped = CardState::NotFlipped(Color32::TRANSPARENT);
            init_cards.push(card);
        }

        Self {
            game_state: init_cards,
            num_matches,
            cards_flipped: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(25.0, 25.0);
                for i in 0..self.num_matches * 2 {
                    let (id, rect) = ui.allocate_space(egui::vec2(150.0, 250.0));

                    let response = ui.interact(rect, id, egui::Sense::click());

                    if response.clicked() {
                        let card = &mut self.game_state[i];
                        card.flipped = CardState::Flipped(Color32::BLUE);
                        self.cards_flipped += 1;
                    }

                    let color = match self.game_state[i].flipped {
                        CardState::Flipped(color) => color,
                        CardState::NotFlipped(color) => color,
                    };

                    ui.painter().rect(
                        rect,
                        Rounding::from(5.0),
                        color,
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
                        &self.game_state[i].face_value,
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    );
                }

                if self.cards_flipped == 2 {
                    // TODO Check if match if so continue and set cards flipped to 0

                    // TODO else
                    for i in 0..self.num_matches * 2 {
                        let card = &mut self.game_state[i];
                        card.flipped = CardState::NotFlipped(Color32::TRANSPARENT);
                    }
                    self.cards_flipped = 0;
                }
            });
        });
    }
}
