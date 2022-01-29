#[get("/")]
pub fn index() -> &'static str {
    let pangram: &'static str = "Hi there! I made an irony";
    return pangram;
}
