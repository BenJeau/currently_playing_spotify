pub fn env(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}