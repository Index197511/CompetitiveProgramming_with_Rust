fn eratosthenes(n: i32) -> Result<Vec<i32>, String> {
    match n {
        n if n < 2 => {
            return Err("n is more than 2".to_owned());
        }
        _ => {
            let mut prime = Vec::new();
            let limit = (n as f64).sqrt();
            let mut table: Vec<i32> = (2..(n + 1)).collect();
            let mut p = 0;

            loop {
                p = table[0];
                if limit <= p as f64 {
                    prime.append(&mut table);
                    return Ok(prime);
                }
                prime.push(p);
                table = table.into_iter().filter(|e| e % p != 0).collect();
            }
        }
    };
}