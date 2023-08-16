use bytes::{Buf, Bytes};
use std::hint::black_box;

pub fn main() {
    let mut time: u128 = 0;
    let mut time_unchecked: u128 = 0;
    let eef = vec![5, 0, 0, 0];
    let mut bytes = Bytes::from(eef);

    let temp = temp(&mut bytes);

    let eef = vec![5, 0, 0, 0];
    let mut bytes = Bytes::from(eef);

    let temp2 = temp_2(&mut bytes);

    println!("Checked: {:?}", temp);
    println!("Unchecked: {:?}", temp2);
}

#[inline(never)]
pub fn temp(eef: &mut Bytes) -> f32 {
    eef.get_f32()
}

#[inline(never)]
pub fn temp_2(eef: &mut Bytes) -> f32 {
    eef.get_f32_unchecked()
}