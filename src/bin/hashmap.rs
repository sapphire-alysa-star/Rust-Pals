use std::collections::HashMap;

fn dp_ladder(i: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if i == 1 {
        return 1
    } else if i == 2 {
        return 2
    } else if let Some(x) = memo.get(&i) {
        return *x
    }

    let smol = dp_ladder(i - 2, memo);
    let big = dp_ladder(i - 1, memo);

    memo.insert(i, smol + big);
    
    return smol + big
}

fn main () {
    
    let mut memo: HashMap<u64, u64> = HashMap::new();

    println!("dp(5) {}", dp_ladder(5, &mut memo));
    println!("dp(6) {}", dp_ladder(6, &mut memo));
    println!("dp(7) {}", dp_ladder(7, &mut memo));

}