use std::cell::RefCell;

use eframe::epaint::Color32;
use eframe::{run_native, NativeOptions, App};
use eframe::egui::{self, Ui, Button, Window};
use minesweeper::{Game, GameOpts};

#[derive(Default)]
struct CustomGameSettings {
    width: String,
    height: String,
    mines_percentage: String
}

#[derive(Clone)]
struct ButtonsOptions {
    text: RefCell<String>,
    color: RefCell<Color32>,
}

impl Default for ButtonsOptions {
    fn default() -> Self {
        Self { text: RefCell::new(String::new()), color: RefCell::new(Color32::WHITE) }
    }
}

struct MyApp {
    game_opts: GameOpts,
    game: Option<Game>,
    playing: bool,
    custom_game: bool,
    custom_game_settings: CustomGameSettings,
    buttons: Vec<ButtonsOptions>,
    game_over: bool
}

impl MyApp {
    pub fn new() -> MyApp {
        MyApp {
            game_opts: GameOpts::default(),
            game: None,
            playing: false,
            custom_game: false,
            custom_game_settings:
            CustomGameSettings::default(),
            buttons: Vec::new(),
            game_over: false
        }
    }

    pub fn new_game(&mut self) {
        self.game = Some(Game::new(&self.game_opts));
        self.buttons = vec![ButtonsOptions::default(); self.game_opts.width() * self.game_opts.height()];
        self.game.as_ref().unwrap().print_map();
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
            ui.end_row();
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

    fn render_game(&mut self, ctx: &egui::Context, ui: &mut Ui) {
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
                    let index = x * width + y;
                    let button = self.buttons.get(index).unwrap();
                    let btn = Button::new(button.text.borrow().to_string())
                    .wrap(true)
                    .fill(button.color.borrow().clone());
                    
                    let btn = ui.add(btn);
                    
                    if btn.clicked() {
                        if let Some(game) = &self.game {
                            match game.check_move(index, &mut None) {
                                Some(celles) => {
                                    for (index, cell) in celles {
                                        let button = self.buttons.get(index).unwrap();
                                        if cell.nearby_mines() == 0 {
                                            *button.color.borrow_mut() = Color32::GRAY;
                                        } else {
                                            *button.text.borrow_mut() = cell.nearby_mines().to_string();
                                        }
                                    }
                                },
                                None => {
                                    *button.color.borrow_mut() = Color32::RED;
                                    self.game_over = true;
                                },
                            }
                        }
                    }
                }
                ui.end_row();
            }
        });

        if self.game_over {
            Window::new("Game Over").show(ctx, |ui| {
                if ui.button("Coglione").clicked() {
                    self.game_over = false;
                    self.playing = false;
                }
            });
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            let layout = egui::Layout::top_down_justified(eframe::emath::Align::Center)
                .with_main_justify(true);
            if !self.playing {
                ui.with_layout(layout, |ui| {
                    egui::Grid::new("menu")
                    .num_columns(2)
                    .show(ui, |ui| {
                        self.render_menu(ui);
                    });
                });
            } else {
                self.render_game(ctx, ui);
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