//let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build().unwrap();
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    println!("lego");
    let range = 1..2;
    println!("{:?}", range);
    println!("{:?}", range.len());
    println!("{:?}", range.start);

    Ok(())
}