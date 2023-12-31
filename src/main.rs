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

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
enum CardState {
    Flipped(Color32),
    NotFlipped(Color32),
    Matched(Color32),
    Incorrect(Color32),
}

struct App {
    game_state: Vec<Card>,
    num_matches: usize,
    matched: usize,
    cards_flipped: usize,
    frame: usize,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>, num_matches: usize) -> Self {
        let mut init_cards = Vec::new();
        let mut face_values = vec!["😍", "🌳", "🐼", "🌹", "🔥", "🍰", "🍑", "🎨", "💩", "💎"];

        fastrand::shuffle(&mut face_values);

        for i in 0..num_matches * 2 {
            let mut card: Card = Card::new();
            if i < 10 {
                card.face_value = face_values[i].to_string();
            } else {
                card.face_value = face_values[i - 10].to_string();
            }

            card.flipped = CardState::NotFlipped(Color32::TRANSPARENT);
            init_cards.push(card);
        }

        Self {
            game_state: init_cards,
            num_matches,
            matched: 0,
            cards_flipped: 0,
            frame: 0,
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
                        if card.flipped != CardState::Flipped(Color32::BLUE) {
                            card.flipped = CardState::Flipped(Color32::BLUE);
                            self.cards_flipped += 1;
                        }
                    }

                    let color = match self.game_state[i].flipped {
                        CardState::Flipped(color) => color,
                        CardState::NotFlipped(color) => color,
                        CardState::Matched(color) => color,
                        CardState::Incorrect(color) => color,
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

                    if self.game_state[i].flipped == CardState::Flipped(Color32::BLUE)
                        || self.game_state[i].flipped == CardState::Matched(Color32::GREEN)
                        || (self.game_state[i].flipped == CardState::Incorrect(Color32::RED)
                            && self.frame < 3)
                    {
                        ui.painter_at(rect).text(
                            point_in_screen,
                            egui::Align2::CENTER_CENTER,
                            &self.game_state[i].face_value,
                            FontId::monospace(75.0),
                            Color32::WHITE,
                        );
                    }
                }

                if self.cards_flipped == 2 {
                    self.frame = 0;
                    let mut incorrect: Vec<&mut Card> = self
                        .game_state
                        .iter_mut()
                        .filter(|card| card.flipped == CardState::Incorrect(Color32::RED))
                        .collect();

                    if incorrect.len() == 1 {
                        incorrect.iter_mut().for_each(|item| {
                            item.flipped = CardState::NotFlipped(Color32::TRANSPARENT)
                        });
                        incorrect.pop();
                    }

                    if !incorrect.is_empty() {
                        incorrect[0].flipped = CardState::NotFlipped(Color32::TRANSPARENT);
                        incorrect[1].flipped = CardState::NotFlipped(Color32::TRANSPARENT);
                    }

                    let mut flipped: Vec<&mut Card> = self
                        .game_state
                        .iter_mut()
                        .filter(|card| card.flipped == CardState::Flipped(Color32::BLUE))
                        .collect();

                    if flipped[0].face_value == flipped[1].face_value {
                        flipped[0].flipped = CardState::Matched(Color32::GREEN);
                        flipped[1].flipped = CardState::Matched(Color32::GREEN);

                        self.matched += 1;
                    } else {
                        flipped[0].flipped = CardState::Incorrect(Color32::RED);
                        flipped[1].flipped = CardState::Incorrect(Color32::RED);
                    }
                    self.cards_flipped = 0;
                }

                if self.matched == self.num_matches {
                    ui.heading("You Win!!!");
                }

                self.frame += 1;
            });
        });
    }
}
