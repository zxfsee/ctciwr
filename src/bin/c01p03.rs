fn urlify(url: &'static str) -> String {
    let urlified_url = String::new();
    let placeholder = "%20";

    url.split_whitespace().fold(urlified_url, |acc, s| {
        if acc.is_empty() {
            String::from(s)
        } else {
            acc + placeholder + s
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&"Mr John Smith     "), "Mr%20John%20Smith");
    }
}

fn main() {
    println!("{}", urlify(&"Mr John Smith     "));
}
