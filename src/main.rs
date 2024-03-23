use num_cpus::{get, get_physical};

fn main() {
    let cores = get();
    let cpus = get_physical();
    println!("logical cores: {cores}");
    println!("physical cores: {cpus}");
}
