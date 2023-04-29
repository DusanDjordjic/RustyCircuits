fn main() {
    println!("Hello, world!");
    let mut network: rusty_circuits::Network<u32> = rusty_circuits::Network::default();

    if let Err(e) = rusty_circuits::loaders::stdin::start_menu(&mut network) {
        eprintln!("{e}");
    }
}
