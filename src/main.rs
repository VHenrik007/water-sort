use water_sort::game_elements::{color::Color, glass::Glass};
use water_sort::{
    solver::solver::Solver,
    game_elements::glass_system::{GlassSystem, GlassSystemResult}
};

fn main() -> GlassSystemResult<()> {
    let glass1 = Glass::new();
    let glass2 = Glass::new();
    let glass3 = Glass::from_colors(&[Color::BLUE, Color::BLUE, Color::GREEN, Color::GREEN])?;
    let glass4 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::GREEN, Color::GREEN])?;
    let glass5 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::YELLOW, Color::YELLOW])?;
    let glass6 = Glass::from_colors(&[Color::YELLOW, Color::YELLOW, Color::RED, Color::RED])?;

    let pre_system = &[glass1, glass2, glass3, glass4, glass5, glass6];

    let system = GlassSystem::new(pre_system.to_vec());
    system.print_system_state();

    println!("Solving...");
    let solver = Solver {};
    let solution_steps = solver.solve(&system)?;
    let solved_system = solver.validate_solution(system, &solution_steps)?;
    if solved_system.is_solved() {
        println!("Solved in {} steps", solution_steps.len());
    } else {
        println!("Incomplete solution!")
    }
    solved_system.print_system_state();

    Ok(())
}
