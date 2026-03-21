// module_13_treasury_performance.rs
// Treasury Performance Index (TPI)
// Ensures capital deployment remains productive over time

pub type Balance = u128;

#[derive(Clone, Debug)]
pub struct TreasuryAction {
    pub deployed: Balance,
    pub returned: Balance,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct PerformanceScore {
    pub efficiency: f64,
    pub trend: f64,
}

pub struct TreasuryPerformance {
    pub history: Vec<TreasuryAction>,
    pub min_efficiency_threshold: f64,
    pub max_window: usize,
}

impl TreasuryPerformance {
    pub fn new(min_efficiency_threshold: f64, max_window: usize) -> Self {
        Self {
            history: Vec::new(),
            min_efficiency_threshold,
            max_window,
        }
    }

    pub fn record(&mut self, action: TreasuryAction) {
        self.history.push(action);

        if self.history.len() > self.max_window {
            self.history.remove(0);
        }
    }

    /// Compute efficiency score
    pub fn compute_efficiency(&self) -> PerformanceScore {
        let mut total_deployed = 0.0;
        let mut total_returned = 0.0;

        for action in &self.history {
            total_deployed += action.deployed as f64;
            total_returned += action.returned as f64;
        }

        let efficiency = if total_deployed > 0.0 {
            total_returned / total_deployed
        } else {
            1.0
        };

        // Trend: recent vs older performance
        let mid = self.history.len() / 2;

        let (early, late) = self.history.split_at(mid);

        let early_avg = early.iter().map(|a| a.returned as f64).sum::<f64>() / (early.len().max(1) as f64);
        let late_avg = late.iter().map(|a| a.returned as f64).sum::<f64>() / (late.len().max(1) as f64);

        let trend = late_avg - early_avg;

        PerformanceScore { efficiency, trend }
    }

    /// Adjust treasury power
    pub fn adjustment_factor(&self, score: &PerformanceScore) -> f64 {
        if score.efficiency < self.min_efficiency_threshold {
            return 0.5; // cut deployment power
        }

        if score.trend < 0.0 {
            return 0.75; // weakening performance
        }

        1.0 // full power
    }
}
