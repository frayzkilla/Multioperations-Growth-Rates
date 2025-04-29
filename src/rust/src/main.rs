mod core;
mod generator;
mod utils;
use core::batch_process_operations;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let k = 2;
    let n = 4;
    let target_length = 256;
    let operations_length = 4;
    let filename = "test.txt".to_string();

    batch_process_operations(k, n, target_length, operations_length, filename)?;

    Ok(())    
}
