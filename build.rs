fn main() {
    #[cfg(target_os = "macos")]
    {
        // Use pkg-config so Homebrew/Nix/other setups can resolve dependencies.
        check_pkg_config("xkbcommon", "xkbcommon");
        check_pkg_config("pixman-1", "pixman");
    }
}

#[cfg(target_os = "macos")]
fn check_pkg_config(pkg: &str, brew_hint: &str) {
    if pkg_config::probe_library(pkg).is_ok() {
        return;
    }

    eprintln!();
    eprintln!("╔══════════════════════════════════════════════════════════════╗");
    eprintln!("║  ERROR: Missing dependency '{}'", pkg);
    eprintln!("╠══════════════════════════════════════════════════════════════╣");
    eprintln!("║  Make sure pkg-config can find it.                           ║");
    eprintln!("║                                                              ║");
    eprintln!("║  Homebrew:                                                   ║");
    eprintln!(
        "║    brew install lib{}                                   ║",
        brew_hint
    );
    eprintln!("║                                                              ║");
    eprintln!("║  Then rebuild:                                               ║");
    eprintln!("║    cargo clean && cargo build --release                      ║");
    eprintln!("╚══════════════════════════════════════════════════════════════╝");
    eprintln!();
    std::process::exit(1);
}
