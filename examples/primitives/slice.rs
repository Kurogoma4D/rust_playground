use std::mem;

fn analyse_slice(slice: &[i32]) {
    println!("first element of the slice:{}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyse_slice(&xs);

    analyse_slice(&ys[1 .. 4]);
}