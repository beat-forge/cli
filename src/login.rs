pub fn login(api_key: String) {
    if api_key.len() != 36 {
        panic!("Invalid API key.");
    }
    std::env::set_var("BEATFORGE_API_KEY", api_key);
}