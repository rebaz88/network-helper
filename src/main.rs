mod options;
use is_root::is_root;

#[cfg(target_os = "windows")]
use winres;

#[cfg(target_os = "windows")]
fn main() {
    options::run();
}

#[cfg(not (target_os = "windows"))]
fn main() {
    if is_root_user() {
        options::run();
    }
}

fn is_root_user() -> bool {
    let is_root = is_root();
    if ! is_root {
        eprintln!("\n*** Should run the application as admin! ***\n");
    }
    return is_root;
}