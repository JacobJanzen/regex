mod dfa;
use dfa::DFA;
use std::collections::HashMap;
use std::collections::HashSet;

fn compile(expression: String) -> DFA {
    let mut transitions = HashMap::new();
    let mut accepting_states = HashSet::new();

    let mut current_state = 0;
    let mut escape = false;

    transitions.insert((0, None), 0);
    for c in expression.chars() {
        current_state += 1;

        if !escape {
            match c {
                '.' => {
                    transitions.insert((current_state - 1, None), current_state);
                }
                '\\' => {
                    escape = true;
                    current_state -= 1;
                }
                _ => {
                    transitions.insert((current_state - 1, Some(c)), current_state);
                }
            };
        } else {
            transitions.insert((current_state - 1, Some(c)), current_state);
            escape = false;
        }
    }
    accepting_states.insert(current_state);
    transitions.insert((current_state, None), current_state);

    DFA::new(transitions, accepting_states)
}

pub fn run(expression: String, input: String) -> bool {
    let dfa = compile(expression);
    dfa.run(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_empty_regex() {
        assert!(run("".to_string(), "".to_string()));
        assert!(run("".to_string(), "afklgjlakj".to_string()))
    }

    #[test]
    fn match_string() {
        assert!(!run("abcd".to_string(), "".to_string()));
        assert!(run("abcd".to_string(), "abcd".to_string()))
    }

    #[test]
    fn match_string_not_at_start() {
        assert!(run("abcd".to_string(), "xxxabcd".to_string()));
    }

    #[test]
    fn match_wildcard() {
        assert!(run("a.cd".to_string(), "abcd".to_string()));
        assert!(run("a.cd".to_string(), "axcd".to_string()));
    }

    #[test]
    fn escape() {
        assert!(run("a\\.cd".to_string(), "a.cd".to_string()));
        assert!(!run("a\\.cd".to_string(), "axcd".to_string()));
    }
}
