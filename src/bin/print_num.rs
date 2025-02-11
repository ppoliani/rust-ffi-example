#[link(name = "num", kind = "static")]
extern "C" {
  fn print_num(num: i32);
}

fn main() {
  unsafe {
    println!("[Rust]");
    print_num(100);
  }
}
