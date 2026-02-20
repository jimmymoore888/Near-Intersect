# Near-Intersect Diagnostic Graph v0
## Full System Node Map + Control Flow + Trust Boundaries

This document defines the canonical diagnostic graph for Near-Intersect.

Purpose:
- expose every control node
- define trust boundaries
- map execution flow
- identify invariant gates
- surface audit points
- establish architectural observability

This is NOT marketing.
This is system anatomy.

---

# 0. Graph Philosophy

Near-Intersect is modeled as:

CONTROL PLANE (authority + admission)
DATA PLANE (execution + runtime)
OBSERVABILITY PLANE (events + audit)

LAW always precedes execution.

---

# 1. Canonical Node List

---

## N0 — External Actor (User / Builder)

Entry point.

Capabilities:
- submit LAW profile
- select modules
- request deployment

Trust Level: UNTRUSTED

---

## N1 — LAW Profile Input

Artifacts:
- LAW_v1 schema
- LAW_v1 invariants
- doctrine constraints

Produces:
- law_profile_hash

Trust Level: UNTRUSTED → VALIDATED

---

## N2 — Module Stack Definition

Artifacts:
- ordered module list
- module manifests

Produces:
- module_stack_hash

Trust Level: UNTRUSTED → VALIDATED

---

## N3 — Registry Lookup

Reads:
- ModuleRecord
- certification level
- ACTIVE status
- LAW compatibility
- invariant maps

Rejects if:
- module missing
- module inactive
- LAW incompatible

Trust Level: SYSTEM AUTHORITY

Plane: CONTROL

---

## N4 — LAW Validation Engine

Stages:
- schema validation
- integrity verification
- doctrine enforcement
- registry conformance
- enforcement matrix compile

Outputs:
- compiled_enforcement_hash
OR
- rejection code

Trust Level: SYSTEM AUTHORITY

Plane: CONTROL

---

## N5 — Enforcement Matrix Compiler

Maps:
LAW invariants →
compile gate
deploy gate
runtime assertions

Produces:
- enforcement_matrix

Trust Level: SYSTEM AUTHORITY

Plane: CONTROL

---

## N6 — Deployment Gate

Binary decision node.

ALLOW only if:
- LAW validated
- registry conformance passes
- enforcement matrix exists

Otherwise:
REJECT

Trust Level: SYSTEM AUTHORITY

Plane: CONTROL

---

## N7 — Asset Registry Writer

Creates:
- AssetRecord
- deterministic asset_id

Emits:
- AssetDeployed event

Trust Level: SYSTEM AUTHORITY

Plane: CONTROL

---

## N8 — Factory Contract Instantiation

Deploys:
- asset contract
- module wiring

Consumes:
- compiled enforcement
- module stack

Trust Level: SYSTEM AUTHORITY

Plane: DATA

---

## N9 — Runtime Invariant Layer

Active during asset lifecycle.

Responsibilities:
- assert invariants
- enforce bounds
- block illegal transitions

Trust Level: SYSTEM AUTHORITY

Plane: DATA

---

## N10 — Treasury / Governance Execution

Handles:
- treasury flows
- governance actions
- module runtime calls

Subject to:
- runtime assertions
- invariant guards

Trust Level: PARTIAL (bounded by invariants)

Plane: DATA

---

## N11 — Observability Layer

Receives:
- ModuleSubmitted
- LawValidationStarted
- LawValidationPassed
- LawValidationRejected
- AssetDeployed
- DeploymentRejected
- Runtime events

Trust Level: READ ONLY

Plane: AUDIT


## N12 — OIM Runtime Governor (Module 8)

Role:
- Policy governor for treasury/savings allocation targeting positive real return over time
- Operates ONLY within declared bucket caps and LAW constraints

Modes:
- ORACLE (inflation index posted by authorized oracle; staleness => freeze)
- FIXED_HURDLE (genesis-fixed hurdle; no oracle)

Hard Boundaries (Non-Negotiable):
- MUST NOT mint/burn/modify supply (INV-OIM-01)
- MUST NOT alter timelocks/vesting/unlock windows (INV-OIM-02)
- MUST NOT bypass LAW validation or deployment gates
- MUST NOT move funds outside Safety <-> Growth rebalance bounds
- Oracle role MUST NOT move funds (INV-OIM-04)

Inputs:
- OimConfig (mode, caps, cooldown, staleness window)
- OimState (last_index, last_rebalance_at, score, status)
- Deterministic accounting signals (vault shares, stable-denominated balances)

Outputs:
- OIM status updates (HEALTHY / BEHIND / ORACLE_STALE)
- Bounded allocation shift Safety <-> Growth (if permitted)
- Audit events: OIM_INDEX_POSTED, OIM_STATUS_UPDATED, OIM_REBALANCED

Trust Level: PARTIAL (bounded by invariants + governance gate)
Plane: DATA (governed by CONTROL decisions)
---

# 2. Trust Boundaries

---

Boundary A:
External Actor → LAW / Module Input

Boundary B:
LAW + Modules → Registry + Validation

Boundary C:
Control Plane → Data Plane

Boundary D:
Runtime → Observability

No boundary may be bypassed.

---

# 3. Execution Flow (Happy Path)

N0 → N1 → N2  
N2 → N3  
N3 → N4  
N4 → N5  
N5 → N6 (ALLOW)  
N6 → N7  
N7 → N8  
N8 → N9  
N9 → N10  
ALL → N11  

---

# 4. Rejection Flow

Any failure at:

N3  
N4  
N6  

Routes to:

DeploymentRejected → N11  

No partial deploy permitted.

---

# 5. Control Plane Nodes

Authoritative:

- N3 Registry Lookup
- N4 LAW Validation
- N5 Enforcement Compiler
- N6 Deployment Gate
- N7 Registry Writer

These define sovereignty.

---

# 6. Data Plane Nodes

Executable:

- N8 Factory Instantiation
- N9 Runtime Invariants
- N10 Treasury / Governance

These never operate without Control Plane approval.

---

# 7. Audit Surfaces

Every critical transition MUST emit events:

- module admission
- law validation start/end
- deployment allow/reject
- runtime invariant violation

Audit graph reconstructs full history.

---

# 8. Attack Surface Map (Minimum)

Critical nodes:

- N3 Registry
- N4 LAW Validation
- N6 Deployment Gate
- N9 Runtime Assertions

Hardening priority must follow this order.

---

# 9. Design Truth

Near-Intersect power lives in:

Registry + LAW Validation + Deployment Gate.

Everything else is execution.

If these nodes hold, the system holds.

---

# 10. Diagnostic Intent

This graph exists so that:

- auditors can trace authority
- developers can understand flow
- AI systems can analyze topology
- governance cannot drift silently

This is constitutional infrastructure.

---

END DIAGNOSTIC_GRAPH_v0
