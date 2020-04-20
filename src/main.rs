use ansi_term::Colour::Blue;
use ansi_term::Colour::Green;
use num_cpus::get;
use num_cpus::get_physical;

fn main() {
    let cores = get();
    let cpus = get_physical();
    println!(
        "{} {}",
        Blue.paint("logical cores:"),
        Green.paint(cores.to_string())
    );
    println!(
        "{} {}",
        Blue.paint("physical cores:"),
        Green.paint(cpus.to_string())
    );
}
