use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;

//let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build().unwrap();
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    println!("lego");

    //default: random seed
    let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed);

    let seed = [90, 115, 247, 168, 157, 255, 63, 39, 98, 186, 212, 255, 239, 7, 255, 77, 247, 23, 31, 142, 57, 236, 111, 160, 154, 32, 249, 249, 244, 211, 10, 148];

    println!("{:?}", seed);
    let mut rng = ChaCha8Rng::from_seed(seed);
    println!("{:?} {:?} {:?}", rng.gen_range(0..1000), rng.gen_range(0..1000), rng.gen_range(0..1000));
    //[90, 115, 247, 168, 157, 255, 63, 39, 98, 186, 212, 255, 239, 7, 255, 77, 247, 23, 31, 142, 57, 236, 111, 160, 154, 32, 249, 249, 244, 211, 10, 148]

    let mut seed2: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
    let mut seed3: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
    rng.fill(&mut seed2);
    rng.fill(&mut seed2);
    let mut rng2 = ChaCha8Rng::from_seed(seed2);
    let mut rng3 = ChaCha8Rng::from_seed(seed3);
    println!("{:?} {:?} {:?}", rng2.gen_range(0..1000), rng2.gen_range(0..1000), rng2.gen_range(0..1000));
    println!("{:?} {:?} {:?}", rng3.gen_range(0..1000), rng3.gen_range(0..1000), rng3.gen_range(0..1000));


    Ok(())
}