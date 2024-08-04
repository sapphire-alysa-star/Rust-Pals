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

    println!("dp(5) = {}. Counter: {}", dp_ladder(5, &mut memo, &mut counter), counter);
    println!("dp(6) = {}. Counter: {}", dp_ladder(6, &mut memo, &mut counter), counter);
    println!("dp(7) = {}. Counter: {}", dp_ladder(7, &mut memo, &mut counter), counter);

    for i in [5, 6, 7, 8, 18, 28] {
        let res = dp_ladder(5, &mut memo, &mut counter);
        println!("dp({i}) = {res}. Counter: {counter}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut memo: HashMap<u64, u64> = HashMap::new();
        let mut counter = 0;

        let num_5 = dp_ladder(5, &mut memo, &mut counter);
        let num_8 = dp_ladder(8, &mut memo, &mut counter);
        let num_18 = dp_ladder(18, &mut memo, &mut counter);
        let num_28 = dp_ladder(28, &mut memo, &mut counter);
        
        assert_eq!(num_5, 8);
        assert_eq!(num_8, 34);
        assert_eq!(num_18, 4181);
        assert_eq!(num_28, 514229);
    }
}