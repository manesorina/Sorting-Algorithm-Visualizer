use egui;
use crate::sorting::SortVisualizer;


pub struct SortingApp {
    visualizer: SortVisualizer,
    original_array: Vec<usize>, 
}

impl Default for SortingApp {
    fn default() -> Self {
        let default_visualizer = SortVisualizer::default();
        Self {
            original_array: default_visualizer.numbers.clone(),
            visualizer: default_visualizer,
        }
    }
}

impl eframe::App for SortingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(rx) = &self.visualizer.rx {
            if let Ok(new_numbers) = rx.try_recv() {
                self.visualizer.numbers = new_numbers;
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let sorting_disabled = self.visualizer.is_sorting;
                
                if ui.add_enabled(!sorting_disabled, egui::Button::new("Bubble Sort")).clicked() {
                    self.original_array = self.visualizer.numbers.clone();
                    self.visualizer.bubble_sort(ctx);
                }
                if ui.add_enabled(!sorting_disabled, egui::Button::new("Insertion Sort")).clicked() {
                    self.original_array = self.visualizer.numbers.clone();
                    self.visualizer.insertion_sort(ctx);
                }
                if ui.add_enabled(!sorting_disabled, egui::Button::new("Selection Sort")).clicked() {
                    self.original_array = self.visualizer.numbers.clone();
                    self.visualizer.selection_sort(ctx);
                }
                if ui.add_enabled(!sorting_disabled, egui::Button::new("Quick Sort")).clicked() {
                    self.original_array = self.visualizer.numbers.clone();
                    self.visualizer.quick_sort(ctx);
                }

                if ui.add_enabled(!sorting_disabled, egui::Button::new("Merge Sort")).clicked(){
                    self.original_array=self.visualizer.numbers.clone();
                    self.visualizer.merge_sort(ctx);
                }
                
                if ui.add_enabled(self.visualizer.is_sorting, egui::Button::new("Stop")).clicked() {
                    self.stop_sorting();
                }
                
                if ui.button("Reset Array").clicked() {
                    self.reset_to_original_array();
                }
                
                if ui.button("New Array").clicked() {
                    self.generate_new_array();
                }
                
                ui.add_space(20.0);
                ui.checkbox(&mut self.visualizer.show_scanning, "Show scanning process");
            });
            
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                0.0,
                egui::Color32::BLACK,
            );
            
            let width = ui.available_width();
            let bar_width = width / self.visualizer.numbers.len() as f32;
            
            for (i, &val) in self.visualizer.numbers.iter().enumerate() {
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

impl SortingApp {
    fn stop_sorting(&mut self) {
        self.visualizer.rx = None;
        
        self.visualizer.is_sorting = false;
    }
    
    fn reset_to_original_array(&mut self) {
        self.stop_sorting();
        
        self.visualizer.numbers = self.original_array.clone();
    }
    
    fn generate_new_array(&mut self) {
        self.stop_sorting();
        
        self.visualizer.reset_array();
        self.original_array = self.visualizer.numbers.clone();
    }
}