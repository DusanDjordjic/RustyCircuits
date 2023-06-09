use std::hash::Hash;
use std::io::Write;
use std::str::FromStr;

use rusty_circuits::Edge;
use rusty_circuits::Network;
use rusty_circuits::Node;

fn main() -> std::io::Result<()> {
    let mut network: Network<usize> = Network::default();
    menu(&mut network)
}

fn menu<K>(network: &mut Network<K>) -> std::io::Result<()>
where
    K: Eq + Hash + Copy + FromStr,
{
    let mut user_input = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    'outer_loop: loop {
        println!("1. Add Node");
        println!("2. Add Edge");
        println!("0. Quit");
        print!("> ");

        stdout.flush()?;

        stdin.read_line(&mut user_input)?;

        if user_input.starts_with('0') {
            break 'outer_loop;
        } else if user_input.starts_with('1') {
            if let Err(e) = network.insert_node(load_node()) {
                eprintln!("{e}")
            }
        } else if user_input.starts_with('2') {
            let u = select_node();
            let v = select_node();
            network.insert_egde(load_edge(u, v));
        } else {
            println!("Invalid input");
        }
    }
    Ok(())
}

fn load_edge<K>(_u: Node<K>, _v: Node<K>) -> Edge<K>
where
    K: FromStr + Copy,
{
    unimplemented!()
}

fn load_node<K>() -> Node<K>
where
    K: FromStr,
{
    unimplemented!()
}

fn select_node<K>() -> Node<K> {
    unimplemented!()
}
