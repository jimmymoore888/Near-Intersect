# Near-Intersect Control Plane
## Registry + Certification + Deployment (RCD)

This document defines the **control plane** of Near-Intersect.

**Core premise:** Near-Intersect is not a “token generator.”
It is an **issuance authority** governed by RagTuff law profiles.
No asset or module is considered valid unless it passes the deterministic gates below.

---

## 1) Definitions

- **LAW Profile (LAW_v1):** Canonical declaration of invariants + module requirements + enforcement expectations.
- **Module:** A composable capability unit that can be attached to an asset profile.
- **Registry:** The canonical index of approved modules and deployed assets (by deterministic IDs).
- **Certification:** A deterministic trust/assurance level assigned to a module based on verification evidence.
- **Deployment Gate:** The final allow/deny decision for asset creation and module attachment.

---

## 2) Non-Negotiable Invariants (Control Plane)

### CP-1: Deterministic Admission
All module registration, certification state, and deployment allow/deny decisions MUST be deterministic.

### CP-2: No Bypass Paths
There MUST NOT exist an alternative path that deploys assets without passing:
- LAW profile validation, and
- module stack conformance checks.

### CP-3: Immutable Audit Trail
All registry actions MUST emit events sufficient to reconstruct:
- who submitted,
- what was submitted,
- what was approved/rejected,
- which law profile governed the decision.

### CP-4: Separation of Concerns
Registry/certification logic is control-plane authority.
Token runtime logic is data-plane execution.
Control plane MUST remain minimal and enforce only admission + conformance.

---

## 3) Registry Data Model (Minimum v0)

### 3.1 ModuleRecord
- `module_id`: deterministic ID (hash of code + manifest + publisher)
- `publisher_id`: account / authority
- `manifest_hash`: hash of module manifest
- `code_hash`: hash of module WASM/source (as applicable)
- `cert_level`: enum (COMMUNITY, AUDITED, RAGTUFF_CERTIFIED, INSTITUTIONAL)
- `status`: enum (PENDING, ACTIVE, SUSPENDED, DEPRECATED)
- `law_compat`: list of LAW schema versions supported
- `invariant_map_hash`: hash of module → invariants mapping
- `created_at`, `updated_at`

### 3.2 AssetRecord
- `asset_id`: deterministic ID (hash of LAW profile + module stack + creator)
- `creator_id`
- `law_profile_hash`
- `module_stack`: ordered list of module_ids
- `status`: enum (ACTIVE, PAUSED, SUNSET)
- `created_at`, `updated_at`

---

## 4) Certification Levels (v1)

- **Level 1: COMMUNITY**
  - Basic manifest present
  - Passes deterministic schema checks
  - No audit requirement

- **Level 2: AUDITED**
  - Audit report hash attached
  - Known auditor identity
  - Passes additional policy checks

- **Level 3: RAGTUFF_CERTIFIED**
  - Passes RagTuff enforcement matrix mapping
  - Verified invariant coverage
  - Regression test suite signature

- **Level 4: INSTITUTIONAL**
  - Formal verification or enhanced audit
  - Operational runbook + incident response
  - Versioning + deprecation policy required

**Note:** Certification does not mean “safe in all cases.”
It means: **evidence-backed conformance to declared invariants.**

---

## 5) Deployment Gate: Allow / Reject Rules (Minimum v0)

A deployment MUST be rejected if any of the following are true:

### DG-1: LAW profile invalid
- schema mismatch
- missing required fields
- signature/checksum invalid (if required)

### DG-2: Mandatory module missing
- OutpaceInflation Module (OIM) MUST be present (Near-Intersect doctrine)

### DG-3: Module stack violates doctrine
- total module count must match doctrine
- stacking rules enforced (no module appears >2 times)
- order rules enforced (deterministic ordering if required)

### DG-4: Any module not ACTIVE in registry
- module status must be ACTIVE
- module law_compat must include LAW_v1

### DG-5: Invariant coverage incomplete
- module invariant_map must cover required invariants for LAW_v1
- enforcement matrix validation must succeed

---

## 6) Events (Required)

- `ModuleSubmitted(module_id, publisher_id, manifest_hash, code_hash)`
- `ModuleCertified(module_id, cert_level, evidence_hash)`
- `ModuleStatusChanged(module_id, status)`
- `AssetDeployed(asset_id, creator_id, law_profile_hash, module_stack_hash)`
- `DeploymentRejected(creator_id, reason_code, details_hash)`

Events MUST enable full reconstruction of control-plane decisions.

---

## 7) Minimal Implementation Roadmap (Control Plane v0)

1. **Registry storage + events**
2. **Module submission**
3. **Module activation (admin/multisig)**
4. **Deployment gate checks**
5. **Asset record creation**
6. **Audit-friendly rejection codes**

---

## 8) Rejection Codes (Canonical)

- `RCD_001_LAW_INVALID`
- `RCD_002_OIM_MISSING`
- `RCD_003_STACK_RULE_VIOLATION`
- `RCD_004_MODULE_NOT_ACTIVE`
- `RCD_005_LAW_INCOMPATIBLE`
- `RCD_006_INVARIANT_COVERAGE_FAIL`

---

## 9) Design Intent

Near-Intersect survives any future monetary rail (crypto, banks, CBDC) because it sells:
- deterministic issuance,
- invariant enforcement,
- module governance,
not “money.”

Control plane is the source of legitimacy.
