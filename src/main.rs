use std::io::stdin;

fn main() {
    let vetor_de_toms = ["C", "D", "E", "F", "G", "A", "B"];

    println!("Insira o tom desejado: ");
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Erro ao ler entrada");

    let key = key.trim();

    if let Some(pos) = vetor_de_toms.iter().position(|&nota| nota == key) {
        let escala_maior: Vec<&str> = vetor_de_toms
            .iter()
            .cycle()
            .skip(pos)
            .take(vetor_de_toms.len())
            .copied()
            .collect();

        println!("Notas na escala de {:?} maior: {:?}", key, escala_maior);

        let mut acordes: Vec<String> = escala_maior.iter().map(|&s| s.to_string()).collect();

        for (i, acordes) in acordes.iter_mut().enumerate() {
            match i {
                0 => acordes.push_str("7M"),
                1 => acordes.push_str("m7"),
                2 => acordes.push_str("m7"),
                3 => acordes.push_str("7M"),
                4 => acordes.push('7'),
                5 => acordes.push_str("m7"),
                _ => acordes.push('°'),
            }
        }
        println!("Acordes do campo harmônico maior natural de {:?}: {:?}",key, acordes);
    } else {
        println!("Nota inválida!");
    }
    println!("Notes in the key of {:?}", key);
}
