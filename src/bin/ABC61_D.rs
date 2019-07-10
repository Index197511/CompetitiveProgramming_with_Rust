


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

struct Graph{
    edges: Vec<(usize, usize, i64)>,
}

impl Graph{

    fn init_graph() -> Graph{
        Graph{edges: Vec::new()}
    }

    fn add_edge(graph: &mut Graph, start_node: usize, end_node: usize, edge_cost: i64, directed: bool) -> (){
        graph.edges.push((start_node, end_node, edge_cost));
        if !directed {graph.edges.push((end_node, start_node, edge_cost));}
    }

}


fn bellman_ford(node: usize, graph: Graph, start_node: usize, distance: &mut Vec<i64>) -> i64{
    distance[start_node] = 0;
    for i in 0..node{
        for &(u, v, c) in &graph.edges{
            if distance[u] != 1000000000007 && distance[u] + c < distance[v]{
                distance[v] = distance[u] + c;
                if i == (node - 1) && v == (node - 1){
                    #[warn(unused_parens)]
                    return (-1000000000007)
                }
            }
        }
    }
    -distance[node-1]
}

fn main(){
    input!{
        n: usize,
        m: usize,
        edges: [(usize, usize, i64); m],
    }
    let mut graph = Graph::init_graph();
    let mut distance: Vec<i64> = vec![1000000000007; n];

    for i in &edges{
        Graph::add_edge(&mut graph, i.0 - 1, i.1 - 1, i.2 * (-1), true);
    }

    let ans = bellman_ford(n, graph, 0, &mut distance);
    if ans == (-1000000000007) {println!("inf")} else {println!("{}",ans)};
}
