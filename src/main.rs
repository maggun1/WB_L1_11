use std::collections::HashMap;

fn main() {
    let temperatures: Vec<f64> = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    let mut temperature_ranges: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    for &temp in &temperatures {
        let lower_bound = (temp / 10.0).floor() as i32 * 10;
        let upper_bound = lower_bound + 10;

        temperature_ranges
            .entry((lower_bound, upper_bound))
            .or_insert_with(Vec::new)
            .push(temp);
    }

    for (range, temps) in &temperature_ranges {
        println!("Interval [{}, {}): {:?}", range.0, range.1, temps);
    }
}
