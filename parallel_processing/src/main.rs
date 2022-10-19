use rayon::current_num_threads;
use rayon::prelude::*;

fn main() {
    println!("Num of threads: {:?}", current_num_threads());
    // 1 kB of data
    let mut v = vec![64; 1024];

    // edit data in parallel
    v.par_iter_mut().for_each(|x| *x += 1);
    let text = String::from_utf8(v).unwrap();
    println!("{:?}", text);
}
