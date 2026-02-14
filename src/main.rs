use comfy_table::Table;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Step {
    source: f64,
    target: f64,
    #[serde(default)]
    extras: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
    game: String,
    source: String,
    target: String,
    steps: Vec<Step>,
}

struct CalculatedStep {
    pay: f64,
    get: f64,
    base: f64,
    bonus: f64,
    rate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new("data");

    if !data_dir.exists() {
        println!("Data directory 'data/' not found.");
        return Ok(());
    }

    println!("Loading game data from {:?}...\n", data_dir);

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<GameConfig>(&content) {
                Ok(config) => process_game(config),
                Err(e) => println!("Failed to parse {:?}: {}", path, e),
            }
        }
    }

    Ok(())
}

fn process_game(config: GameConfig) {
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
