// module_12_externality_proof.rs
// Externality Proof Layer (EPL)
// Validates whether capital is genuinely new to the system

use std::collections::{HashMap, HashSet};

pub type Balance = u128;
pub type Address = String;

#[derive(Clone, Debug)]
pub struct FlowRecord {
    pub source_chain: String,
    pub tx_id: String,
    pub sender: Address,
    pub amount: Balance,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct ExternalityScore {
    pub is_external: bool,
    pub confidence: f64,
    pub discounted_amount: Balance,
}

pub struct ExternalityProof {
    // Tracks seen transactions to prevent replay
    pub seen_txs: HashSet<String>,

    // Tracks capital lineage (to detect loops)
    pub flow_history: HashMap<Address, Vec<FlowRecord>>,

    // Known trusted external sources (bridges, fiat onramps)
    pub trusted_sources: HashSet<String>,

    // Loop detection sensitivity
    pub loop_threshold: usize,
}

impl ExternalityProof {
    pub fn new(trusted_sources: HashSet<String>, loop_threshold: usize) -> Self {
        Self {
            seen_txs: HashSet::new(),
            flow_history: HashMap::new(),
            trusted_sources,
            loop_threshold,
        }
    }

    /// Validate if incoming capital is external
    pub fn validate_inflow(&mut self, record: FlowRecord) -> ExternalityScore {
        // 1. Replay protection
        if self.seen_txs.contains(&record.tx_id) {
            return ExternalityScore {
                is_external: false,
                confidence: 0.0,
                discounted_amount: 0,
            };
        }

        self.seen_txs.insert(record.tx_id.clone());

        // 2. Source validation
        let trusted = self.trusted_sources.contains(&record.source_chain);

        // 3. Loop detection (simple lineage depth check)
        let history = self.flow_history.entry(record.sender.clone()).or_default();
        history.push(record.clone());

        let loop_detected = history.len() > self.loop_threshold;

        // 4. Compute confidence
        let mut confidence = if trusted { 1.0 } else { 0.5 };

        if loop_detected {
            confidence *= 0.3; // heavy penalty for recursion
        }

        // 5. Discounted value
        let discounted = (record.amount as f64 * confidence) as Balance;

        ExternalityScore {
            is_external: trusted && !loop_detected,
            confidence,
            discounted_amount: discounted,
        }
    }
}
