use std::collections::HashMap;

fn dp_ladder(i: u64, memo: &mut HashMap<u64, u64>, counter: &mut u64) -> u64 {
    *counter += 1;

    if i == 1 {
        return 1
    } else if i == 2 {
        return 2
    } else if let Some(x) = memo.get(&i) {
        return *x
    }

    let smol = dp_ladder(i - 2, memo, counter);
    let big = dp_ladder(i - 1, memo, counter);

    memo.insert(i, smol + big);

    return smol + big
}

fn main () {
    
    let mut memo: HashMap<u64, u64> = HashMap::new();
    let mut counter = 0;

    for i in [5, 6, 7, 8, 18, 28] {
        let res = dp_ladder(i, &mut memo, &mut counter);
        println!("dp({i}) = {res}. Counter: {counter}");
    }
    // Counter shows we arent accidnetally taking too many calls.
    // We correctly comput fib(n+1)

    // dp(5) = 8. Counter: 7
    // dp(6) = 13. Counter: 10
    // dp(7) = 21. Counter: 13
    // dp(8) = 34. Counter: 16
    // dp(18) = 4181. Counter: 37
    // dp(28) = 514229. Counter: 58
}