fn main() {
  println!("cargo:rustc-link-search=all=libs");
  println!("cargo:rustc-link-lib=static=main");
}
