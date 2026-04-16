use resona::Config;

mod utils;

fn main() {
    let config = Config { min: 4, max: 8 };
    match utils::generate_name::generate_name(&mut nanorand::ChaCha::new(), &config) {
        Ok(name) => println!("{}", name),
        Err(e) => eprintln!("Error generating name: {}", e),
    }
}
