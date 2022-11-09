pub use convert_case::Case;

use crate::keywords::KEYWORDS;

pub trait Casing {
    fn to_case(&self, case: Case) -> String;
}

impl<T: AsRef<str> + convert_case::Casing<T>> Casing for T
where
    String: PartialEq<T>,
{
    fn to_case(&self, case: Case) -> String {
        let this = self.as_ref();
        let should_raw = KEYWORDS.contains(&this);

        let cased = match &this[this.len() - 1..] {
            "_" if this.len() >= 2 => (&this[0..this.len() - 1]).to_case(case) + "_",
            _ => convert_case::Casing::to_case(&this, case),
        };

        if should_raw {
            format!("r#{cased}")
        } else {
            cased
        }
    }
}
