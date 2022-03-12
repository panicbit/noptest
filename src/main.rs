use std::arch::asm;
use std::time::Instant;

use metarust::metarust;

macro_rules! nops {
    ($num:expr) => {
        unsafe {
            metarust! {
                let nops = std::iter::repeat("nop")
                    .take($num)
                    .join("\n");

                quote! {
                    asm!(#nops);
                }
            }
        }
    }
}

fn main() {
    let start = Instant::now();

    nops!(1);

    println!("1 nop took {:?}", start.elapsed());

    let start = Instant::now();

    nops!(1_000_000);

    println!("1_000_000 nops took {:?}", start.elapsed());
}
