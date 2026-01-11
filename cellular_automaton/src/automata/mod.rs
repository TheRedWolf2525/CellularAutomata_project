pub mod dummy;
pub mod life;

use crate::automaton::Automaton;

pub fn available() -> Vec<Box<dyn Automaton>> {
    vec![
        Box::new(dummy::Dummy),
        Box::new(life::Life),
    ]
}

pub fn by_name(name: &str) -> Option<Box<dyn Automaton>> {
    available()
        .into_iter()
        .find(|a| a.name().eq_ignore_ascii_case(name))
}
