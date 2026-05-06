# Module 8: OutpaceInflation — OIM

## Canonical Status

OutpaceInflation, also called OIM, is a permanent doctrine module for Near Intersect Volcano.

OIM is designed to support purchasing-power protection over time through deterministic treasury and savings-positioning rules.

OIM is not a profit guarantee.

OIM is not live as production financial enforcement unless and until the matching Rust contract implements it, the WASM is rebuilt from that exact source, and the deployed contract is publicly verified.

## Purpose

OIM governs treasury and savings positioning with the goal of targeting positive real return over time.

In plain language, OIM exists to help the system think about inflation pressure without changing the token supply, breaking timelocks, or making promises of yield.

OIM must remain subordinate to the Canonical Pre-Launch Law.

## OIM Does Not

OIM does not:

- mint tokens
- inflate token supply
- burn tokens unless another approved module already governs burn behavior
- modify total supply
- alter deterministic unlock windows
- alter vesting schedules
- alter timelock state
- promise yield
- promise profit
- guarantee purchasing-power protection
- run discretionary trading strategies
- use market-price feeds for timing trades
- create financial rights for local watcher profiles

## OIM Does

OIM may:

- define an inflation hurdle
- use oracle mode or fixed-hurdle mode
- compute a real-return score from deterministic accounting
- classify system status as healthy, behind, or oracle-stale
- adjust treasury allocation between approved Safety and Growth buckets
- enforce bucket caps
- enforce rebalance cooldowns
- freeze when oracle data is stale
- emit auditable events
- expose read-only OIM status methods

## Modes

### 1. Oracle Mode

In Oracle Mode, an authorized oracle account may post inflation-index data.

The oracle may only post data.

The oracle cannot move funds, rebalance funds, trigger distributions, alter timelocks, or change supply.

If oracle data becomes stale beyond the configured maximum age, OIM must enter an oracle-stale state and freeze rebalancing.

### 2. Fixed Hurdle Mode

In Fixed Hurdle Mode, no oracle is required.

A fixed annual inflation hurdle is set at genesis and does not change after deployment unless a future governance process is explicitly approved, audited, and publicly documented.

Fixed Hurdle Mode is simpler and safer for early launch because it reduces oracle risk.

## Deterministic Buckets

Treasury and savings positions may be organized into deterministic buckets.

Allowed bucket categories:

- Safety bucket
- Growth bucket
- Liquidity bucket
- Reserve bucket

OIM may only shift allocation between Safety and Growth if the contract permits that behavior and all caps, cooldowns, and safety rails are satisfied.

Burn and airdrop behavior must remain governed by their own modules and must not be controlled directly by OIM.

## Non-Negotiable Invariants

### INV-OIM-01: Supply Immutability

OIM must not mint, burn, or otherwise modify total token supply.

### INV-OIM-02: Timelock Immutability

OIM must not alter any deterministic unlock window, vesting schedule, or timelock state.

### INV-OIM-03: Bounded Rebalance

Any rebalance must remain within declared bucket caps.

Any rebalance should also respect a maximum step size per rebalance.

### INV-OIM-04: Oracle Cannot Move Funds

The oracle role, if enabled, may only post inflation-index data.

The oracle must not be able to trigger allocation changes, withdrawals, transfers, claims, or distributions.

### INV-OIM-05: Oracle Staleness Fail-Safe

If oracle inflation data is stale beyond the configured maximum age, OIM status must become ORACLE_STALE.

In ORACLE_STALE status, rebalance must freeze, revert, or no-op safely.

### INV-OIM-06: Cooldown Enforcement

OIM rebalance logic must enforce a cooldown between successful rebalances.

### INV-OIM-07: Deterministic Accounting

Real-return score calculations must use deterministic accounting.

Accepted inputs may include vault share deltas, stable-denominated balances, and contract-recorded treasury accounting.

Market-price-based timing is not permitted in v1.

### INV-OIM-08: Governance Boundary

Only approved governance, multisig, or contract-defined authority may initialize or control OIM configuration.

Post-genesis config changes should be forbidden unless the change is explicitly documented, audited, logged, and publicly disclosed.

## Interface v1

### InflationIndex

Fields:

- index_id
- period
- value_bps
- posted_at

### OimMode

Allowed modes:

- ORACLE
- FIXED_HURDLE

### OimStatus

Allowed statuses:

- HEALTHY
- BEHIND
- ORACLE_STALE

### OimConfig

Recommended fields:

- mode
- oracle_account
- fixed_hurdle_bps_annual
- measurement_window_days
- min_real_return_bps
- rebalance_cooldown_sec
- max_oracle_age_sec
- safety_cap_bps
- growth_cap_bps
- liquidity_cap_bps
- max_rebalance_step_bps

### OimState

Recommended fields:

- last_index
- last_rebalance_at
- real_return_score_bps
- status

## View Methods

Recommended read-only methods:

- get_oim_config()
- get_oim_state()

## Oracle Method

Recommended oracle method:

- post_inflation_index(index_id, period, value_bps)

Rules:

- predecessor must equal oracle_account
- method must update latest inflation index
- method must record posted_at timestamp
- method must emit OIM_INDEX_POSTED
- method must not move funds

## Control Method

Recommended control method:

- oim_rebalance()

Rules:

- must enforce rebalance cooldown
- must verify oracle freshness in Oracle Mode
- must compute deterministic real-return score
- must set OIM status
- may rebalance only within caps
- may shift allocation only within permitted buckets
- must emit status and rebalance events
- must not alter supply
- must not alter timelocks

## Events

Recommended events:

- OIM_INDEX_POSTED
- OIM_STATUS_UPDATED
- OIM_REBALANCED
- OIM_ORACLE_STALE

Events must be structured enough for public verification, audits, and indexers.

## Binding to LAW

OIM is subordinate to the Canonical Pre-Launch Law and all approved LAW invariants.

OIM must not weaken or override:

- fixed supply law
- timelock law
- vesting law
- distribution law
- treasury law
- audit law
- legal disclaimer law

## Binding to Treasury

OIM may operate only inside the treasury and savings portion permitted by the production contract.

OIM must not control user funds outside the approved contract rules.

OIM must not create a promise of profit or guaranteed return.

## Binding to Local Profiles

Local watcher profiles are awareness records only.

Local profiles do not create token rights, treasury rights, eruption rights, OIM rights, or guaranteed future allocations.

Only eligible active wallet participants recorded on-chain may become eligible for future on-chain financial participation.

## Pre-Launch Implementation Status

OIM is permanent doctrine.

OIM must be implemented in Rust before any public fund-taking launch.

Until the Rust contract proves OIM runtime enforcement, the correct status is:

**Specified / Pending Runtime Verification**

## Launch Rule

Near Intersect Volcano must not publicly claim OIM is live unless all of the following are true:

1. OIM logic exists in `contract/src/lib.rs`.
2. The WASM is rebuilt from that exact Rust source.
3. The deployed contract account is published.
4. The WASM hash is published.
5. The source commit hash is published.
6. Explorer links are published.
7. Audit and verification status are disclosed.
8. The website and README match this file.

## Legal Disclaimer

OIM is not investment advice.

OIM does not guarantee returns.

OIM does not guarantee profit.

OIM does not guarantee that any eruption, treasury level, token value, savings outcome, or purchasing-power result will occur.

OIM is a rules-based design component that must be implemented, audited, verified, and legally reviewed before any public fund-taking launch.	
