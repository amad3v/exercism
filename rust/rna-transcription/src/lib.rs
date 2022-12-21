const DNA: [char; 4] = ['A', 'C', 'G', 'T'];
const RNA: [char; 4] = ['A', 'C', 'G', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: Vec<char>,
}

macro_rules! new {
    ($r:expr) => {
        pub fn new(input: &str) -> Result<Self, usize> {
            let mut seq = vec![];

            for (idx, c) in input.chars().enumerate() {
                if !$r.contains(&c) {
                    return Err(idx);
                }

                seq.push(c);
            }

            Ok(Self { seq })
        }
    };
}

impl Dna {
    new!(DNA);

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: self
                .seq
                .iter()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    &v => panic!("Invalid nucleotide: {}", v),
                })
                .collect(),
        }
    }
}

impl Rna {
    new!(RNA);
}
