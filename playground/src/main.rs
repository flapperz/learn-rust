use std::collections::HashMap;

fn fib(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if n < 2 {
        return 1;
    }

    match cache.get(&n) {
        Some(&num) => return num,
        _ => {
            let res = fib(n - 1, cache) + fib(n - 2, cache);
            cache.insert(n, res);
            return res;
        }
    }
}
fn main() {
    let mut x;
    let mut cache = HashMap::new();
    for i in 0..50 {
        x = fib(i, &mut cache);
        println!("{i} - {x}");
    }
    eprintln!("hello")
}
