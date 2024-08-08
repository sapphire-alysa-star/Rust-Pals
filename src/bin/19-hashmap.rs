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

        let res_5 = 8;
        let res_8 = 34;
        let res_18 = 4181;
        let res_28 = 514229;

        assert_eq!(num_5, res_5, "fifth number was {}, it should be {}", num_5, res_5);
        assert_eq!(num_8, res_8, "eighth number was {}, it should be {}", num_8, res_8);
        assert_eq!(num_18, res_18, "18th number was {}, it should be {}", num_18, res_18);
        assert_eq!(num_28, res_28, "28th number was {}, it should be {}", num_28, res_28);

        // What an error looks like:
        // assertion `left == right` failed: 28th number was 317811, it should be 514229
        // left: 317811
        // right: 514229

        // It doesnt really matter how you test but testing is important.
    }
}