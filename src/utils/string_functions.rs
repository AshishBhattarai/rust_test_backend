pub fn get_evn_or_panic(var: &str) -> String {
    std::env::var(var).expect(&format!("{} {}", var, "must be set."))
}
