mod nfa;
use nfa::NFAChar;
use nfa::Transition;
use nfa::NFA;
use std::collections::HashSet;

fn check_start(expression: &String) -> (bool, Transition) {
    let mut transitions = Transition::new();
    // If first char is ^ then process differently
    if expression.chars().next() == Some('^') {
        return (true, transitions);
    } else {
        // add loop until first char is read
        transitions.insert((0, NFAChar::Else), 0);
    }

    (false, transitions)
}

fn match_character(
    c: char,
    curr_state: &mut u32,
    transitions: &mut Transition,
    escape: &mut bool,
    last_char: &mut Option<char>,
    end: &mut bool,
) {
    if !*escape {
        match c {
            '.' => {
                // wildcard
                transitions.insert((*curr_state - 1, NFAChar::Else), *curr_state);
            }
            '\\' => {
                // escape
                *escape = true;
                *curr_state -= 1;
            }
            '$' => {
                // end
                *end = true;
                transitions.insert((*curr_state - 1, NFAChar::If(c)), *curr_state);
            }
            '?' => {
                // zero or one
                *curr_state -= 1;
                transitions.insert((*curr_state - 1, NFAChar::Epsilon), *curr_state);
            }
            '*' => {
                // zero or more
                match last_char {
                    Some(c) => {
                        *curr_state -= 1;
                        transitions.remove(&(*curr_state - 1, NFAChar::If(*c)));
                        transitions.insert((*curr_state - 1, NFAChar::Epsilon), *curr_state);
                        transitions.insert((*curr_state, NFAChar::If(*c)), *curr_state);
                    }
                    None => {
                        // normal character
                        transitions.insert((*curr_state - 1, NFAChar::If(c)), *curr_state);
                        *last_char = Some(c);
                    }
                };
            }
            '+' => {
                // one or more
                match last_char {
                    Some(c) => {
                        *curr_state -= 1;
                        transitions.insert((*curr_state, NFAChar::If(*c)), *curr_state);
                    }
                    None => {
                        // normal character
                        transitions.insert((*curr_state - 1, NFAChar::If(c)), *curr_state);
                        *last_char = Some(c);
                    }
                };
            }
            _ => {
                // normal character
                transitions.insert((*curr_state - 1, NFAChar::If(c)), *curr_state);
                *last_char = Some(c);
            }
        }
    } else {
        // insert escaped character
        transitions.insert((*curr_state - 1, NFAChar::If(c)), *curr_state);
        *last_char = Some(c);
        *escape = false;
    }
}

fn check_end(transitions: &mut Transition, curr_state: &mut u32, end: &bool) {
    // If last char is $ then remove last transition
    if *end {
        *curr_state -= 1;
        transitions.remove(&(*curr_state, NFAChar::If('$')));
    } else {
        // add loop after end of expression is read
        transitions.insert((*curr_state, NFAChar::Else), *curr_state);
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
    let mut last_char = None;
    for c in expression.chars() {
        current_state += 1;
        *end = false;

        // If ^ was at the start then don't read it
        if *start {
            current_state -= 1;
            *start = false;
            continue;
        }

        match_character(
            c,
            &mut current_state,
            transitions,
            &mut escape,
            &mut last_char,
            end,
        );
    }

    current_state
}

/// Compiles regular expression into NFA.
/// Can be reused for multiple runs.
pub fn compile(expression: String) -> NFA {
    let (mut start, mut transitions) = check_start(&expression);
    let mut end = false;

    let mut current_state =
        iterate_through_expression(expression, &mut transitions, &mut start, &mut end);

    check_end(&mut transitions, &mut current_state, &mut end);

    // add accepting state
    let mut accepting_states = HashSet::new();
    accepting_states.insert(current_state);

    NFA::new(transitions, accepting_states)
}

pub fn compile_and_run(expression: String, input: String) -> bool {
    let dfa = compile(expression);
    dfa.run(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_empty_regex() {
        assert!(compile_and_run("".to_string(), "".to_string()));
        assert!(compile_and_run("".to_string(), "afklgjlakj".to_string()))
    }

    #[test]
    fn match_string() {
        assert!(!compile_and_run("abcd".to_string(), "".to_string()));
        assert!(compile_and_run("abcd".to_string(), "abcd".to_string()))
    }

    #[test]
    fn match_string_not_at_start() {
        assert!(compile_and_run("abcd".to_string(), "xxxabcd".to_string()));
    }

    #[test]
    fn match_wildcard() {
        assert!(compile_and_run("a.cd".to_string(), "abcd".to_string()));
        assert!(compile_and_run("a.cd".to_string(), "axcd".to_string()));
    }

    #[test]
    fn escape() {
        assert!(compile_and_run("a\\.cd".to_string(), "a.cd".to_string()));
        assert!(compile_and_run("a\\\\".to_string(), "a\\".to_string()));
        assert!(!compile_and_run("a\\.cd".to_string(), "axcd".to_string()));
    }

    #[test]
    fn check_for_start() {
        assert!(compile_and_run("^abcd".to_string(), "abcd".to_string()));
        assert!(!compile_and_run("^abcd".to_string(), "xxxabcd".to_string()));
    }

    #[test]
    fn check_for_end() {
        assert!(compile_and_run("abcd$".to_string(), "abcd".to_string()));
        assert!(!compile_and_run("abcd$".to_string(), "abcdxxx".to_string()));
    }

    #[test]
    fn zero_or_one() {
        assert!(compile_and_run("a?b?c?d".to_string(), "acd".to_string()));
        assert!(compile_and_run("a?b?c?d".to_string(), "ad".to_string()));
        assert!(compile_and_run("a?b?c?d".to_string(), "d".to_string()));
    }

    #[test]
    fn zero_or_more() {
        assert!(compile_and_run("a*b*c*d".to_string(), "abcd".to_string()));
        assert!(compile_and_run("a*b*c*d".to_string(), "aacccd".to_string()));
        assert!(compile_and_run("a*b*c*d".to_string(), "d".to_string()));
    }

    #[test]
    fn one_or_more() {
        assert!(compile_and_run("a+b+c+d".to_string(), "abcd".to_string()));
        assert!(compile_and_run(
            "a+b+c+d".to_string(),
            "aaabbccccd".to_string()
        ));
        assert!(!compile_and_run(
            "a+b+c+d".to_string(),
            "aaccccd".to_string()
        ));
    }
}
