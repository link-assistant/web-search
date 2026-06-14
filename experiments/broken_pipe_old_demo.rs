// Reproduces the ORIGINAL --list-providers behaviour: many println! calls.
// When piped into `head`, the reader closes early and the next println!
// panics with "failed printing to stdout: Broken pipe (os error 32)".
fn print_providers() {
    println!("Registered providers (200 total):\n");
    for i in 0..200 {
        println!("  provider-{i:<4} Example Provider {i}");
    }
}
fn main() {
    print_providers();
}
