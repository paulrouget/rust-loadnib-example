use std::process::Command;
fn main() {
    Command::new("ibtool")
        .args(&["Application.xib", "--compile", "Application.nib"])
        .status()
        .unwrap();
}
