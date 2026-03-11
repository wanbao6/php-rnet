fn main() {
    // ext-php-rs 0.15+ handles all PHP link configuration in its own build script.
    // We only need to re-run if env vars change.
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");
    println!("cargo:rerun-if-env-changed=PHP");
}
