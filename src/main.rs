#![allow(unused)]
// scale_finder_v2

// Write new tests
// Fix some scales

use std::io;

mod lib;

fn main() {
    println!(
"Coose a scale number:
1 - Locrian
2 - Minor (Aeolian)
3 - Mixolydian
4 - Lydian
5 - Phrygian
6 - Dorian
7 - Major (Ionian)
---------------------
--> Choose a root note:
C D E F G A H
---------------------
--> Choose sharp or flat:
# b
---------------------"
    );

    let mut scale = String::new();
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");

    let number_of_scale = lib::convert_to_number(&scale[..1]);
    let note_char = &scale.to_uppercase()[1..2];
    let char_acc: char = scale[2..=2].parse().unwrap();

    if &scale.len() == &3_usize || &scale.len() == &4_usize {
        lib::scale_name(number_of_scale, note_char, char_acc);
    } else if &scale.len() < &3_usize {
        println!("Less than 2 symbols. Please pick scale number and root note from the list")
    } else if &scale.len() > &4_usize {
        println!("More than 3 symbols. Please pick scale number and root note from the list")
    };

    lib::constructor(number_of_scale, char_acc, &scale[1..]);
}
