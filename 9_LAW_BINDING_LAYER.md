# MODULE BINDINGS — LAW v1

Purpose:
Bind canonical LAW v1 sections to concrete contract modules.
No module may operate outside its assigned law scope.

---

## SECTION 1 — IDENTITY

Bound Module:
- contract/src/lib.rs (root initializer)

Responsibilities:
- Store immutable asset_name
- Store immutable ticker
- Enforce NEAR network binding

---

## SECTION 2 — SUPPLY LAW

### FIXED SUPPLY
Bound Module:
- Module 1: FixedSupply

Invariants Enforced:
- I-2.1
- I-2.2
- I-2.3

---

### CAPPED SUPPLY
Bound Module:
- Module 2: BurnCap

Invariants Enforced:
- I-2.4
- I-2.6

---

### MINTABLE SUPPLY (ADVANCED)
Bound Module:
- Governance wrapper (if enabled)

Invariants Enforced:
- I-2.5

---

## SECTION 3 — SAFETY SYSTEMS (BURN)

Bound Module:
- Module 2: BurnCap

Invariants Enforced:
- I-3.1
- I-3.2
- I-3.3
- I-3.4
- I-3.5

---

## SECTION 4 — SAFETY SYSTEMS (TIME LOCK)

Bound Module:
- Module 3: TimeLock

Invariants Enforced:
- I-4.1
- I-4.2
- I-4.3
- I-4.4
- I-4.5

---

## SECTION 5 — DISTRIBUTION

### LIQUIDITY
Bound Module:
- Module 6: LiquidityBootstrap

### AIRDROP
Bound Module:
- Module 4: Airdrop

### SAVINGS / VESTING
Bound Module:
- Module 5: VestingSchedule

Coordinator:
- Module 7: PercentageDistribution

Invariants Enforced:
- I-5.1
- I-5.2
- I-5.3
- I-5.4
- I-5.5

---

## SECTION 6 — LAW TEXT

Bound Component:
- Schema renderer (off-chain)

Invariants Enforced:
- I-6.1
- I-6.2
- I-6.3

---

## GLOBAL GUARANTEE

No module may:
- Mint supply
- Burn supply
- Transfer locked funds
- Reallocate distribution

Unless explicitly authorized by LAW v1 schema and invariants.

## Module 8: OutpaceInflation (OIM)
- Purpose: Real-return oriented treasury/savings allocation policy (inflation defense)
- Depends on: LAW_v1_INVARIANTS, Module 7 (PercentageDistribution), treasury/savings vault primitives
- Prohibits: supply changes, timelock changes, discretionary trading
- Modes: ORACLE (off-chain CPI feed) or FIXED_HURDLE (genesis-locked)
- Audit Events: OIM_INDEX_POSTED, OIM_STATUS_UPDATED, OIM_REBALANCED
