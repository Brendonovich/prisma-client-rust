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

        let starts_with_underscore = this.starts_with('_');
        let ends_with_underscore = this.ends_with('_') && this.len() >= 2;

        let body = &this[0..this.len() - if ends_with_underscore { 1 } else { 0 }];

        let cased = format!(
            "{}{}{}",
            if starts_with_underscore { "_" } else { "" },
            convert_case::Casing::to_case(&body, case),
            if ends_with_underscore { "_" } else { "" }
        );

        if !raw && KEYWORDS.iter().any(|k| k == &cased) {
            format!("r#{cased}")
        } else {
            cased
        }
    }
}
