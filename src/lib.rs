mod dfa;
use dfa::DFA;
use std::collections::HashMap;
use std::collections::HashSet;
pub fn compile() {
    let mut accepting_states = HashSet::new();
    accepting_states.insert(1);
    let mut transitions = HashMap::new();
    transitions.insert((0, None), 1);
    transitions.insert((0, Some('0')), 2);
    let dfa = DFA::new(transitions, accepting_states);

    println!("{}", dfa.run("alkjlj;".to_string()));
}
