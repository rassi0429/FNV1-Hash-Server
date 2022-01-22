#[macro_use] extern crate rocket;

#[get("/?<text>")]
fn hello(text: Option<String>) -> String {
    text.map(|text| format!("{}", hash(text)))
        .unwrap_or_else(|| "text param not provided".into())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

fn hash(s: String) -> u32 {
    let prime: u32 = 16777619;
    let mut hash: u32 = 2166136261;
    for c in s.chars() {
        hash = hash.wrapping_mul(prime);
        hash = hash ^ (c as u32);
    }
    return hash;
}