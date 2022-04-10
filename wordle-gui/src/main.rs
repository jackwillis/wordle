#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi};

struct WordleApp {
    secret_word: wordle::Word,
}

impl WordleApp {
    fn new() -> Self {
        Self {
            secret_word: wordle::random_word(),
        }
    }
}

impl epi::App for WordleApp {
    fn name(&self) -> &str {
        "Wordle"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let text = format!("Today's word is: {}", self.secret_word);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(text);
        });
    }
}

fn main() {
    let app = WordleApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
