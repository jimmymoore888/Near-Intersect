# OIM Binding to LAW + Other Modules

## LAW Binding (Hard)

OIM is subordinate to LAW_v1_SCHEMA and LAW_v1_INVARIANTS.

Fixed supply and timelock determinism are absolute.

## Treasury Binding (Soft)

OIM operates only inside treasury/savings allowed by:

- Module 7: PercentageDistribution
- Module 6: LiquidityBoost
- Vault primitives

OIM may shift Safety <-> Growth only.

## Oracle Boundary

Oracle posts inflation only.
No rebalance authority.

## Factory Integration

Factory create_token() accepts optional oim_config.

Factory records:
- oim_enabled
- oim_mode

Factory v0 wires config only.
Runtime logic executes inside asset contract.
