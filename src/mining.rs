#[derive(Debug, Clone, Copy, Default)]
pub struct MiningReward {
    ema_reward: f32,
}

impl MiningReward {
    pub fn new() -> Self { Self { ema_reward: 0.0 } }

    pub fn compute(&mut self, hashrate: f32, power: f32, temp_c: f32) -> f32 {
        let efficiency = (hashrate / 1000.0).clamp(0.0, 1.0);
        let thermal_stress = ((temp_c - 65.0).max(0.0) / 20.0).clamp(0.0, 1.0);
        let energy_waste = (power / 350.0).clamp(0.0, 1.0);

        let raw = 0.7 * efficiency - 0.2 * thermal_stress - 0.1 * energy_waste;
        self.ema_reward = 0.3 * raw + 0.7 * self.ema_reward;
        self.ema_reward.clamp(0.0, 1.0)
    }
}
