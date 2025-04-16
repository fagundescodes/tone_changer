pub const CHROMATIC_SCALE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Debug, Clone, PartialEq)]
pub enum ScaleType {
    Major,
    Minor,
    MinorMelodic,
    MinorHarmonic,
    MajorPentatonic,
    MinorPentatonic,
}

fn get_intervals(scale_type: &ScaleType) -> Vec<i32> {
    match scale_type {
        ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
        ScaleType::Minor => vec![0, 2, 3, 5, 7, 8, 10],
        ScaleType::MinorMelodic => vec![0, 2, 3, 5, 7, 9, 11],
        ScaleType::MinorHarmonic => vec![0, 2, 3, 5, 7, 8, 11],
        ScaleType::MajorPentatonic => vec![0, 2, 4, 7, 9],
        ScaleType::MinorPentatonic => vec![0, 3, 5, 7, 10],
    }
}

pub fn get_scale_notes(tonic: &str, scale_type: ScaleType) -> Option<Vec<&str>> {
    let tonic_index = CHROMATIC_SCALE.iter().position(|&note| note == tonic)?;
    let intervals = get_intervals(&scale_type);

    let notes: Vec<&str> = intervals
        .into_iter()
        .map(|interval| {
            let note_index = (tonic_index as i32 + interval) % 12;
            let note_index = if note_index >= 0 {
                note_index
            } else {
                note_index + 12
            };
            CHROMATIC_SCALE[note_index as usize]
        })
        .collect();

    Some(notes)
}

// pub struct Scale {}

// (T-T-ST-T-T-T-ST)
