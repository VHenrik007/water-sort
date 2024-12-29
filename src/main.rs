use water_sort::glass::{glass::Glass, color::Color};
use water_sort::solver::solver::{Solver, SolverResult};

fn main() -> SolverResult<()>{
    let glass1 = Glass::new();
    let glass2 = Glass::new();
    let glass3 = Glass::from_colors(&[Color::BLUE, Color::BLUE, Color::GREEN, Color::GREEN])?;
    let glass4 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::GREEN, Color::GREEN])?;
    let glass5 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::YELLOW, Color::YELLOW])?;
    let glass6 = Glass::from_colors(&[Color::YELLOW, Color::YELLOW, Color::RED, Color::RED])?;

    let pre_system = &[
        glass1, glass2, glass3, glass4, glass5, glass6
    ];

    let mut solver = Solver::new(pre_system.to_vec());

    solver.print_system_state();

    solver.try_pour(2, 0)?;
    solver.try_pour(3, 0)?;
    solver.try_pour(4, 1)?;
    solver.try_pour(5, 4)?;
    solver.try_pour(5, 1)?;
    solver.try_pour(4, 5)?;
    solver.try_pour(4, 2)?;
    solver.try_pour(3, 5)?;
    solver.try_pour(3, 2)?;

    solver.print_system_state();
    println!("{}", solver.is_solved());

    Ok(())
}
