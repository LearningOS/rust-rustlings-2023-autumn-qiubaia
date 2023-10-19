//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, set up an environment variable called `TEST_FOO`.
    // Print in the standard output to let Cargo do it.
    let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs();
println!("cargo:TEST_FOO={}", timestamp);

    // In tests8, enable the "pass" feature to make the test return early.
    let your_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("{}", your_command);
}

