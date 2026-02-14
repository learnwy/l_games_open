use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Step {
    pub source: f64,
    pub target: f64,
    #[serde(default)]
    pub extras: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub game: String,
    pub source: String,
    pub target: String,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct CalculatedStep {
    pub pay: f64,
    pub get: f64,
    pub base: f64,
    pub bonus: f64,
    pub rate: f64,
}
