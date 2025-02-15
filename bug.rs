fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0).unwrap();
    let second = vec.get(1).unwrap();
    println!("First: {}", first);
    println!("Second: {}", second);
    // This will panic because we've already consumed the vector
    // println!("Third: {}", vec.get(2).unwrap());
}