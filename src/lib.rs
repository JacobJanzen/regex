use std::collections::HashMap;
use std::collections::HashSet;
struct DFA {
    number_of_states: u32,
    transistions: HashMap<(u32, Option<char>), u32>,
    accepting_states: HashSet<u32>,
}

impl DFA {
    fn new(
        number_of_states: u32,
        transistions: HashMap<(u32, Option<char>), u32>,
        accepting_states: HashSet<u32>,
    ) -> DFA {
        DFA {
            number_of_states,
            transistions,
            accepting_states,
        }
    }

    fn run(&self, input: String) -> bool {
        let mut current_state = 0;
        let mut try_general_case = false;

        for c in input.chars() {
            if current_state >= self.number_of_states {
                return false;
            }
            let state_transition = (current_state, Some(c));
            match self.transistions.get(&state_transition) {
                Some(new_state) => current_state = *new_state,
                None => try_general_case = true,
            };

            if try_general_case {
                let state_transition = (current_state, None);
                match self.transistions.get(&state_transition) {
                    Some(new_state) => current_state = *new_state,
                    None => return false,
                }
            }
        }

        if current_state < self.number_of_states && self.accepting_states.contains(&current_state) {
            return true;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_on_empty_string() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(0);
        let dfa = DFA::new(1, HashMap::new(), accepting_states);

        assert!(dfa.run("".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transistions = HashMap::new();
        transistions.insert((0, Some('0')), 1);
        let dfa = DFA::new(2, transistions, accepting_states);

        assert!(!dfa.run("".to_string()));
    }

    #[test]
    fn run_on_string_length_one() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transistions = HashMap::new();
        transistions.insert((0, Some('0')), 1);
        let dfa = DFA::new(2, transistions, accepting_states);

        assert!(dfa.run("0".to_string()));
        assert!(!dfa.run("1".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        accepting_states.insert(2);
        let mut transistions = HashMap::new();
        transistions.insert((0, Some('0')), 1);
        transistions.insert((0, Some('1')), 2);
        let dfa = DFA::new(3, transistions, accepting_states);

        assert!(dfa.run("1".to_string()));
    }

    #[test]
    fn invalid_transition() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transistions = HashMap::new();
        transistions.insert((0, Some('0')), 1);
        let dfa = DFA::new(1, transistions, accepting_states);

        assert!(!dfa.run("0".to_string()));
    }

    #[test]
    fn transition_on_all() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transistions = HashMap::new();
        transistions.insert((0, None), 1);
        let dfa = DFA::new(2, transistions, accepting_states);

        assert!(dfa.run("0".to_string()));
    }
}
