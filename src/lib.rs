mod dfa;
use dfa::DFA;
use std::collections::HashMap;
use std::collections::HashSet;

fn compile(expression: String) -> DFA {
    let mut transitions = HashMap::new();
    let mut accepting_states = HashSet::new();

    let mut current_state = 0;
    let mut escape = false;
    let mut start = false;
    let mut end = false;

    // If first char is ^ then process differently
    if expression.chars().next() == Some('^') {
        start = true;
    } else {
        // add loop until first char is read
        transitions.insert((0, None), 0);
    }
    for c in expression.chars() {
        current_state += 1;
        end = false;

        // If ^ was at the start then don't read it
        if start {
            current_state -= 1;
            start = false;
            continue;
        }

        // If this character was not escaped, process normally
        if !escape {
            match c {
                '.' => {
                    // wildcard
                    transitions.insert((current_state - 1, None), current_state);
                }
                '\\' => {
                    // escape
                    escape = true;
                    current_state -= 1;
                }
                '$' => {
                    // end
                    end = true;
                    transitions.insert((current_state - 1, Some(c)), current_state);
                }
                _ => {
                    // normal character
                    transitions.insert((current_state - 1, Some(c)), current_state);
                }
            };
        } else {
            // insert escaped character
            transitions.insert((current_state - 1, Some(c)), current_state);
            escape = false;
        }
    }

    if end {
        current_state -= 1;
        transitions.remove(&(current_state, Some('$')));
    } else {
        // add loop after end of expression read
        transitions.insert((current_state, None), current_state);
    }
    accepting_states.insert(current_state);

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
        assert!(run("a\\\\".to_string(), "a\\".to_string()));
        assert!(!run("a\\.cd".to_string(), "axcd".to_string()));
    }

    #[test]
    fn check_for_start() {
        assert!(run("^abcd".to_string(), "abcd".to_string()));
        assert!(!run("^abcd".to_string(), "xxxabcd".to_string()));
    }

    #[test]
    fn check_for_end() {
        assert!(run("abcd$".to_string(), "abcd".to_string()));
        assert!(!run("abcd$".to_string(), "abcdxxx".to_string()));
    }
}
