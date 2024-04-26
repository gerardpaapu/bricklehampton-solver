use std::collections::HashMap;
use std::str;

static BRANCHES: [i32; 13] = [-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6];
static ROTATIONS: [i32; 12] = [-6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6];
static FLIPS: [i32; 1] = [0];

struct PruningTable {
    map: HashMap<[u8; 14], i32>,
    mask: [u8; 7],
}

struct Stats {
    seen: i32,
    pruned: i32,
}

fn main() {
    let start: [u8; 14] = *b"tlibcheaomkpnr";
    let map: HashMap<[u8; 14], i32> = HashMap::new();
    let mut pruning_table = PruningTable {
        map,
        mask: *b"hampton",
    };
    build_table(&mut pruning_table);
    println!(
        "State = {}",
        str::from_utf8(&start).expect("should be valid")
    );
    if let Some(result) = search(&pruning_table, start) {
        println!("Solution found: {}", write_path(&result));
        let mut state = start.clone();
        run(&mut state, write_path(&result).into());
        println!(
            "{} -> {}",
            str::from_utf8(&start).unwrap(),
            str::from_utf8(&state).unwrap()
        );
    } else {
        println!("No solution")
    }
}

fn write_path(path: &Vec<i32>) -> String {
    let mut s = String::new();
    for element in path.iter() {
        if element < &0 {
            for _ in 0..(-element) {
                s.push_str("l")
            }
        } else if element > &0 {
            for _ in 0..(*element) {
                s.push_str("r")
            }
        } else {
            s.push_str("!")
        }
    }
    return s;
}

fn mask_state(state: &[u8; 14], mask: &[u8; 7]) -> [u8; 14] {
    let mut result = state.clone();
    for ch in result.iter_mut() {
        if mask.contains(ch) {
            *ch = b'_';
        }
    }
    result
}

fn path_length(path: &Vec<i32>) -> usize {
    path.iter()
        .map(|x| if x == &0 { 1 } else { x.abs() })
        .sum::<i32>()
        .try_into()
        .unwrap()
}

fn is_solved(state: &[u8; 14]) -> bool {
    state == b"bricklehampton"
}

fn build_table(table: &mut PruningTable) {
    let start = mask_state(b"bricklehampton", &table.mask);

    for depth in 0..20 {
        println!("Building table at depth: {}", depth);
        build_table_at_depth(table, start.clone(), &vec![], depth);
        println!("{} positions recorded", table.map.len());
    }
}

fn lookup(table: &PruningTable, state: [u8; 14]) -> Option<&i32> {
    let key = mask_state(&state, &table.mask);
    table.map.get(&key)
}

fn build_table_at_depth(
    table: &mut PruningTable,
    state: [u8; 14],
    path: &Vec<i32>,
    max_path_length: usize,
) {
    let len = path_length(&path);
    if len == max_path_length {
        if len <= 3 {
            println!("{}", str::from_utf8(&state).unwrap());
        }
        let entry = table.map.get(&state);
        let depth: i32 = len.try_into().unwrap();
        if entry.is_none() || entry.is_some_and(|x| x > &depth) {
            table.map.insert(state.clone(), depth);
        }
        return;
    }

    if len > max_path_length {
        return;
    }
    let branches = if path.len() == 0 {
        BRANCHES.iter()
    } else if path.last().is_some_and(|x| x == &0) {
        ROTATIONS.iter()
    } else {
        FLIPS.iter()
    };
    for step in branches {
        let next = run_step(state, *step);

        let mut path_next = path.clone();
        path_next.push(*step);
        build_table_at_depth(table, next, &path_next, max_path_length)
    }
}

fn run_step(state: [u8; 14], step: i32) -> [u8; 14] {
    let mut next = state.clone();
    if step < 0 {
        next.rotate_left((-step).try_into().unwrap());
    } else if step > 0 {
        next.rotate_right((step).try_into().unwrap());
    } else {
        next.swap(0, 3);
        next.swap(1, 2);
    }
    next
}

fn search(table: &PruningTable, state: [u8; 14]) -> Option<Vec<i32>> {
    let mut stats = Stats { seen: 0, pruned: 0 };
    for i in 0..40 {
        print!("Searching at depth: {}", i);
        let result = search_at_depth(table, state, &vec![], i, &mut stats);
        println!("\t(Seen = {},\tPruned = {})", stats.seen, stats.pruned);
        if result.is_some() {
            return result;
        }
    }

    None
}

fn search_at_depth(
    table: &PruningTable,
    state: [u8; 14],
    path: &Vec<i32>,
    max_path_length: usize,
    stats: &mut Stats,
) -> Option<Vec<i32>> {
    let len = path_length(&path);
    let minimum_remaining: usize = (*lookup(table, state).unwrap_or(&20)).try_into().unwrap();
    if len == max_path_length {
        stats.seen += 1;
        if is_solved(&state) {
            return Some(path.clone());
        }
        return None;
    }

    if is_solved(&state) {
        println!(
            "This was mis-pruned earlier: {}, {}, {}, {}",
            write_path(path),
            str::from_utf8(&state).unwrap(),
            len,
            max_path_length
        );

        return Some(path.clone());
    }

    if (len + minimum_remaining) > max_path_length {
        stats.pruned += 1;
        return None;
    }
    let branches = if path.len() == 0 {
        BRANCHES.iter()
    } else if path.last().is_some_and(|x| x == &0) {
        ROTATIONS.iter()
    } else {
        FLIPS.iter()
    };
    for step in branches {
        let next = run_step(state, *step);
        let mut path_next = path.clone();
        path_next.push(*step);
        if let Some(result) = search_at_depth(table, next, &path_next, max_path_length, stats) {
            return Some(result);
        }
    }

    None
}

fn run(src: &mut [u8; 14], script: Vec<u8>) {
    for char in script {
        run1(src, char)
    }
}

fn run1(src: &mut [u8; 14], step: u8) {
    match step {
        b'!' => {
            src.swap(0, 3);
            src.swap(1, 2);
        }
        b'l' => src.rotate_left(1),
        b'r' => src.rotate_right(1),
        _ => {}
    }
}
