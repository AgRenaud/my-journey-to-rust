fn main() {
    // Create a vector of int32
    let mut v: Vec<i32> = Vec::new();

    // add element to v
    v.push(6);
    v.push(7);
    v.push(8);

    println!("My vec is {:?}", v);

    println!("Add 10 to each element of vector");

    for i in &mut v {
        *i += 10;
    }

    println!("My vec is {:?}", v);

}
