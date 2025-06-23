pub fn clean_git_url(repo_url: &str) -> String {
    let s = repo_url
        .split_once("://")
        .map(|(_, without_protocol)| without_protocol)
        .unwrap_or(repo_url);

    s.strip_suffix(".git").unwrap_or(s).to_string()
}

mod tests {
    use crate::url::clean_git_url;

    #[test]
    fn test_url() {
        let https_url = "https://github.com/moss-foundation/sapic.git";

        let cleaned_url = clean_git_url(https_url);
        println!("{}", cleaned_url);
    }
}
