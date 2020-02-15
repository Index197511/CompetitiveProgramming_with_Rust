#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
//usage
//input! {
//  h: i32;
//  v: [i32; n]
//  x: [[i32; n]; m]
//  f: chars <- Vec<char>
//}

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

fn main() {
    input!(x: i32);
    let mut primes = eratosthenes(x * 2);
    for i in primes.unwrap() {
        if i >= x {
            println!("{}", i);
            break;
        } else {}
    }
}