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
    

    let x: Option<u8> = None;
    
    println!("{:?}", x); // None
    println!("{:?}", Some(42)); // Some(42)



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



