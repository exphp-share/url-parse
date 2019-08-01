use url::Url;

fn main() {
    let url = "https://bla/bla".to_string();
    let _ = Url::parse(&url).unwrap();
}
