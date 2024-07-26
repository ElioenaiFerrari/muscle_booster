use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TrainingType {
    #[serde(rename(serialize = "gym", deserialize = "gym"))]
    Gym,
    #[serde(rename(serialize = "home", deserialize = "home"))]
    Hybrid,
    #[serde(rename(serialize = "hybrid", deserialize = "hybrid"))]
    Home,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HoursPerDay {
    #[serde(rename(serialize = "less_than_one", deserialize = "less_than_one"))]
    LessThanOne,
    #[serde(rename(serialize = "one", deserialize = "one"))]
    One,
    #[serde(rename(serialize = "two", deserialize = "two"))]
    Two,
    #[serde(rename(serialize = "three", deserialize = "three"))]
    Three,
    #[serde(rename(serialize = "four", deserialize = "four"))]
    Four,
    #[serde(rename(serialize = "more_than_four", deserialize = "more_than_four"))]
    MoreThanFour,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MuscleGoal {
    #[serde(rename(serialize = "gain", deserialize = "gain"))]
    Gain,
    #[serde(rename(serialize = "maintain", deserialize = "maintain"))]
    Maintain,
    #[serde(rename(serialize = "lose", deserialize = "lose"))]
    Lose,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Motivation {
    #[serde(rename(serialize = "healthy", deserialize = "healthy"))]
    Healthy,
    #[serde(rename(serialize = "aesthetic", deserialize = "aesthetic"))]
    Aesthetic,
    #[serde(rename(serialize = "strength", deserialize = "strength"))]
    Strength,
    #[serde(rename(serialize = "flexibility", deserialize = "flexibility"))]
    Flexibility,
    #[serde(rename(serialize = "endurance", deserialize = "endurance"))]
    Endurance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlanDto {
    pub height: f64,
    pub weight: f64,
    pub weight_goal: f64,
    pub training_type: TrainingType,
    pub hours_per_day: HoursPerDay,
    pub muscle_goal: MuscleGoal,
    pub motivations: Vec<Motivation>,
}

impl CreatePlanDto {
    pub fn imc(&self) -> f64 {
        self.weight / self.height.powi(2)
    }

    pub fn category(&self) -> &'static str {
        let imc = self.imc();
        if imc < 18.5 {
            "underweight"
        } else if imc < 24.9 {
            "normal_weight"
        } else if imc < 29.9 {
            "overweight"
        } else {
            "obesity"
        }
    }
}
