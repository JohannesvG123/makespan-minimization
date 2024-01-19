use std::thread::sleep;
use std::time::{Duration, SystemTime};
use chrono::Local;

use rand::Rng;
use rand::rngs::ThreadRng;

//let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build().unwrap();
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    println!("lego");
    let date = Local::now();
    println!("{}", date.format("%Y-%m-%d_%H-%M-%S"));
    println!("{:?}", SystemTime::now());

    let map = concurrent_map::ConcurrentMap::<(usize, usize), usize>::default();
    println!("{}", map.len());

    /*map.insert((1, 0), 10);
    println!("{}", map.len());
    map.insert((1, 1), 11);
    println!("{}", map.len());
    map.insert((1, 2), 12);
    println!("{}", map.len());
    map.insert((2, 0), 33);
    println!("{}", map.len());
    println!("{:?}", map.get_gte(&(1, 0)));
    println!("{:?}", map.get_lt(&(2, 0)));*/

    let thread_pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let m = map.clone();

    let z = thread_pool.scope(move |scope| {
        m.insert((1, 0), 99);

        for i in 0..10 {
            let m = m.clone();
            scope.spawn(move |_| {
                let mut rng = ThreadRng::default();
                sleep(Duration::from_millis(rng.gen_range(0..1000)));
                m.insert((i, 0), 99);
                sleep(Duration::from_millis(rng.gen_range(0..1000)));
                println!("t {} {}", m.len(), i);
            });
        }
        77
    });
    println!("{:?}", z);
    println!("{}", map.len());
    Ok(())
}