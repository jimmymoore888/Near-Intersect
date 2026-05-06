# Module 13: Treasury Performance Index — TPI

## Canonical Status

Treasury Performance Index, also called TPI, is a doctrine and measurement module for Near Intersect Volcano.

TPI is designed to measure treasury performance, constrain treasury deployment risk, and produce auditable performance status over time.

TPI does not guarantee profit.

TPI does not guarantee treasury growth.

TPI does not guarantee that any eruption, distribution, savings result, token value, or purchasing-power outcome will occur.

TPI is not live as production financial enforcement unless and until the matching Rust contract implements it, the WASM is rebuilt from that exact source, and the deployed contract is publicly verified.

## Purpose

The purpose of TPI is to prevent treasury behavior from becoming blind, discretionary, or unmeasured.

TPI gives the system a rules-based way to ask:

- Was treasury capital deployed?
- Was treasury capital returned?
- Was the return efficient?
- Is performance improving or weakening?
- Should deployment power remain full, be reduced, or be frozen?

TPI is a measurement and constraint layer.

It is not a profit engine.

It is not investment advice.

It is not a guarantee of positive return.

## Relationship to Canonical Pre-Launch Law

TPI is subordinate to the Canonical Pre-Launch Law.

TPI must not override:

- 5.00% total system fee law
- treasury routing law
- volcano pressure law
- reserve law
- OIM law
- phase milestone law
- 2-year production lock
- 7-day exit window
- no-guarantee legal rule
- audit rule
- public verification rule

## Treasury Fee Context

Near Intersect Volcano uses a 5.00% total system fee.

Canonical split:

- 1.40% Treasury
- 0.90% Growth
- 1.05% Volcano Pressure
- 0.55% Permanent Reserve
- 1.10% Core Ops / R&D

TPI mainly observes and constrains the Treasury, Growth, and Reserve side of the system.

TPI must not change the canonical fee split unless a future audited governance process explicitly approves and publishes a new law.

## TPI Does Not

TPI does not:

- mint tokens
- burn tokens
- modify total supply
- change fee percentages
- alter eruption milestones
- manually trigger eruptions
- alter 2-year production locks
- alter 7-day exit windows
- create rights for local watcher profiles
- guarantee yield
- guarantee profit
- guarantee treasury growth
- perform discretionary trading
- bypass audit or verification rules

## TPI Does

TPI may:

- record treasury deployment events
- record treasury return events
- compute treasury efficiency
- compute treasury trend
- classify treasury status
- reduce future deployment power when performance is weak
- recommend freeze status when performance is unsafe
- emit auditable events
- expose read-only treasury performance views

## Core Measurement Model

TPI measures performance using deterministic accounting.

The basic model is:

```text
efficiency_bps = returned_amount * 10,000 / deployed_amount
