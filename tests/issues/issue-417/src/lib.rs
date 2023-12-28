#[allow(warnings, unused)]
mod db;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _: db::item::WhereParam = db::item::path::equals(vec![0, 1, 2, 3]);
    }
}
