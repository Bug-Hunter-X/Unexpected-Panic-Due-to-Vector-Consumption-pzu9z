fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0).copied().unwrap_or_default();
    let second = vec.get(1).copied().unwrap_or_default();
    println!("First: {}", first);
    println!("Second: {}", second);
    // This will not panic now because we haven't consumed the vector
    // The original code panicked because the get method borrows the vector
    // and it returns an Option<&T>. To avoid the panic, it is better to copy
    // the values that we need with copied() method.
    // Also, this solution handles cases where the index is out of bound
    // by using unwrap_or_default() instead of unwrap()
}