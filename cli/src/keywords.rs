use convert_case::{Case, Casing};

pub const KEYWORDS: &'static [&'static str] = &[
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "macro_rules",
    "union",
    "'static",
    "dyn",
];

pub fn is_reserved_name(name: &str) -> bool {
    KEYWORDS.contains(&name)
}

pub trait UnderscoreSafeCasing {
    fn to_case_safe(&self, case: Case) -> String;
}

impl<T: AsRef<str>> UnderscoreSafeCasing for T
where
    String: PartialEq<T>,
{
    fn to_case_safe(&self, case: Case) -> String {
        let this = self.as_ref();

        match &this[this.len() - 1..] {
            "_" if this.len() >= 2 => (&this[0..this.len() - 1]).to_case(case) + "_",
            _ => this.to_case(case),
        }
    }
}
