pub const SURVIVAL_BONUS: f32 = 0.25;
pub const TARGET_BONUS: f32 = 0.5;

pub fn score(mass: f32, target: f32) -> f32 {
    if mass <= 0.0 {
        return 0.0;
    }
    mass + SURVIVAL_BONUS + if mass >= target { TARGET_BONUS } else { 0.0 }
}

pub fn mean(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f32>() / values.len() as f32
}

pub fn median(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    sorted[sorted.len() / 2]
}
