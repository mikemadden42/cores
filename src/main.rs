use ansi_term::Colour::RGB;
use num_cpus::get;
use num_cpus::get_physical;

fn main() {
    let cores = get();
    let cpus = get_physical();
    println!("{} {}", RGB(112, 150, 75).paint("logical cores:"), RGB(90, 72, 98).paint(cores.to_string()));
    println!("{} {}", RGB(112, 150, 75).paint("physical cores:"), RGB(90, 72, 98).paint(cpus.to_string()));
}
