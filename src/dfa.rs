use std::collections::HashMap;
use std::collections::HashSet;

/// Represents a deterministic finite automaton
/// # Q: States
/// We don't need to explicitly store this.
/// # q0: Start state
/// Always defined as 0.
/// # Σ: Alphabet
/// Defined as being all unicode characters.
pub struct DFA {
    /// # δ: Transition Function
    /// Specifies a state and its input to get output state.
    /// Uses `None` option to represent unconditional transition;
    /// if `Some(x)` was also specified, it will look at that first.
    transitions: HashMap<(u32, Option<char>), u32>,
    /// # F: Accepting states
    /// Set of states that the DFA will accept in.
    accepting_states: HashSet<u32>,
}

impl DFA {
    /// Create a new DFA
    ///
    /// Give a transition function in the form of a `HashMap`.
    /// Use `None` as the input character to add transition on everything after
    /// the `Some(x)` transitions.
    ///
    /// Give the set of accepting states in the form of a `HashSet`.
    /// Refer to states by an integer.
    pub fn new(
        transitions: HashMap<(u32, Option<char>), u32>,
        accepting_states: HashSet<u32>,
    ) -> DFA {
        DFA {
            transitions,
            accepting_states,
        }
    }

    /// Run the DFA
    ///
    /// give a unicode string as an input.
    /// return true if the DFA accepts.
    /// return false if the DFA rejects.
    pub fn run(&self, input: String) -> bool {
        let mut current_state = 0;
        let mut try_general_case = false;

        for c in input.chars() {
            let state_transition = (current_state, Some(c));
            match self.transitions.get(&state_transition) {
                Some(new_state) => current_state = *new_state,
                None => try_general_case = true,
            };

            if try_general_case {
                let state_transition = (current_state, None);
                match self.transitions.get(&state_transition) {
                    Some(new_state) => {
                        current_state = *new_state;
                        try_general_case = false;
                    }
                    None => return false,
                }
            }
        }

        if self.accepting_states.contains(&current_state) {
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
        let dfa = DFA::new(HashMap::new(), accepting_states);

        assert!(dfa.run("".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = HashMap::new();
        transitions.insert((0, Some('0')), 1);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(!dfa.run("".to_string()));
    }

    #[test]
    fn run_on_string_length_one() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = HashMap::new();
        transitions.insert((0, Some('0')), 1);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(dfa.run("0".to_string()));
        assert!(!dfa.run("1".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        accepting_states.insert(2);
        let mut transitions = HashMap::new();
        transitions.insert((0, Some('0')), 1);
        transitions.insert((0, Some('1')), 2);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(dfa.run("1".to_string()));
    }

    #[test]
    fn transition_on_all() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = HashMap::new();
        transitions.insert((0, None), 1);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(dfa.run("0".to_string()));
    }

    #[test]
    fn transition_on_some() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = HashMap::new();
        transitions.insert((0, None), 1);
        transitions.insert((0, Some('0')), 2);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(!dfa.run("0".to_string()));

        assert!(dfa.run("1".to_string()));
    }

    #[test]
    fn use_loop_at_start() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = HashMap::new();
        transitions.insert((0, None), 0);
        transitions.insert((0, Some('1')), 1);
        let dfa = DFA::new(transitions, accepting_states);

        assert!(dfa.run("0001".to_string()));
    }
}
