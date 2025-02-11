#[link(name = "multiply", kind = "static")]
extern "C" {
  fn multiply(a: i32, b: i32);
}

fn main() {
  unsafe {
    println!("[Rust]");
    multiply(10, 20);
  }
}
