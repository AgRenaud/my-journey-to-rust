fn main() {
    // Add string to string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // Add character to String
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    // Concatenate two String
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("s1 is {}", s1); Don't work here.
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    // We need to refer to the 2 first bytes that encode
    // a character to get the first character of the string
    println!("{}", hello);
    println!("{}", answer);

    println!("print characters of नमस्ते");
    for c in "नमस्ते".chars() {
        println!("\t{}", c);
    }

    println!("print bytes of नमस्ते");
    for b in "नमस्ते".bytes() {
        println!("\t{}", b);
    }
}
