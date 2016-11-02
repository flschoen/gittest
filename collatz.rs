

fn collatz(n: i32, count: i32) {
    
    let mut counter = count;
    
    println!(" {} -> {}",counter,  n);
    if n == 1 { return }
    else if n % 2 == 0 {
        counter += 1;
        collatz(n / 2, counter) }
    else {
        counter += 1;
        collatz(3*n + 1, counter) }
    
}

fn main() {

    collatz(27, 0);
}   