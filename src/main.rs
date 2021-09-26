mod distinct_powers;
use distinct_powers::DistinctPowers;

fn main() {
    println!("Calculation started");
    let mut dp = DistinctPowers::new(100, 100);

    let answer_p29: usize = dp.number_of_distinct_terms();

    println!(
        "The answer to problem 29 of project Euler is {}.",
        answer_p29
    );
}
