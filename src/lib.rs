pub fn convert_to_number(scale: &str) -> usize {
    if scale == "1" {
        1
    } else if scale == "2" {
        2
    } else if scale == "3" {
        3
    } else if scale == "4" {
        4
    } else if scale == "5" {
        5
    } else if scale == "6" {
        6
    } else if scale == "7" {
        7
    } else {
        0
    }
}

pub fn scale_name(scale: usize, note: &str, char_acc: char) {
    let char_cleaned: char;
    if char_acc == '\n' {
        char_cleaned = ' '
    } else {
        char_cleaned = char_acc
    }

    let mut scale = scale;
    let mut note_name = "x";

    if *note == *"C" {
        note_name = "C"
    } else if *note == *"D" {
        note_name = "D"
    } else if *note == *"E" {
        note_name = "E"
    } else if *note == *"F" {
        note_name = "F"
    } else if *note == *"G" {
        note_name = "G"
    } else if *note == *"A" {
        note_name = "A"
    } else if *note == *"H" {
        note_name = "H"
    } else {
        scale = 0
    };
    match scale {
        1 => print!("{}{} Locrian scale: ", note_name, char_cleaned),
        2 => print!("{}{} Minor (Aeolian) scale: ", note_name, char_cleaned),
        3 => print!("{}{} Mixolydian scale: ", note_name, char_cleaned),
        4 => print!("{}{} Lydian scale: ", note_name, char_cleaned),
        5 => print!("{}{} Phrygian scale: ", note_name, char_cleaned),
        6 => print!("{}{} Dorian scale: ", note_name, char_cleaned),
        7 => print!("{}{} Major (Ionian) scale: ", note_name, char_cleaned),
        0 => print!("Invalid input. Please pick scale number and root note from the list"),
        _ => (),
    }
}

fn swapper(note_acc_to_swap: &str) -> &str {
    match note_acc_to_swap {
        "Cb" => "H",
        "H#" => "C",
        "Fb" => "E",
        "E#" => "F",
        _ => note_acc_to_swap,
    }
}

fn changer<'a>(temp: &std::vec::Vec<&'a str>) -> Vec<&'a str> {
    let mut index = 0;
    let mut new_vec = temp.clone();
    for _i in temp {
        if new_vec[index].contains('#') {
            new_vec[index] = match new_vec[index] {
                "C#" => "Db",
                "D#" => "Eb",
                "E#" => "F",
                "F#" => "Gb",
                "G#" => "Ab",
                "A#" => "Hb",
                "H#" => "Cb",
                &_ => "",
            }
        } else if new_vec[index].contains('b') {
            new_vec[index] = match new_vec[index] {
                "Cb" => "H",
                "Db" => "C#",
                "Eb" => "D#",
                "Fb" => "E",
                "Gb" => "F#",
                "Ab" => "G#",
                "Hb" => "A#",
                &_ => "",
            }
        }

        index = index + 1
    }
    return new_vec;
}

fn sharp_flats(temp: &std::vec::Vec<&str>) -> String {
    let mut index = 0;
    let mut new_vec = temp.clone();
    for _i in temp {
        let a;
        let b;

        if index + 1 == 7 {
            a = new_vec[0].to_string();
            b = new_vec[6].to_string();
        } else {
            a = new_vec[index].to_string();
            b = new_vec[index + 1].to_string();
        }
        if a[..1].contains(&b[..1]) {
            new_vec = changer(&new_vec);
        }

        index = index + 1
    }
    let final_string = format!(
        "{}, {}, {}, {}, {}, {}, {}",
        new_vec[0], new_vec[1], new_vec[2], new_vec[3], new_vec[4], new_vec[5], new_vec[6]
    );

    final_string
}

pub fn constructor<'a>(number_of_scale: usize, char_acc: char, scale: &str) -> String {
    let current_scale;
    let mut note_acc = scale[0..1].to_uppercase().to_owned();
    if char_acc == 'b' || char_acc == '#' {
        note_acc.push(char_acc)
    }

    println!("note_acc: {}", note_acc);

    note_acc = swapper(&note_acc).to_string();

    if char_acc == 'b' {
        current_scale = [
            "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Hb", "H",
        ]
    } else {
        current_scale = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "H",
        ]
    }

    let index: usize = current_scale.iter().position(|&x| x == note_acc).unwrap();
    let first_part = &current_scale[index..=11];
    let second_part = &current_scale[0..index];
    let mut temp = vec![];
    println!("temp: {:?}", temp);

    let remove_this_notes = match number_of_scale {
        1 => [11, 9, 7, 4, 2], // Locrian
        2 => [11, 9, 6, 4, 1], // Minor
        3 => [11, 8, 6, 3, 1], // Mixolydian
        4 => [10, 8, 5, 3, 1], // Lydian
        5 => [11, 9, 6, 4, 2], // Phrygian
        6 => [11, 8, 6, 4, 1], // Dorian
        7 => [10, 8, 6, 3, 1], // Major
        _ => [0, 0, 0, 0, 0],
    };

    temp.extend(first_part.to_vec().into_iter());
    temp.extend(second_part.to_vec().into_iter());

    let mut index = 0;

    while index < 5 {
        temp.remove(remove_this_notes[index]);
        index = index + 1;
    }

    let mut result = "".to_owned();
    if temp.contains(&"C") && temp.contains(&"C#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"D") && temp.contains(&"D#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"E") && temp.contains(&"E#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"F") && temp.contains(&"F#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"G") && temp.contains(&"G#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"A") && temp.contains(&"A#") {
        result = sharp_flats(&temp)
    } else if temp.contains(&"H") && temp.contains(&"H#") {
        result = sharp_flats(&temp)
    } else {
        result = format!(
            "{}, {}, {}, {}, {}, {}, {}",
            temp[0], temp[1], temp[2], temp[3], temp[4], temp[5], temp[6]
        );
    }

    println!("{}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c() {
        assert_eq!("C, Db, Eb, F, Gb, Ab, Hb", constructor(1, ' ', "C"));
        assert_eq!("C, D, Eb, F, G, Ab, Hb", constructor(2, ' ', "C"));
        assert_eq!("C, D, E, F, G, A, Hb", constructor(3, ' ', "C"));
        assert_eq!("C, D, E, F#, G, A, H", constructor(4, ' ', "C"));
        assert_eq!("C, Db, Eb, F, G, Ab, Hb", constructor(5, ' ', "C"));
        assert_eq!("C, D, Eb, F, G, A, Hb", constructor(6, ' ', "C"));
        assert_eq!("C, D, E, F, G, A, H", constructor(7, ' ', "C"));
    }
    #[test]
    fn c_sharp_d_flat() {
        assert_eq!("C#, D, E, F#, G, A, H", constructor(1, '#', "C"));
        assert_eq!("C#, D#, E, F#, G#, A, H", constructor(2, '#', "C"));
        assert_eq!("C#, D#, E#, F#, G#, A#, H", constructor(3, '#', "C"));
        assert_eq!("Db, Eb, F, G, Ab, Hb, C", constructor(4, 'b', "D"));
        assert_eq!("C#, D, E, F#, G#, A, H", constructor(5, '#', "C"));
        assert_eq!("C#, D#, E, F#, G#, A#, H", constructor(6, '#', "C"));
        assert_eq!("Db, Eb, F, Gb, Ab, Hb, C", constructor(7, 'b', "D"));
    }
    #[test]
    fn d() {
        assert_eq!("D, Eb, F, G, Ab, Hb, C", constructor(1, ' ', "D"));
        assert_eq!("D, E, F, G, A, Hb, C", constructor(2, ' ', "D"));
        assert_eq!("D, E, F#, G, A, H, C", constructor(3, ' ', "D"));
        assert_eq!("D, E, F#, G#, A, H, C#", constructor(4, ' ', "D"));
        assert_eq!("D, Eb, F, G, A, Hb, C", constructor(5, ' ', "D"));
        assert_eq!("D, E, F, G, A, H, C", constructor(6, ' ', "D"));
        assert_eq!("D, E, F#, G, A, H, C#", constructor(7, ' ', "D"));
    }
    #[test]
    fn d_sharp_e_flat() {
        assert_eq!("D#, E, F#, G#, A, H, C#", constructor(1, ' ', "D"));
        assert_eq!("Eb, F, Gb, Ab, Hb, Cb, Db", constructor(2, ' ', "D"));
        assert_eq!("Eb, F, G, Ab, Hb, C, Db", constructor(3, ' ', "D"));
        assert_eq!("Eb, F, G, A, Hb, C, D", constructor(4, ' ', "D"));
        assert_eq!("D#, E, F#, G#, A#, H, C#", constructor(5, ' ', "D"));
        assert_eq!("Eb, F, Gb, Ab, Hb, C, Db", constructor(6, ' ', "D"));
        assert_eq!("Eb, F, G, Ab, Hb, C, D", constructor(7, ' ', "D"));
    }
}
