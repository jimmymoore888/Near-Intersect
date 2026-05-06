	1.	README.md
# Module 8: OutpaceInflation (OIM)
- Module 8: OutpaceInflation (real-return treasury/savings policy; oracle or fixed hurdle)
## Purpose
OutpaceInflation (OIM) is a policy module that governs treasury/savings positioning to target positive real return over time (i.e., outpace inflation), without modifying token supply or violating LAW invariants.

OIM does NOT:
- mint, inflate, or modify supply
- change deterministic unlock windows / timelock schedules
- promise yield or run discretionary strategies
- rely on price feeds for market timing

OIM DOES:
- define an inflation hurdle (oracle-fed or fixed at genesis)
- compute a real-return score from deterministic accounting
- adjust treasury allocation between Safety and Growth buckets within hard caps
- fail-safe to freeze when inflation data is stale (oracle mode)

## Modes
1) Oracle Mode:
- An authorized oracle account posts an inflation measure (bps) per period.
- OIM enforces freshness (staleness => freeze rebalance).

2) Fixed Hurdle Mode:
- No oracle required.
- A fixed annual inflation hurdle (bps) is set at genesis and never changes.

## Deterministic Buckets
Treasury/savings funds are constrained to buckets with caps:
- Safety bucket (stables / low-vol parking)
- Growth bucket (allowed strategies, capped)
- Liquidity bucket (LP/support, capped)
- Burn/Airdrop buckets remain governed by their existing modules
OIM may only shift allocation between Safety <-> Growth, bounded by caps and cooldown.

2.	OIM_INVARIANTS.txt
OIM — Non-Negotiable Invariants (LAW-Compatible)

INV-OIM-01: Supply Immutability
- OIM MUST NOT mint, burn, or otherwise modify token total supply.

INV-OIM-02: Timelock Immutability
- OIM MUST NOT alter any deterministic unlock window, vesting schedule, or timelock state.

INV-OIM-03: Bounded Rebalance
- Any rebalance MUST remain within pre-declared bucket caps.
- Rebalance MUST NOT exceed max_delta_bps per rebalance (optional, recommended).

INV-OIM-04: Oracle Cannot Move Funds
- Oracle role (if enabled) may ONLY post inflation index data.
- Oracle role MUST NOT be able to trigger allocation changes, withdrawals, or transfers.

INV-OIM-05: Oracle Staleness Fail-Safe
- If oracle inflation data is stale beyond max_age_sec, OIM status MUST become ORACLE_STALE.
- In ORACLE_STALE state, oim_rebalance() MUST revert or no-op (freeze).

INV-OIM-06: Cooldown Enforcement
- oim_rebalance() MUST enforce rebalance_cooldown_sec between successful rebalances.

INV-OIM-07: Deterministic Accounting
- Real-return score calculations MUST use deterministic on-chain accounting (vault share deltas, stable-denominated balances).
- No market-price-based scoring is permitted in v1.

INV-OIM-08: Governance Boundary
- Only governance/multisig may enable/disable OIM or set initial config at genesis.
- Post-genesis, config changes MUST be either forbidden (preferred) or strictly limited and logged (if allowed).

3.	OIM_INTERFACE.md
# OIM Interface (v1)

## Types

### InflationIndex
- index_id: String (e.g., "US_CPI_U")
- period: String (e.g., "2026-01")
- value_bps: u32  (inflation for the period in basis points, or index-level bps — pick one convention and keep it)
- posted_at: u64  (block timestamp)

### OimMode
- ORACLE
- FIXED_HURDLE

### OimConfig
- mode: OimMode
- oracle_account: Option<AccountId>          (required iff mode == ORACLE)
- fixed_hurdle_bps_annual: u32              (required iff mode == FIXED_HURDLE)
- measurement_window_days: u16              (recommended: 365)
- min_real_return_bps: i32                  (recommended: +200 bps)
- rebalance_cooldown_sec: u32               (recommended: 86400)
- max_oracle_age_sec: u32                   (recommended: 45 days in seconds)
- safety_cap_bps: u32                       (bucket cap)
- growth_cap_bps: u32                       (bucket cap)
- liquidity_cap_bps: u32                    (bucket cap)
- max_rebalance_step_bps: Option<u32>       (recommended safety rail)

### OimStatus
- HEALTHY
- BEHIND
- ORACLE_STALE

### OimState
- last_index: Option<InflationIndex>
- last_rebalance_at: u64
- real_return_score_bps: i32
- status: OimStatus

---

## View Methods (read-only)
- get_oim_config() -> OimConfig
- get_oim_state()  -> OimState

---

## Oracle Method (ORACLE mode only)
- post_inflation_index(index_id: String, period: String, value_bps: u32)

Rules:
- MUST assert predecessor == oracle_account
- MUST update last_index and posted_at
- MUST emit OIM_INDEX_POSTED

---

## Control Plane (governance/multisig)
- oim_rebalance()

Rules:
- MUST enforce rebalance_cooldown_sec
- IF mode == ORACLE:
  - MUST require last_index exists
  - MUST require (now - last_index.posted_at) <= max_oracle_age_sec
  - else status=ORACLE_STALE and freeze
- MUST compute real_return_score_bps and status
- IF status == BEHIND:
  - MAY shift allocation toward Growth within caps and step limit
- MUST emit OIM_STATUS_UPDATED and OIM_REBALANCED

---

## Events (auditability)
- OIM_INDEX_POSTED { index_id, period, value_bps, posted_at }
- OIM_STATUS_UPDATED { status, real_return_score_bps, at }
- OIM_REBALANCED { from_safety_bps, to_growth_bps, at }

4.	OIM_BINDING.md
# OIM Binding to LAW + Other Modules

## Binding to LAW (Hard)
OIM is subordinate to LAW_v1_SCHEMA and LAW_v1_INVARIANTS.
OIM MUST NOT weaken or override any LAW invariant.

Relevant LAW constraints:
- Fixed supply (Module 1) is absolute.
- Timelock / vesting determinism (Modules 3 & 5) is absolute.

## Binding to Treasury Distribution (Soft)
OIM operates ONLY within the treasury/savings portion permitted by:
- Module 7: PercentageDistribution
- Module 6: LiquidityBoost (if defined)
- Any savings vault / treasury vault primitives in the contract layer

OIM may only shift between:
- Safety bucket <-> Growth bucket
Subject to:
- bucket caps
- step caps (if enabled)
- cooldown
- oracle staleness rules (oracle mode)

## Oracle Boundary
Oracle account can post inflation index data ONLY.
Oracle has no authority to rebalance or transfer funds.

## Factory Integration (Near-Intersect v0)
Factory create_token() may accept optional oim_config.
Factory records:
- oim_enabled: bool
- oim_mode: ORACLE | FIXED_HURDLE

Factory v0 does not execute OIM logic; it wires config at instantiation and exposes read methods.
