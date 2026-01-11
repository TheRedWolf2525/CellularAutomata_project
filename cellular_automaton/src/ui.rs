use std::time::{Duration, Instant};
use eframe::egui;
use crate::{automata, engine::Engine};

const PALETTE: [egui::Color32; 8] = [
    egui::Color32::BLACK,       // 0
    egui::Color32::WHITE,       // 1
    egui::Color32::DARK_BLUE,   // 2
    egui::Color32::PURPLE,      // 3
    egui::Color32::GREEN,       // 4
    egui::Color32::DARK_GREEN,  // 5
    egui::Color32::RED,         // 6
    egui::Color32::GOLD,         // 7
];


pub struct App {
    engine: Engine,
    running: bool,
    step_ms: u64,
    selected: String,

    // time sync
    last_frame: Instant,
    acc: Duration,

    save_name: String,
    status: String,
    grids: Vec<String>,
    selected_grid: String, 
}

impl App {
    pub fn new() -> Self {
        let default = "mazesolver";
        let automaton = automata::by_name(default).unwrap_or_else(|| automata::available().remove(0));
        let grids = crate::io::bin::list_grids().unwrap_or_default();
        let selected_grid = grids.get(0).cloned().unwrap_or_default();
        
        Self {
            engine: Engine::new(80, 45, automaton),
            running: true,
            step_ms: 300,
            selected: default.to_string(),

            last_frame: Instant::now(),
            acc: Duration::ZERO,

            save_name: "grid1".to_string(),
            status: String::new(),
            grids,
            selected_grid,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- barre de contrôle
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                
                // Pause button
                if ui.button(if self.running { "Pause" } else { "Run" }).clicked() {
                    self.running = !self.running;
                    self.last_frame = Instant::now();
                    self.acc = Duration::ZERO;
                }

                // Step button
                if ui.button("Step").clicked() {
                    self.engine.step_once();
                    self.last_frame = Instant::now();
                    self.acc = Duration::ZERO;
                }
                
                // Speed slider
                ui.add(egui::Slider::new(&mut self.step_ms, 1..=500).text("ms/step"));

                egui::ComboBox::from_label("Automate")
                    .selected_text(&self.selected)
                    .show_ui(ui, |ui| {
                        for a in automata::available() {
                            let name = a.name().to_string();
                            if ui.selectable_label(self.selected == name, &name).clicked() {
                                self.selected = name.clone();
                                self.engine.set_automaton(a);
                                self.last_frame = Instant::now();
                                self.acc = Duration::ZERO;
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                // Save/Load
                ui.label("Save:");
                ui.text_edit_singleline(&mut self.save_name);

                if ui.button("Save").clicked() {
                    let mut file = self.save_name.trim().to_string();
                    if file.is_empty() {
                        self.status = "Nom vide".to_string();
                    } else {
                        if !file.ends_with(".cagr") {
                            file.push_str(".cagr");
                        }
                        let path = crate::io::bin::path_in_dir(&file);
                        match crate::io::bin::save(&path, self.engine.current()) {
                            Ok(()) => {
                                self.status = format!("Saved: {:?}", path);
                                self.grids = crate::io::bin::list_grids().unwrap_or_default();
                                self.selected_grid = file;
                            }
                            Err(e) => self.status = format!("Save error: {e:?}"),
                        }
                    }
                }

                if ui.button("Refresh").clicked() {
                    self.grids = crate::io::bin::list_grids().unwrap_or_default();
                    if self.selected_grid.is_empty() {
                        self.selected_grid = self.grids.get(0).cloned().unwrap_or_default();
                    }
                }

                egui::ComboBox::from_id_salt("grid_load_combo")
                    .selected_text(if self.selected_grid.is_empty() { "(none)" } else { &self.selected_grid })
                    .show_ui(ui, |ui| {
                        for name in &self.grids {
                            ui.selectable_value(&mut self.selected_grid, name.clone(), name);
                        }
                    });

                if ui.button("Load").clicked() {
                    if self.selected_grid.is_empty() {
                        self.status = "Aucune grille".to_string();
                    } else {
                        let path = crate::io::bin::path_in_dir(&self.selected_grid);
                        match crate::io::bin::load(&path) {
                            Ok(g) => {
                                self.engine.set_grid(g);
                                self.status = format!("Loaded: {:?}", path);
                            }
                            Err(e) => self.status = format!("Load error: {e:?} (path={:?})", path),
                        }
                    }
                }

                if !self.status.is_empty() {
                    ui.label(&self.status);
                }

            });
        });

        // rendu 
        egui::CentralPanel::default().show(ctx, |ui| {
            let g = self.engine.current();
            let avail = ui.available_size();
            let cell_w = (avail.x / g.width() as f32).floor().max(1.0);
            let cell_h = (avail.y / g.height() as f32).floor().max(1.0);
            let cell = cell_w.min(cell_h);

            let (rect, _resp) = ui.allocate_exact_size(
                egui::vec2(cell * g.width() as f32, cell * g.height() as f32),
                egui::Sense::hover(),
            );
            let painter = ui.painter_at(rect);

            for y in 0..g.height() {
                for x in 0..g.width() {
                    let v = g.get(x, y) as usize;
                    if v == 0 { continue; }

                    let color = PALETTE.get(v).copied().unwrap_or(egui::Color32::GRAY);

                    let min = rect.min + egui::vec2(x as f32 * cell, y as f32 * cell);
                    let r = egui::Rect::from_min_size(min, egui::vec2(cell, cell));
                    painter.rect_filled(r, 0.0, color);
                }
            }
        });

        // --- simulateur stable (accumulateur)
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame);
        self.last_frame = now;

        if self.running {
            let step = Duration::from_millis(self.step_ms);
            self.acc += dt;

            // cap: évite de passer 2s à rattraper si l'UI freeze
            const MAX_STEPS_PER_FRAME: usize = 8;

            let mut nsteps = 0usize;
            while self.acc >= step && nsteps < MAX_STEPS_PER_FRAME {
                self.engine.step_once();
                self.acc -= step;
                nsteps += 1;
            }

            // si on est trop en retard, on drop l'excès
            if nsteps == MAX_STEPS_PER_FRAME {
                self.acc = Duration::ZERO;
            }

            // repaint asap, l'update est déjà régulé par l'accu
            ctx.request_repaint();
        } else {
            // en pause, on n'a pas besoin de repaints continus
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }
}
