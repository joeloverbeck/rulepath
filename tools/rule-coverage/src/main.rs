fn main() {
    run_placeholder("rule-coverage", std::env::args().skip(1));
}

fn run_placeholder(name: &str, args: impl IntoIterator<Item = String>) {
    let wants_help = args
        .into_iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "--version" | "-V"));

    if wants_help {
        println!("{name} 0.1.0");
        println!("Gate 0 placeholder command.");
    } else {
        println!("{name}: no-op placeholder");
    }
}
