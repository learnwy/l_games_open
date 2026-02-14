use comfy_table::Table;
use crate::recharge::models::{GameConfig, CalculatedStep};

pub fn process_game(config: GameConfig) {
    println!("Game: {}", config.game);
    println!("Exchange: {} -> {}", config.source, config.target);

    let mut results: Vec<CalculatedStep> = config
        .steps
        .iter()
        .map(|step| {
            let extra_sum: f64 = step.extras.iter().sum();
            let total_target = step.target + extra_sum;
            let rate = if step.source != 0.0 {
                total_target / step.source
            } else {
                0.0
            };

            CalculatedStep {
                pay: step.source,
                get: total_target,
                base: step.target,
                bonus: extra_sum,
                rate,
            }
        })
        .collect();

    // Sort by rate descending
    results.sort_by(|a, b| b.rate.partial_cmp(&a.rate).unwrap());

    // Create table
    let mut table = Table::new();
    table.set_header(vec![
        format!("Pay ({})", config.source),
        format!("Get ({})", config.target),
        "Base".to_string(),
        "Bonus".to_string(),
        format!("Rate ({}/{})", config.target, config.source),
    ]);

    for step in results {
        table.add_row(vec![
            format!("{:.2}", step.pay),
            format!("{:.0}", step.get),
            format!("{:.0}", step.base),
            format!("{:.0}", step.bonus),
            format!("{:.2}", step.rate),
        ]);
    }

    println!("{table}");
    println!("--------------------------------------------------");
}
