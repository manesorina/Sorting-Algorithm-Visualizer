mod sorting;
mod ui;

use eframe::NativeOptions;
use ui::SortingApp;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Sorting Visualizer",
        options,
        Box::new(|_cc| Box::<SortingApp>::default()),
    )
}
