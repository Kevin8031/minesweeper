use eframe::{run_native, NativeOptions, App};
use eframe::egui::{self, Ui, Button};
use minesweeper::{Game, GameOpts};

#[derive(Default)]
struct CustomGameSettings {
    width: String,
    height: String,
    mines_percentage: String
}

struct MyApp {
    game_opts: GameOpts,
    game: Option<Game>,
    playing: bool,
    custom_game: bool,
    custom_game_settings: CustomGameSettings
}

impl MyApp {
    pub fn new() -> MyApp {
        MyApp { game_opts: GameOpts::default(), game: None, playing: false, custom_game: false, custom_game_settings: CustomGameSettings::default() }
    }

    pub fn new_game(&mut self) {
        self.game = Some(Game::new(&self.game_opts));
    }

    fn custom_game(&mut self, ui: &mut Ui) {
        if ui.button("Back").clicked() {
            self.custom_game = false;
        }

        egui::Grid::new("custom_game").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Width");
                ui.text_edit_singleline(&mut self.custom_game_settings.width);
                ui.label("Height");
                ui.text_edit_singleline(&mut self.custom_game_settings.height);
            });
            ui.end_row();
            ui.horizontal(|ui| {
                ui.label("Mines Percentage");
                ui.text_edit_singleline(&mut self.custom_game_settings.mines_percentage);
            });
            ui.end_row();
            if ui.button("Play").clicked() {
                let settings = &self.custom_game_settings;
                if settings.height.len() > 0
                && settings.width.len() > 0
                && settings.mines_percentage.len() > 0 {
                    let height: usize = match settings.height.trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };

                    let width: usize = match settings.width.trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };

                    let mines_percentage: usize = match settings.mines_percentage.trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };

                    self.game_opts = GameOpts::new(width, height, 0, Some(mines_percentage));
                    self.new_game();
                    self.custom_game = false;
                    self.playing = true;
                }
            }
        });
    }

    fn render_menu(&mut self, ui: &mut Ui) {
        if self.custom_game {
            self.custom_game(ui);
        } else {

            if ui.button("8x8\n10 mines").clicked() {
                self.game_opts = GameOpts::new(8, 8, 10, None);
                self.new_game();
                self.playing = true;
            }
    
            if ui.button("16x16\n40 mines").clicked() {
                self.game_opts = GameOpts::new(16, 16, 40, None);
                self.new_game();
                self.playing = true;
            }
            if ui.button("30x16\n99 mines").clicked() {
                self.game_opts = GameOpts::new(30, 16, 99, None);
                self.new_game();
                self.playing = true;
            }
            if ui.button("?\nCustom").clicked() {
                self.custom_game = true;
            }
        }
    }

    fn render_game(&mut self, ui: &mut Ui) {
        if ui.button("Back").clicked() {
            self.playing = false;
        }
        
        let width = self.game_opts.width();
        let height = self.game_opts.height();

        egui::Grid::new("buttons")
        .num_columns(width)
        .show(ui, |ui| {
            for x in 0..width {
                for y in 0..height {
                    let btn = 
                        Button::new(format!("{}", x * width + y))
                        .wrap(true);
                    if ui.add(btn).clicked() {
                        println!("ciao");
                    }
                }
                ui.end_row();
            }
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.playing {
                self.render_menu(ui);
            } else {
                self.render_game(ui);
            }
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();
    run_native(
        "Minesweeper",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new()))
    );    
}