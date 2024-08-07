use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rustc-link-arg=-fuse-ld=lld");
    generate_cargo_keys();

    rerun_if_git_head_changed();
}
