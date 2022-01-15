use std::time::Instant;
use varisat::{Lit, Solver};

fn main() {
    solve(8);
    solve(10);
    solve(16);


    // 17x17 and 18x18 take about 5 hours each
    // solve(17);
    // solve(18);


    // for i in 2..=16 {
    //     solve(i);
    // }
}

fn solve(size: usize) {
    let mut solver = Solver::new();
    let dimacs_cnf = build_dimacs_cnf(size + ((size % 2 != 0) as usize));
    solver.add_dimacs_cnf(&dimacs_cnf[..]).expect("parse error");

    println!("............................................");
    println!(
        "{f}...................{s}x{s}....................{f}",
        s = size,
        f = if size < 10 { "." } else { "" }
    );

    let start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = start.elapsed();

    println!("... Took: {:?}", duration);
    println!("............................................");

    if solution {
        let model = solver.model().unwrap();
        print_solution(size, model);
    } else {
        println!("No solution found!");
    }
}

fn build_dimacs_cnf(size: usize) -> Vec<u8> {
    if size % 2 != 0 {
        panic!("solve: size must be even!");
    }

    let mut dimacs_cnf = Vec::new();
    for i in 0..size - 1 {
        for j in i + 1..size {
            for k in 0..size - 1 {
                for l in k + 1..size {
                    let ik = get_dimacs_pos_id(size, i, k);
                    let il = get_dimacs_pos_id(size, i, l);
                    let jk = get_dimacs_pos_id(size, j, k);
                    let jl = get_dimacs_pos_id(size, j, l);
                    dimacs_cnf.extend_from_slice(
                        format!("-{} -{} -{} -{} 0\n", ik, il, jk, jl).as_bytes(),
                    );
                }
            }
        }
    }
    for r in 0..size / 2 {
        for c in 0..size / 2 {
            let x1 = get_dimacs_pos_id(size, r, c);
            let x2 = get_dimacs_pos_id(size, c, size - r - 1);
            let x3 = get_dimacs_pos_id(size, size - r - 1, size - c - 1);
            let x4 = get_dimacs_pos_id(size, size - c - 1, r);

            // at least x must be 1
            dimacs_cnf.extend_from_slice(format!("{} {} {} {} 0\n", x1, x2, x3, x4).as_bytes());

            // at most one x can be one
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x1, x2).as_bytes());
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x1, x3).as_bytes());
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x1, x4).as_bytes());
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x2, x3).as_bytes());
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x2, x4).as_bytes());
            dimacs_cnf.extend_from_slice(format!("-{} -{} 0\n", x3, x4).as_bytes());
        }
    }

    dimacs_cnf
}

fn get_dimacs_pos_id(size: usize, row: usize, col: usize) -> usize {
    row * size + col + 1
}

fn get_cords_from_lit(mut size: usize, lit: Lit) -> (usize, usize) {
    if size % 2 != 0 {
        size += 1;
    }
    let pos = lit.index();
    let row = pos / size;
    let col = pos % size;
    (row, col)
}

fn print_solution(size: usize, model: Vec<Lit>) {
    let mut grid = vec![vec![false; size]; size];
    for lit in model {
        let (row, col) = get_cords_from_lit(size, lit);
        if row != size && col != size {
            grid[row][col] = lit.is_positive();
        }
    }
    let mut solution = vec![vec![0_u32; size]; size];
    for color in 1..=4 {
        for r in 0..size {
            for c in 0..size {
                if grid[r][c] {
                    solution[r][c] = color;
                }
            }
        }
        grid = rotate_90_degree(grid);
    }

    for line in solution {
        let line: String = line
            .iter()
            .map(|&d| char::from_digit(d, 10).unwrap())
            .collect();
        println!("{}", line);
    }
}

// rotates vec, must be quadratic
fn rotate_90_degree(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let size = v.len();
    let mut rotated = vec![vec![false; size]; size];

    #[allow(clippy::needless_range_loop)]
    for r in 0..size {
        for c in 0..size {
            rotated[r][c] = v[c][size - r - 1];
        }
    }

    rotated
}
