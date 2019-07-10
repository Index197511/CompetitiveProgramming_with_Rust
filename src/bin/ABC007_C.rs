use std::collections::VecDeque;

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



fn main(){

    input!{
        r: i32,
        c: i32,
        start_y: i32,
        start_x: i32,
        goal_y: i32,
        goal_x: i32,
        mut field: [chars; r],
    }

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut cnt: i32 = 0;

    #[warn(unused_assignments)] 
    let mut loop_cnt: usize = 0;
    let dxy: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    queue.push_back((start_y - 1, start_x - 1));
    's: loop{
        loop_cnt = queue.len();
        cnt += 1;

        for _i in 0..loop_cnt{
            let (y, x): (i32, i32) = queue.pop_front().unwrap();

            for &(dy, dx) in &dxy{


                if -1 < (dy + y) && (dy + y) < r && -1 < (dx + x) && (dx + x) < c {
                    if field[(dy + y) as usize][(dx + x) as usize] == '.' {
                        queue.push_back((dy + y, dx + x));
                        field[(dy + y) as usize][(dx + x) as usize] = '#';
                    }
                }
            }

        }

        if queue.contains(&(&goal_y - 1, &goal_x - 1)){break 's;};

    }

    println!("{}", cnt);

}






