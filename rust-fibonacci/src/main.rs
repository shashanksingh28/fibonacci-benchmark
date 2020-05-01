use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Provide type (rec:0, loop:1, dynamic:2) and n'th fibonacci to compute");
        return 
    }

    let func : usize = args[1].trim_end().parse().unwrap();
    let num : usize = args[2].trim_end().parse().unwrap();
    
    match func {
        0 => println!("{}", fibonacci_rec(num)),
        1 => println!("{}", fibonacci_loop(num)),
        2 => {
            let mut mem = HashMap::new();
            println!("{}", fibonacci_dynamic(num, &mut mem))
        },
        _ => println!("Wrong type provided!")
    }
}

fn fibonacci_rec(n: usize) -> usize {
    // 1 1 2 3 5 8 13
    if n < 3 {
        1
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
    }
}

fn fibonacci_loop(n: usize) -> usize {
    // 1 1 2 3 5 8 13
   let mut prev = 1;
   let mut current = 1;
   let mut temp;
   for _i in 2..n {
      temp = current + prev;
      prev = current;
      current = temp;
   }
   current
}

fn fibonacci_dynamic(n: usize, memoized: &mut HashMap<usize, usize>) -> usize{
    if n < 3 {
        1
    } else if memoized.contains_key(&n) {
        memoized.remove(&n).unwrap()
    } else {
        let value = fibonacci_dynamic(n - 1, memoized) + fibonacci_dynamic(n - 2, memoized);
        memoized.insert(n, value);
        value
    }
}
