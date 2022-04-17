#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi};

struct WordleApp {
    secret_word: wordle::Word,

    input: String,
    predictions: Vec<String>,
}

impl WordleApp {
    fn new() -> Self {
        Self {
            secret_word: wordle::random_word(),
            input: String::new(),
            predictions: Vec::new(),
        }
    }
}

impl epi::App for WordleApp {
    fn name(&self) -> &str {
        "Wordle"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Today's word is: {}", self.secret_word));

            let text_edit = ui.text_edit_singleline(&mut self.input);

            if text_edit.lost_focus() {
                self.predictions.push(self.input.clone());
            }

            if !text_edit.has_focus() {
                text_edit.request_focus();
            }

            for prediction in &self.predictions {
                match prediction.parse::<wordle::Word>() {
                    Ok(word) => {
                        ui.label(format!("Ok: {}", word));
                    }
                    Err(msg) => {
                        ui.label(format!("Err: {}", msg));
                    }
                }
            }
        });
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {}

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}

fn main() {
    let app = WordleApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
