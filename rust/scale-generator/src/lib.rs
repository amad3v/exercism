// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
const LENGTH: usize = 12;

const FLATS: [&str; LENGTH] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];
const ORD_FLATS: [&str; LENGTH] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
const SHARPS: [&str; 14] = [
    "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#", "C", "a",
];
const ORD_SHARPS: [&str; LENGTH] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

#[derive(Debug)]
pub struct Error;

pub struct Scale {
    seq: Vec<String>,
}

type RScale = Result<Scale, Error>;

trait Case {
    fn to_titlecase(&self) -> String;
}

impl Case for str {
    fn to_titlecase(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

/// without intervals
fn gen_scale(tonic: &str, ordered: &[&str]) -> RScale {
    let pos = ordered
        .iter()
        .position(|t| t == &tonic.to_titlecase().as_str())
        .unwrap();

    Ok(Scale {
        seq: (pos..=LENGTH + pos)
            .map(|i| ordered[i % LENGTH].to_string())
            .collect(),
    })
}

/// with intervals
fn gen_scale_wi(tonic: &str, intervals: &str, ordered: &[&str]) -> RScale {
    let start = tonic.to_titlecase();
    let mut pos = ordered.iter().position(|t| t == &start.as_str()).unwrap();

    Ok(Scale {
        seq: vec![start]
            .into_iter()
            .chain(intervals.chars().map(|c| {
                pos = match c {
                    'M' => pos + 2,
                    'm' => pos + 1,
                    'A' => pos + 3,
                    u => panic!("Unknown interval: {}", u),
                };
                ordered[pos % LENGTH].to_string()
            }))
            .collect(),
    })
}

fn intervals_check(tonic: &str, intervals: &str, ordered: &[&str]) -> RScale {
    match intervals.is_empty() {
        true => gen_scale(tonic, ordered),
        false => gen_scale_wi(tonic, intervals, ordered),
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> RScale {
        match SHARPS.contains(&tonic) {
            true => intervals_check(tonic, intervals, &ORD_SHARPS),
            false => match FLATS.contains(&tonic) {
                true => intervals_check(tonic, intervals, &ORD_FLATS),
                false => Err(Error),
            },
        }
    }

    pub fn chromatic(tonic: &str) -> RScale {
        Self::new(tonic, "")
    }
    pub fn enumerate(&self) -> Vec<String> {
        self.seq.to_vec()
    }
}
