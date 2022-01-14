use std::time::Instant;

// use exercise_45_b::grid::Grid;
use varisat::{Lit, Solver};

fn main() {
    solve(2);
    solve(8);
    solve(10);
    solve(14);
    solve(15);
    solve(16);
}

fn solve(size: usize) {
    let mut solver = Solver::new();

    // let formula = build_formula(size);
    // solver.add_formula(&formula);

    let dimacs_cnf = build_dimacs_cnf(size);
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
    let mut dimacs_cnf = Vec::new();

    // let mut c_count = 0;
    // for i in 0..size - 1 {
    //     for j in i + 1..size {
    //         for k in 0..size - 1 {
    //             for l in k + 1..size {
    //                 c_count += 4;
    //             }
    //         }
    //     }
    // }

    // dimacs_cnf.extend_from_slice(format!("p cnf {} {}\n", 2 * size * size, c_count).as_bytes());

    for i in 0..size - 1 {
        for j in i + 1..size {
            for k in 0..size - 1 {
                for l in k + 1..size {
                    let (a_ik, b_ik) = get_dimacs_pos_id(size, i, k);
                    let (a_il, b_il) = get_dimacs_pos_id(size, i, l);
                    let (a_jk, b_jk) = get_dimacs_pos_id(size, j, k);
                    let (a_jl, b_jl) = get_dimacs_pos_id(size, j, l);
                    dimacs_cnf.extend_from_slice(
                        format!(
                            "{} {} {} {} {} {} {} {} 0\n",
                            a_ik, b_ik, a_il, b_il, a_jk, b_jk, a_jl, b_jl
                        )
                        .as_bytes(),
                    );
                    dimacs_cnf.extend_from_slice(
                        format!(
                            "-{} {} -{} {} -{} {} -{} {} 0\n",
                            a_ik, b_ik, a_il, b_il, a_jk, b_jk, a_jl, b_jl
                        )
                        .as_bytes(),
                    );
                    dimacs_cnf.extend_from_slice(
                        format!(
                            "{} -{} {} -{} {} -{} {} -{} 0\n",
                            a_ik, b_ik, a_il, b_il, a_jk, b_jk, a_jl, b_jl
                        )
                        .as_bytes(),
                    );
                    dimacs_cnf.extend_from_slice(
                        format!(
                            "-{} -{} -{} -{} -{} -{} -{} -{} 0\n",
                            a_ik, b_ik, a_il, b_il, a_jk, b_jk, a_jl, b_jl
                        )
                        .as_bytes(),
                    );
                }
            }
        }
    }

    dimacs_cnf
}

fn get_dimacs_pos_id(size: usize, row: usize, col: usize) -> (usize, usize) {
    let num = (row * size + col) * 2 + 1; // 1 indexed in dimacs
    (num, num + 1)
}

fn get_cords_from_lit(size: usize, lit: Lit) -> (usize, usize, bool) {
    let mut pos = lit.index();
    let is_a = pos % 2 == 0;
    pos /= 2;
    let row = pos / size;
    let col = pos % size;
    (row, col, is_a)
}

fn print_solution(size: usize, model: Vec<Lit>) {
    let mut grid = vec![vec![0_u32; size]; size];
    for lit in model {
        let (row, col, is_a) = get_cords_from_lit(size, lit);
        if lit.is_positive() {
            grid[row][col] |= 1 << (!is_a as u32);
        }
    }
    for line in grid {
        let line: String = line
            .iter()
            .map(|&d| char::from_digit(d + 1, 10).unwrap())
            .collect();
        println!("{}", line);
    }
}

// fn build_formula(size: usize) -> CnfFormula {
//     let mut formula = CnfFormula::new();

//     let mut grid = Vec::with_capacity(size);
//     let mut var_count = 0;
//     for _ in 0..size {
//         let mut grid_row = Vec::with_capacity(size);
//         for _ in 0..size {
//             grid_row.push((Var::from_index(var_count), Var::from_index(var_count + 1)));
//             var_count += 2;
//         }
//         grid.push(grid_row);
//     }

//     for i in 0..size - 1 {
//         for j in i + 1..size {
//             for k in 0..size - 1 {
//                 for l in k + 1..size {
//                     let (a_ik, b_ik) = grid[i][k];
//                     let (a_il, b_il) = grid[i][l];
//                     let (a_jk, b_jk) = grid[j][k];
//                     let (a_jl, b_jl) = grid[i][l];
//                     // let x =
//                     formula.add_clause(&[
//                         a_ik.positive(),
//                         b_ik.positive(),
//                         a_il.positive(),
//                         b_il.positive(),
//                         a_jk.positive(),
//                         b_jk.positive(),
//                         a_jl.positive(),
//                         b_jl.positive(),
//                     ]);
//                     formula.add_clause(&[
//                         a_ik.negative(),
//                         b_ik.positive(),
//                         a_il.negative(),
//                         b_il.positive(),
//                         a_jk.negative(),
//                         b_jk.positive(),
//                         a_jl.negative(),
//                         b_jl.positive(),
//                     ]);
//                     formula.add_clause(&[
//                         a_ik.positive(),
//                         b_ik.negative(),
//                         a_il.positive(),
//                         b_il.negative(),
//                         a_jk.positive(),
//                         b_jk.negative(),
//                         a_jl.positive(),
//                         b_jl.negative(),
//                     ]);
//                     formula.add_clause(&[
//                         a_ik.negative(),
//                         b_ik.negative(),
//                         a_il.negative(),
//                         b_il.negative(),
//                         a_jk.negative(),
//                         b_jk.negative(),
//                         a_jl.negative(),
//                         b_jl.negative(),
//                     ]);
//                 }
//             }
//         }
//     }

//     formula
// }
