use num_cpus::{get, get_physical};

fn main() {
    let logical = get();
    let physical = get_physical();
    println!("logical cores: {logical}");
    println!("physical cores: {physical}");
}

#[cfg(test)]
mod tests {
    use num_cpus::{get, get_physical};

    #[test]
    fn counts_are_nonzero() {
        assert!(get() >= 1);
        assert!(get_physical() >= 1);
    }
}
