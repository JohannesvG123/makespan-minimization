use std::thread::sleep;
use std::time::Duration;

//let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build().unwrap();
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    println!("{}", pool.current_num_threads());
    for i in 0..20 {
        pool.spawn(move || {
            println!("Hello from my fully custom thread!{}", i);
            sleep(Duration::new(1, 0));
            println!("gumo{}", i);
        });
    }
    sleep(Duration::new(3, 0));
    println!("ende");
    Ok(())
}