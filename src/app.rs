use std::io::stdin;

pub fn app() {
    // let notes = [
    //     "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    // ];

    let notes = ["C", "D", "E", "F", "G", "A", "B"];

    println!("Insira o tom desejado: ");
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Erro ao ler entrada");

    let key = key.trim();

    if let Some(pos) = notes.iter().position(|&nota| nota == key) {
        let escala_maior: Vec<&str> = notes
            .iter()
            .cycle()
            .skip(pos)
            .take(notes.len())
            .copied()
            .collect();

        println!("Notas na escala de {:?} maior: {:?}", key, escala_maior);

        let mut acordes: Vec<String> = escala_maior.iter().map(|&s| s.to_string()).collect();

        for (i, acorde) in acordes.iter_mut().enumerate() {
            match i {
                0 => acorde.push_str("7M"),
                1 => acorde.push_str("m7"),
                2 => acorde.push_str("m7"),
                3 => acorde.push_str("7M"),
                4 => acorde.push('7'),
                5 => acorde.push_str("m7"),
                _ => acorde.push('°'),
            }
        }
        println!(
            "Acordes do campo harmônico maior natural de {:?}: {:?}",
            key, acordes
        );
    } else {
        println!("Nota inválida!");
    }
    println!("Notes in the key of {:?}", key);
}
