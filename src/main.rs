use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let city = &args[1];
    let country = &args[2];
    let weather_type = &args[3];
}
