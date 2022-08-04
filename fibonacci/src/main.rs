use std::io;

fn next(a: u32, b: u32) -> (u32, u32) {
    (b, a + b)
}

fn main() {
    println!("Enter an integer N.");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("N needs to be a nonnegative integer.");
    let res = match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                (a, b) = next(a, b);
            }
            b
        }
    };

    println!("{}th fibonacci number is {}.", n, res);
}
