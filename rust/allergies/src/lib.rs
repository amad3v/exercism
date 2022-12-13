pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        use Allergen::*;

        match allergen {
            Eggs => self.score & 1 > 0,
            Peanuts => self.score & 2 > 0,
            Shellfish => self.score & 4 > 0,
            Strawberries => self.score & 8 > 0,
            Tomatoes => self.score & 16 > 0,
            Chocolate => self.score & 32 > 0,
            Pollen => self.score & 64 > 0,
            Cats => self.score & 128 > 0,
        }
        // self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;

        (0..8)
            .filter_map(|rank| match self.score & 2u32.pow(rank) > 0 {
                true => match rank {
                    0 => Some(Eggs),
                    1 => Some(Peanuts),
                    2 => Some(Shellfish),
                    3 => Some(Strawberries),
                    4 => Some(Tomatoes),
                    5 => Some(Chocolate),
                    6 => Some(Pollen),
                    7 => Some(Cats),
                    _ => None,
                },
                false => None,
            })
            .collect()
    }
}
