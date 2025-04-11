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

fn get_scale_notes() {

    let tonic = CHROMATIC_SCALE;

}

pub struct Scale {}

// (T-T-ST-T-T-T-ST)
