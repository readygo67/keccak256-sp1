
use tiny_keccak::{Hasher, Keccak};

pub fn keccak256_iterations(n :u32, input : Vec<u8>) -> [u8; 32] {
    println!("n: {} input: {}", n, hex::encode(&input));
    let mut current_hash = input;
    let mut output = [0u8; 32];
    for i in 0..n {
        let mut hasher = Keccak::v256();
        hasher.update(&current_hash);
        hasher.finalize(&mut output);
        current_hash = output.to_vec();
        // println!("i:{},output: {}",i, hex::encode(&output));
    }
    return output;
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        println!("Hello, world!");
    }

    #[test]
    fn test_keccak256() {
        let input = vec![0u8; 3];
        let output = super::keccak256_iterations(10000, input);
        println!("output: {}", hex::encode(&output));
    }
}