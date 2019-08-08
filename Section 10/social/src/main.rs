extern crate social;

fn main() {
    social::engine::login();
    social::engine::post(String::from("Test"));
    social::engine::logout();
}
