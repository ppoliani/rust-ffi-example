extern crate cc;

fn main() {
  cc::Build::new().file("c/num.c").compile("num");
  cc::Build::new().file("c/multiply.c").compile("multiply");
}
