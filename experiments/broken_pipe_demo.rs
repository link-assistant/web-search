// Standalone reproduction of the --list-providers broken-pipe handling used in
// rust/src/main.rs. Demonstrates that capturing+writing once and treating
// BrokenPipe as success yields exit 0 even when piped into `head`.
fn render_providers() -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let _ = writeln!(out, "Registered providers (200 total):\n");
    for i in 0..200 {
        let _ = writeln!(out, "  provider-{i:<4} Example Provider {i}");
    }
    out
}

fn write_stdout(text: &str) -> std::io::Result<()> {
    use std::io::Write as _;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(text.as_bytes())?;
    handle.flush()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match write_stdout(&render_providers()) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(error.into()),
    }
}
