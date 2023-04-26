// use std::{fs::File, io::Write};
// use std::path::Path;
// // use byteorder::{BigEndian, WriteBytesExt};

// fn main() {
//     let path = Path::new("/tmp/test1.db");

//     let mut file = match File::create(&path) {
//         Err(why) => panic!("Unable to create {}: {}", path.display(), why),
//         Ok(file) => file,
//     };

//     let page_size: usize = 4096;

//     for _loop_index in 0 .. 10000000 {
//         let mut buf : Vec<u8> = vec![0u8; page_size];
//         for _j in 0 .. 1024 { 
//             buf[(_j*4) .. (_j+1)*4].copy_from_slice(&[_j as u8, (_j+1) as u8, (_j+2) as u8, (_j+3) as u8]);
//         }
//         file.write(&buf).unwrap();
//     }

//     println!("Database created");
// }

use std::collections::{HashMap,HashSet};

struct StateMachine {
    _states: HashSet<String>,
    transitions: HashMap<String, HashSet<String>>,
}

impl StateMachine {
    fn create (states: Vec<&str>, state_transitions: Vec<(&str, &str)>) -> Result<StateMachine, String> {
        let states = HashSet::<String>::from_iter(states.into_iter().map(|state| state.to_string()));
        let mut transitions: HashMap<String, HashSet<String>> = HashMap::with_capacity(states.len());
        
        for (from, to) in state_transitions.into_iter() {
            if !states.contains(from) {
                return Err(format!("from-state does not exist in state set: {} -> {}", from, to));
            }
            if !states.contains(to) {
                return Err(format!("to-state does not exist in state set: {} -> {}", from, to));
            }
            if !transitions.contains_key(from) {
                transitions.insert(String::from(from), HashSet::<String>::new());
            }
            transitions.get_mut(from).and_then(|result| Some(result.insert(to.to_string())));
        }
       
        Ok(StateMachine {
            _states: states,
            transitions,
        })
    }

    fn is_valid_state_transition(&self, from_state: &str, to_state: &str) -> bool {
        self.transitions.get(from_state).and_then(|targets| Some(targets.contains(to_state))).is_some()
    }
}

fn main () {
    let state_machine = StateMachine::create(
        Vec::from(["init", "start", "finished"]), 
        vec![("init", "start"), ("start", "finished1")]
    ).unwrap();

    println!("Valid State Transition: {}", state_machine.is_valid_state_transition("init", "start"));
}
