use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    let num_to_add: usize = args[2].parse().expect("Couldn't parse num_to_add arg");

    println!("File path: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // create a list of the sums, then 
    let mut sums = Vec::new();
    let mut cur_sum = 0;
    for line in contents.lines() {
        if line == "" {
            println!("Pushing: {}", cur_sum);
            sums.push(cur_sum);
            cur_sum = 0;
        }
        else {
            let x: u32 = line.parse().expect("Couldn't parse line");
            cur_sum += x;
        }
    }
    sums.sort();
    sums.reverse();

    println!("Adding: {:?}", &sums[..num_to_add]);

    let sum: u32 = sums[..num_to_add].iter().sum();

    println!("Sum: {}", sum);
}
