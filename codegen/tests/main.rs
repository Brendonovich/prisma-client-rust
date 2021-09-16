#[cfg(test)]
mod binaries_tests {
    use prisma::binaries;

    #[test]
    fn fetch_engine_test(){
        println!("BRUH");

        binaries::fetch_engine(
            "/Users/brendanallan/Documents/GitHub/prisma-client-rust/engine".to_string(),
            "query-engine".to_string(),
            "darwin".to_string()
        );
    }
}
