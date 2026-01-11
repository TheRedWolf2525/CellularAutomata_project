use std::{thread, time::Duration};

use cellular_automaton::{
    automata,
    engine::Engine,
    render::{terminal::TerminalRenderer, Renderer},
};


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--list") {
        for a in automata::available() {
            println!("{}", a.name());
        }
        return;
    }

    let name = args.get(1).map(|s| s.as_str()).unwrap_or("dummy");
    let automaton = automata::by_name(name).unwrap_or_else(|| {
        eprintln!("Automate inconnu: {name}\nUtilise --list");
        std::process::exit(2);
    });

    let mut engine = Engine::new(40, 20, automaton);
    let mut renderer = TerminalRenderer::new();

    loop {
        renderer.render(engine.current());
        engine.step_once();
        thread::sleep(Duration::from_millis(80));
    }
}
