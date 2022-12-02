fn is_compiled_for_64_bit_arch() -> bool {
    cfg!(target_pointer_width = "64")
}

use git_version::git_version;

fn main() {
    assert!(
        is_compiled_for_64_bit_arch(),
        "must be built for 64-bit architectures"
    );
    println!(
        "cargo:rustc-env=COMPILE_TIME={}",
        chrono::Local::now().to_rfc2822()
    );

    let _: &str = git_version!(args = ["--abbrev=40", "--always"]);
    println!(
        "cargo:rustc-env=GIT_VERSION={}",
        git_version!(args = ["--abbrev=40", "--always"])
    );
}
