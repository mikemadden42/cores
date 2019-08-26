extern crate num_cpus;

fn main() {
    let cores = num_cpus::get();
    let cpus = num_cpus::get_physical();
    println!("logical cores = {}", cores);
    println!("physical cpus = {}", cpus);
}
