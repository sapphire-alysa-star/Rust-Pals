// Options let you avoid runtime errors like v[out_of_bounds]

fn main() {
    // Option enum has 2 variants.

    // 1- None is used to indicate failure or no value

    // 2- Some which is tuple-struct that wraps the value

    // class Some<T> implements Option<T> {
    //     private value: T;
       
    //     constructor(v: T) {
    //       this.value = v;
    //     }
       
    //     unwrap(): T {
    //       return this.value
    //   }
    // }

    // class None<T> implements Option<T> {
    //     // you do not need constructor here     
    //      unwrap(): T {
    //        return null as T;
    //     }
    // }

    #[derive(Debug, Clone, Copy)]
    enum MixedData<'a> {
        Name(&'a str),
        Number(u8)
    }

    let x: Option<MixedData> = None;
    let y: Option<MixedData> = Some(MixedData::Number(42));
    let celene = Some(MixedData::Name("Celene"));

    let options_to_unwrap = [&x, &y, &celene];

    for opt in &options_to_unwrap {
        if opt.is_some() {
            println!("We can unwrap this option and get: {:?}", opt.unwrap());
        } else {
            println!("we avoided an error by nor unwrapping a None option and getting null");
        }
    }
    



    // Options




    // Match statements
    


    // vec.get instead of v[]

    let v = vec![0, 1, 2, 3, 4];
    let _res1 = v.get(0); // 0
    let _res2 = v.get(6); // None

    // for let
    // syntactic sugar I find moderately confusing but its common ok
    let indices = [0, 1, 6];
    for i in &indices {
        if let Some(x) = v.get(*i) {
            println!("result {}", x);
        } else {
            println!("out of bounds");
        }
    }


}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



