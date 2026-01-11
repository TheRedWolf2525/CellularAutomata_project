use cellular_automaton::ui::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cellular Automata",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
