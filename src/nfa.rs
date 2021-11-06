use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub enum NFAChar {
    Epsilon,
    If(char),
    Else,
}

pub type Transition = HashMap<(u32, NFAChar), u32>;

/// Represents a deterministic finite automaton
/// # Q: States
/// We don't need to explicitly store this.
/// # q0: Start state
/// Always defined as 0.
/// # Σ: Alphabet
/// Defined as being all unicode characters.
pub struct NFA {
    /// # δ: Transition Function
    /// Specifies a state and its input to get output state.
    /// Uses `None` option to represent unconditional transition;
    /// if `Some(x)` was also specified, it will look at that first.
    transitions: Transition,
    /// # F: Accepting states
    /// Set of states that the NFA will accept in.
    accepting_states: HashSet<u32>,
}

impl NFA {
    /// Create a new NFA
    ///
    /// Give a transition function in the form of a `Transition`.
    /// Use `None` as the input character to add transition on everything after
    /// the `Some(x)` transitions.
    ///
    /// Give the set of accepting states in the form of a `HashSet`.
    /// Refer to states by an integer.
    pub fn new(transitions: Transition, accepting_states: HashSet<u32>) -> NFA {
        NFA {
            transitions,
            accepting_states,
        }
    }

    /// Recursively find all states connected through epsilon transitions
    /// to given state: curr.
    fn follow_epilon_transition(&self, current_states: &mut HashSet<u32>, curr: u32) {
        let state_transition = (curr, NFAChar::Epsilon);
        match self.transitions.get(&state_transition) {
            Some(new_state) => {
                current_states.insert(*new_state);
                self.follow_epilon_transition(current_states, *new_state);
            }
            None => (),
        };
    }

    /// Run the NFA
    ///
    /// give a unicode string as an input.
    /// return true if the NFA accepts.
    /// return false if the NFA rejects.
    pub fn run(&self, input: String) -> bool {
        let mut current_states = HashSet::new();
        let mut next_states = HashSet::new();
        current_states.insert(0);
        self.follow_epilon_transition(&mut current_states, 0);

        for c in input.chars() {
            let mut successes = 0;
            let mut check_else = true;

            // TODO: use multithreading
            for state in &current_states {
                let state_transition = (*state, NFAChar::If(c));
                match self.transitions.get(&state_transition) {
                    Some(new_state) => {
                        successes += 1;
                        next_states.insert(*new_state);
                        self.follow_epilon_transition(&mut next_states, *new_state);
                        check_else = false;
                    }
                    None => (),
                };

                if check_else {
                    let state_transition = (*state, NFAChar::Else);
                    match self.transitions.get(&state_transition) {
                        Some(new_state) => {
                            successes += 1;
                            next_states.insert(*new_state);
                            self.follow_epilon_transition(&mut next_states, *new_state);
                        }
                        None => (),
                    };
                }
            }

            // exit if there were no valid transitions
            if successes == 0 {
                return false;
            }

            // move the next states into the current states
            // TODO: use pointers instead of copying to reduce time complexity
            current_states = HashSet::new();
            for state in &next_states {
                current_states.insert(*state);
            }
        }

        for s in &current_states {
            if self.accepting_states.contains(s) {
                return true;
            }
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
        let nfa = NFA::new(Transition::new(), accepting_states);

        assert!(nfa.run("".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::If('0')), 1);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(!nfa.run("".to_string()));
    }

    #[test]
    fn run_on_string_length_one() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::If('0')), 1);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("0".to_string()));
        assert!(!nfa.run("1".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        accepting_states.insert(2);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::If('0')), 1);
        transitions.insert((0, NFAChar::If('1')), 2);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("1".to_string()));
    }

    #[test]
    fn transition_on_all() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::Else), 1);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("0".to_string()));
    }

    #[test]
    fn transition_on_some() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::Else), 1);
        transitions.insert((0, NFAChar::If('0')), 2);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(!nfa.run("0".to_string()));

        assert!(nfa.run("1".to_string()));
    }

    #[test]
    fn use_loop_at_start() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::Else), 0);
        transitions.insert((0, NFAChar::If('1')), 1);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("0001".to_string()));
    }

    #[test]
    fn epsilon_transition() {
        let mut accepting_states = HashSet::new();
        accepting_states.insert(1);
        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::Epsilon), 1);
        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("".to_string()));

        let mut accepting_states = HashSet::new();
        accepting_states.insert(4);

        let mut transitions = Transition::new();
        transitions.insert((0, NFAChar::If('a')), 1);
        transitions.insert((0, NFAChar::Epsilon), 1);
        transitions.insert((1, NFAChar::If('b')), 2);
        transitions.insert((1, NFAChar::Epsilon), 2);
        transitions.insert((2, NFAChar::If('c')), 3);
        transitions.insert((2, NFAChar::Epsilon), 3);
        transitions.insert((3, NFAChar::If('d')), 4);

        let nfa = NFA::new(transitions, accepting_states);

        assert!(nfa.run("acd".to_string()));
    }
}
