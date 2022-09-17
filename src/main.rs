use num_cpus::get;
use num_cpus::get_physical;

fn main() {
    let cores = get();
    let cpus = get_physical();
    #[cfg(target_os = "windows")]
    let _enabled = ansi_term::enable_ansi_support();
    println!("logical cores: {}", cores);
    println!("physical cores: {}", cpus);
}
