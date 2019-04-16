use std::collections::HashMap;

fn main() {
    let mut memo: HashMap<u128, u128> = HashMap::new();
    for n in 1..1000 {
        println!("fib({}) = {:?}", n, memo_fib(n, &mut memo));
    }
}


fn memo_fib(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    match n {
        0 | 1 => 1,
        n => {
            if memo.contains_key(&n) {
                return *memo.get(&n).unwrap();
            }

            let val = memo_fib(n - 1, memo) + memo_fib(n - 2, memo);
            memo.insert(n, val);
            val
        }
    }
}
