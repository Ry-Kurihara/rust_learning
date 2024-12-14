fn main() {
    let mut data_vec = Vec::new();
    data_vec.push(1i32);
    data_vec.push(7i32);
    let mut vec: Vec<&mut Vec<i32>> = Vec::new();

    vec.push(&mut data_vec);
    vec[0][1] = 99;
    println!("{:?}", data_vec);
}