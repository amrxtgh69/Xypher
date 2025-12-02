fn get_seeds_urls() -> Vec<String> {
    std::fs::read_to_string("seeds.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

