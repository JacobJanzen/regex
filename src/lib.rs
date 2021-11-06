mod dfa;
use dfa::Transition;
use dfa::DFA;
use std::collections::HashSet;

fn check_start(expression: &String) -> (bool, Transition) {
    let mut transitions = Transition::new();
    // If first char is ^ then process differently
    if expression.chars().next() == Some('^') {
        return (true, transitions);
    } else {
        // add loop until first char is read
        transitions.insert((0, None), 0);
    }

    (false, transitions)
}

fn match_character(
    c: char,
    curr_state: &mut u32,
    transitions: &mut Transition,
    escape: &mut bool,
    end: &mut bool,
) {
    if !*escape {
        match c {
            '.' => {
                // wildcard
                transitions.insert((*curr_state - 1, None), *curr_state);
            }
            '\\' => {
                // escape
                *escape = true;
                *curr_state -= 1;
            }
            '$' => {
                // end
                *end = true;
                transitions.insert((*curr_state - 1, Some(c)), *curr_state);
            }
            _ => {
                // normal character
                transitions.insert((*curr_state - 1, Some(c)), *curr_state);
            }
        }
    } else {
        // insert escaped character
        transitions.insert((*curr_state - 1, Some(c)), *curr_state);
        *escape = false;
    }
}

fn check_end(transitions: &mut Transition, curr_state: &mut u32, end: &bool) {
    // If last char is $ then remove last transition
    if *end {
        println!("{}", *curr_state);
        *curr_state -= 1;
        transitions.remove(&(*curr_state, Some('$')));
    } else {
        // add loop after end of expression is read
        transitions.insert((*curr_state, None), *curr_state);
    }
}

fn iterate_through_expression(
    expression: String,
    transitions: &mut Transition,
    start: &mut bool,
    end: &mut bool,
) -> u32 {
    let mut current_state = 0;
    let mut escape = false;
    for c in expression.chars() {
        current_state += 1;
        *end = false;

        // If ^ was at the start then don't read it
        if *start {
            current_state -= 1;
            *start = false;
            continue;
        }

        match_character(c, &mut current_state, transitions, &mut escape, end);
    }

    current_state
}

fn compile(expression: String) -> DFA {
    let (mut start, mut transitions) = check_start(&expression);
    let mut end = false;

    let mut current_state =
        iterate_through_expression(expression, &mut transitions, &mut start, &mut end);

    check_end(&mut transitions, &mut current_state, &mut end);

    // add accepting state
    let mut accepting_states = HashSet::new();
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
