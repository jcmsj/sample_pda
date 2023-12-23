use std::collections::{BTreeSet, VecDeque};

pub struct Transition {
    pub input: char,
    pub top: char,
    pub next_state: String,
    pub give: Vec<char>,
}

impl From<(char, char, String, Vec<char>)> for Transition {
    fn from((input, top, next_state, give): (char, char, String, Vec<char>)) -> Self {
        Self {
            input,
            top,
            next_state,
            give,
        }
    }
}
pub struct PushdownAutomata<'a> {
    // PDA definition
    pub input_alphabet: BTreeSet<&'a str>, 
    pub stack_alphabet: BTreeSet<&'a str>,
    pub start_stack_symbol: &'a char,
    pub transitions: Vec<(String,Transition)>,
    pub start_state: char,
    pub end_states : BTreeSet<&'a str>,
    // For string checking
    stack: Vec<
    &'a char>,
}

impl <'a> PushdownAutomata<'a>  {
    pub fn test(&'a mut self, input: &str) -> bool {
        // TODO: pda validation using the alphabets
        self.stack = vec![&self.start_stack_symbol];
        let mut input_deque = input.chars().collect::<VecDeque<char>>();
        let mut state:&String = &self.start_state.to_string();
        let mut built = String::from("");
        while let (Some(top),Some(char)) = (self.stack.pop(),input_deque.pop_front())  {
            for (in_state, t) in &self.transitions {
                let matches_state = state == in_state;
                let matches_input = char == t.input;
                let epsilon = t.input == 'e';
                let matches_top = *top == t.top;
                if matches_state && (matches_input || epsilon) && matches_top {
                    built.push(char);
                    // check if the built string fits as the prefix of the input
                    if built != input[0..built.len()] {
                        built.pop();
                        continue; // try next transition
                    }
                    for g in &t.give {
                        self.stack.push(g)
                    }
                    state = &t.next_state;
                    // if epsilon, must reuse the char
                    if epsilon {
                        input_deque.push_front(char);
                    }
                    break; // proceed to next char in the input
                }
           }
        }
        
        return self.stack.is_empty() 
            && self.end_states.contains(state.as_str()) 
            && built == input;
    }
}

#[cfg(test)]
mod tests {
    use crate::pda::Transition;
    use super::PushdownAutomata;
    // Based on https://en.wikipedia.org/wiki/Pushdown_automaton#Explanation
    fn sample() -> PushdownAutomata<'static> {
        return PushdownAutomata {
            input_alphabet: vec!["0", "1", "e"].into_iter().collect(),
            stack_alphabet: vec!["A", "Z"].into_iter().collect(),
            start_stack_symbol: &'Z',
            transitions: vec![
                // (p,0,Z,p,AZ)
                (String::from("p"), 
                    Transition::from((
                        '0', 
                        'Z', 
                        String::from("p"), 
                        vec!['A', 
                        'Z'])
                    )
                ),
                // (p,0,A,p,AA)
                (String::from("p"), 
                    Transition::from((
                        '0', 
                        'A', 
                        String::from("p"), 
                        vec!['A', 
                        'A'])
                    )
                ),
                // (p,\epsilon, Z,q,Z)
                (String::from("p"), 
                Transition::from((
                    'e', 
                    'Z', 
                    String::from("q"), 
                    vec!['Z'])
                )),
                // (p,\epsilon ,A,q,A)
                (String::from("p"),
                    Transition::from((
                        'e', 
                        'A', 
                        String::from("q"), 
                        vec!['A'])
                    )
                ),
                // (q,1,A,q,\epsilon )
                (String::from("q"),
                    Transition::from((
                        '1', 
                        'A', 
                        String::from("q"), 
                        vec![])
                    )
                ),
                // (q,\epsilon ,Z,r,Z)
                (String::from("q"),
                    Transition::from((
                        'e', 
                        'Z', 
                        String::from("r"), 
                        vec!['Z'])
                    )
                ),
            ],
            start_state: 'p',
            end_states: vec!["r"].into_iter().collect(),
            stack: vec![],
        };
    }
    #[test]
    fn test_accepts_same_number_of_zeros_and_ones() {
        let mut pda = sample();
        assert_eq!(true, pda.test("0011"));
    }
    #[test]
    fn test_rejects_diff_number_of_zeros_and_ones() {
        let mut pda = sample();
        assert_eq!(false, pda.test("00111"));
    }
}
