use eframe::egui;
use rand::Rng;
use std::{thread, time::Duration};

struct SortVisualizer {
    numbers: Vec<i32>,
    is_sorting: bool,
}

impl Default for SortVisualizer {
    fn default() -> Self {
        Self {
            numbers: (0..50).map(|_| rand::thread_rng().gen_range(10..400)).collect(),
            is_sorting: false,
        }
    }
}

impl SortVisualizer {
    fn bubble_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();

        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();

        thread::spawn(move || {
            for i in 0..len {
                for j in 0..len - i - 1 {
                    if numbers_clone[j] > numbers_clone[j + 1] {
                        numbers_clone.swap(j, j + 1);
                    }
                    thread::sleep(Duration::from_millis(30)); 
                    ctx_clone.request_repaint(); 
                }
            }
        });
    }

    fn reset_array(&mut self) {
        self.numbers = (0..50).map(|_| rand::thread_rng().gen_range(10..400)).collect();
        self.is_sorting = false;
    }
}

impl eframe::App for SortVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Start Sorting").clicked() && !self.is_sorting {
                    self.bubble_sort(ctx);
                }
                if ui.button("Reset").clicked() {
                    self.reset_array();
                }
            });

            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                0.0,
                egui::Color32::BLACK,
            );

            let width = ui.available_width();
            let bar_width = width / self.numbers.len() as f32;
            
            for (i, &val) in self.numbers.iter().enumerate() {
                let height = val as f32;
                let x = i as f32 * bar_width;
                let rect = egui::Rect::from_min_size(
                    egui::pos2(x, ui.max_rect().bottom() - height),
                    egui::vec2(bar_width - 2.0, height),
                );
                ui.painter().rect_filled(rect, 0.0, egui::Color32::LIGHT_BLUE);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualizer",
        options,
        Box::new(|_cc| Box::<SortVisualizer>::default()),
    )
}
