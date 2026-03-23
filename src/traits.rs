pub trait HftReward {
    fn sync_bonus(&self) -> f32;
    fn price_reflex(&self) -> f32;
    fn thermal_pain(&self) -> f32;
    fn mining_efficiency_bonus(&self) -> f32;  // NEW
}

/// Default implementation for testing and fallback scenarios
#[derive(Debug, Clone, Default)]
pub struct DefaultHftReward {
    sync_bonus_value: f32,
    price_reflex_value: f32,
    thermal_pain_value: f32,
    mining_efficiency_value: f32,
}

impl DefaultHftReward {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create with specific reward values
    pub fn with_values(sync_bonus: f32, price_reflex: f32, thermal_pain: f32, mining_efficiency: f32) -> Self {
        Self {
            sync_bonus_value: sync_bonus.clamp(0.0, 1.0),
            price_reflex_value: price_reflex.clamp(0.0, 1.0),
            thermal_pain_value: thermal_pain.clamp(0.0, 1.0),
            mining_efficiency_value: mining_efficiency.clamp(0.0, 1.0),
        }
    }
}

impl HftReward for DefaultHftReward {
    fn sync_bonus(&self) -> f32 {
        self.sync_bonus_value
    }
    
    fn price_reflex(&self) -> f32 {
        self.price_reflex_value
    }
    
    fn thermal_pain(&self) -> f32 {
        self.thermal_pain_value
    }
    
    fn mining_efficiency_bonus(&self) -> f32 {
        self.mining_efficiency_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hft_reward() {
        let reward = DefaultHftReward::with_values(0.8, 0.6, 0.2, 0.7);
        
        assert_eq!(reward.sync_bonus(), 0.8);
        assert_eq!(reward.price_reflex(), 0.6);
        assert_eq!(reward.thermal_pain(), 0.2);
        assert_eq!(reward.mining_efficiency_bonus(), 0.7);
    }

    #[test]
    fn test_hft_reward_clamping() {
        let reward = DefaultHftReward::with_values(1.5, -0.5, 2.0, -1.0);
        
        assert_eq!(reward.sync_bonus(), 1.0);  // Clamped to 1.0
        assert_eq!(reward.price_reflex(), 0.0); // Clamped to 0.0
        assert_eq!(reward.thermal_pain(), 1.0); // Clamped to 1.0
        assert_eq!(reward.mining_efficiency_bonus(), 0.0); // Clamped to 0.0
    }
}
