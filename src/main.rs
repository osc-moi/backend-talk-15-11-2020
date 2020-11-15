extern crate crossbeam;

use std::thread;
use std::time::Duration;

fn main() {
    assert_eq!(factorial(5), 120);
    println!("done");

    concurrent()
}

fn factorial(num: u8) -> i64 {
    //example: sum 5+4+3+2+1=15
    //let sum =(1..=num).fold(0i64,|sum,x| sum + x as i64);
    //assert_eq!(15,sum);

    // 5! = 5*4*3*2*1
    (1..=num).fold(1i64, |acc, x| acc * x as i64)
}

const MEMBERS: u32 = 100;

fn concurrent() {
    crossbeam::scope(|spawner| {
        for i in 1..=MEMBERS {
            spawner.spawn(move || {
                thread::sleep(Duration::from_secs(1));
                println!("{}: person done eating", i);
            });
        }
    });
}
