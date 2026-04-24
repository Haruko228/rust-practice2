mod hackerrank {
    pub mod staircase;
    pub mod grading;
    pub mod apple_orange;
}

fn main() {
    println!("--- Running All Practicals ---");
    // Виклики, щоб не було warnings про "dead code"
    hackerrank::staircase::staircase(0);
    let _ = hackerrank::grading::grading_students(&[]);
    hackerrank::apple_orange::count_apples_and_oranges(0, 0, 0, 0, &[], &[]);
}
