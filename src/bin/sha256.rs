use std::ffi::{c_char, c_long};

#[link(name = "sha256", kind = "static")]
extern "C" {
  fn sha256_init(ctx: &mut Ctx);
  fn sha256_update(ctx: &mut Ctx, data: *const u8, len: usize);
  fn sha256_final(ctx: &mut Ctx, hash: *const u8);
}

#[derive(Debug)]
#[repr(C)]
struct Ctx {
  pub data: [c_char; 64],
  pub word: i32,
  pub bitlen: c_long,
  pub state: [u8; 8],
}

impl Default for Ctx {
  fn default() -> Self {
    let char = c_char::from(0_i8);

    Self {
      data: [char; 64],
      word: Default::default(),
      bitlen: Default::default(),
      state:  [0_u8; 8],
    }
  }
}

fn main() {
  unsafe {
    println!("[Rust]");
    let msg = "message".as_bytes();
    let mut hash: [u8; 32] = [0_u8; 32];
    let mut ctx = Ctx::default();

    sha256_init(&mut ctx);
    sha256_update(&mut ctx, msg.as_ptr(), msg.len());
    sha256_final(&mut ctx, hash.as_mut_ptr());

    println!("Hash of '{:?}' is '{:?}'", msg, hex::encode(&hash));
  }
}
