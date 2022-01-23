use grid_coloring_problem_solver::solve;
use std::time::Instant;

fn main() {
    for i in 2..=16 {
        solve_and_print(i);
    }

    // 17x17 and 18x18 take about 5 hours each
    // solve_and_print(17);
    // solve_and_print(18);
}

fn solve_and_print(size: usize) {
    println!("............................................");
    println!(
        "{f}...................{s}x{s}....................{f}",
        s = size,
        f = if size < 10 { "." } else { "" }
    );

    let start = Instant::now();
    let solution = solve(size).unwrap();
    let duration = start.elapsed();

    println!("... Took: {:?}", duration);
    println!("............................................");

    if let Some(solution) = solution {
        for line in solution {
            let line: String = line
                .into_iter()
                .map(|d| char::from_digit(d, 10).unwrap())
                .collect();
            println!("{}", line);
        }
    } else {
        println!("No solution found!");
    }
}
