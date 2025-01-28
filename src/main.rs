use std::collections::{HashSet, HashMap};

struct Automota {
    num_states: u8,
    start_state: u8,
    accepting_states: Vec<u8>,
    transitions: HashMap<(u8, char), u8>,
}

impl Automota {
    fn new(num_states: u8, start_state: u8, accepting_states: Vec<u8>, transitions: &[(u8, char, u8)]) -> Self {
        let mut new = Self { num_states, start_state, accepting_states, transitions: HashMap::new() }; 
        new.add_transitions(transitions);
        return new;
    }

    fn add_transition(&mut self, from: u8, on: char, to: u8) {
        self.transitions.insert((from, on), to);
    }

    fn add_transitions(&mut self, transitions: &[(u8, char, u8)]) {
        transitions.iter().for_each(|(from, on, to)| self.add_transition(*from, *on, *to)); 
    }

    fn verify(&self, input: &str) -> bool {
        let mut curr = self.start_state;

        let len = input.len();
        let mut chars_iter = input.chars();
        for _ in 0..len {
            let c = chars_iter.next().unwrap();

            if let Some(to) = self.transitions.get(&(curr, c)) {
                curr = *to;
            }
        }

        return self.accepting_states.contains(&curr);
    }
}

fn main() {
    let num = 5;
    let start = 0;
    let accepting = vec![4];
    let transitions = vec![(0, 'a', 1), (0, 'b', 0), 
                           (1, 'a', 2), (1, 'b', 0),
                           (2, 'a', 2), (2, 'b', 3),
                           (3, 'a', 4), (3, 'b', 0),
                           (4, 'a', 4), (4, 'b', 4)];
    let mut aut = Automota::new(num, start, accepting, &transitions);

    println!("{}", aut.verify(&"bbaabb"));



    let num = 8;
    let start = 0;
    let accepting = vec![7];
    let transitions = vec![(0, 'a', 4), (0, 'b', 1), 
                           (1, 'a', 6), (1, 'b', 1),
                           (2, 'a', 4), (2, 'b', 3),
                           (3, 'a', 7), (3, 'b', 3),
                           (4, 'a', 0), (4, 'b', 5),
                           (5, 'a', 2), (5, 'b', 5),
                           (6, 'a', 0), (6, 'b', 7),
                           (7, 'a', 3), (7, 'b', 7)];
    let mut aut = Automota::new(num, start, accepting, &transitions); 

    println!("{}", aut.verify("baba"));
    println!("{}", aut.verify("aaa"));
}
