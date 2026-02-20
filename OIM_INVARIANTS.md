# OIM — Non-Negotiable Invariants (LAW-Compatible)

INV-OIM-01: Supply Immutability  
OIM MUST NOT mint, burn, or modify total supply.

INV-OIM-02: Timelock Immutability  
OIM MUST NOT alter unlock windows, vesting, or timelocks.

INV-OIM-03: Bounded Rebalance  
Rebalances MUST stay within bucket caps and optional max_delta_bps.

INV-OIM-04: Oracle Cannot Move Funds  
Oracle may ONLY post inflation data.

INV-OIM-05: Oracle Staleness Fail-Safe  
If data is stale beyond max_age_sec → ORACLE_STALE → rebalance frozen.

INV-OIM-06: Cooldown Enforcement  
oim_rebalance() MUST enforce rebalance_cooldown_sec.

INV-OIM-07: Deterministic Accounting  
Real-return scoring MUST use on-chain accounting only.

INV-OIM-08: Governance Boundary  
Only governance/multisig configures OIM at genesis.
Post-genesis mutation forbidden or strictly logged.
