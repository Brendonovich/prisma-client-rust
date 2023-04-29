pub use convert_case::Case;

use crate::keywords::KEYWORDS;

pub trait Casing {
    fn to_case(&self, case: Case, raw: bool) -> String;
}

impl<T: AsRef<str> + convert_case::Casing<T>> Casing for T
where
    String: PartialEq<T>,
{
    fn to_case(&self, case: Case, raw: bool) -> String {
        let this = self.as_ref();

        let cased = match &this[this.len() - 1..] {
            "_" if this.len() >= 2 => (&this[0..this.len() - 1]).to_case(case, raw) + "_",
            _ => convert_case::Casing::to_case(&this, case),
        };

        if !raw && KEYWORDS.iter().any(|k| k == &cased) {
            format!("r#{cased}")
        } else {
            cased
        }
    }
}
