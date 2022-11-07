use std::ops::{Add, Div};

#[derive(Debug)]
pub struct WrappedU8(u8);

impl Add for WrappedU8 {
    type Output = Result<Self, anyhow::Error>;

    fn add(self, other: Self) -> Result<Self, anyhow::Error> {
        if self.0 > u8::MAX - other.0 {
            Err(anyhow::format_err!("overflow exception"))
        } else {
            Ok(WrappedU8(self.0 + other.0))
        }
    }
}

impl Div for WrappedU8 {
    type Output = Result<Self, anyhow::Error>;

    fn div(self, other: Self) -> Result<Self, anyhow::Error> {
        if other.0 == 0 {
            Err(anyhow::format_err!("zero division exception"))
        } else {
            Ok(WrappedU8(self.0 / other.0))
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    println!("Start");

    // Step 1
    let x = WrappedU8(1) + WrappedU8(1);
    let x = x?;

    println!("1 + 1 = {}", x.0);

    // Step 2
    let x = WrappedU8(255) + WrappedU8(1);
    let x = x.unwrap_err();
    println!("255 + 1 = {}", x);

    // Step 3
    let x = WrappedU8(9) / WrappedU8(3);
    let x = x?;
    println!("9 / 3 = {}", x.0);

    // Step 4
    let x = WrappedU8(255) / WrappedU8(0);
    let x = x.unwrap_err();
    println!("255 / 0 = {}", x);

    println!("Finish");
    Ok(())
}
