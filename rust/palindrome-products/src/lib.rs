/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        (is_palindrome(value)).then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut r = 0;
    let mut p = n;

    while p > 0 {
        r = (10 * r) + (p % 10);
        p /= 10;
    }

    r == n
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pals = vec![];
    let mut latest = 0;

    for (counter, val1) in (min..=max).enumerate() {
        for val2 in (min + counter as u64)..=max {
            let value = val1 * val2;

            if value > latest && is_palindrome(value) {
                latest = value;
                pals.push(value)
            }
        }
    }

    match !pals.is_empty() {
        true => Some((
            Palindrome(*pals.iter().min().unwrap()),
            Palindrome(*pals.iter().max().unwrap()),
        )),
        false => None,
    }
}
