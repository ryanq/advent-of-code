fn main() -> anyhow::Result<()> {
    let number_for_five_zeroes = find_hash_with_n_leading_zeroes(5, INPUT);
    println!("part 1: {number_for_five_zeroes}");

    let number_for_six_zeroes = find_hash_with_n_leading_zeroes(6, INPUT);
    println!("part 2: {number_for_six_zeroes}");
    
    Ok(())
}

fn find_hash_with_n_leading_zeroes(n: usize, key: &str) -> usize {
    let pattern = "0".repeat(n);
    
    // We're iterating an infinite series. We'll either find it or hit the end
    // of the datatype range.
    (1..)
        .map(|n| (n, n))
        .map(|(n, input)| (n, format!("{:x}", md5::compute(format!("{key}{input}")))))
        .find(|(_, digest)| digest.starts_with(&pattern))
        .map(|(n, _)| n)
        .unwrap()
}

const INPUT: &str = "ckczppom";
