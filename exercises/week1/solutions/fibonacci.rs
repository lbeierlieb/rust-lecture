/// Calculate the nth number of the Fibonacci sequence
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut previous = 0;
    let mut current = 1;
    let mut counter = 1;

    while counter < n {
        let tmp = previous;
        previous = current;
        current += tmp;

        counter += 1;
    }

    current
}

fn main() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
    assert_eq!(fibonacci(50), 12586269025);

    println!("Your code seems to work well!");
}
