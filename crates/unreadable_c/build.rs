fn main() {
    println!("cargo::rerun-if-changed=src/lol.c");
    cc::Build::new().file("src/lol.c").compile("lol_c");
}
