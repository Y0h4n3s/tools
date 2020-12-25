pub mod matchers {
    pub fn domain_matches_by_hostname(hotname: &str, root_domain: &str) -> bool {
        let re = format!(".?{}.?", root_domain);
        let matcher = regex::Regex::new(&re).unwrap();
        matcher.is_match(hotname)
    }
}