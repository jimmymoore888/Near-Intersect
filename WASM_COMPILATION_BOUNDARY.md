# WASM COMPILATION BOUNDARY — LAW v1

Status: Deterministic · Sealed  
Scope: UI → Compiler → NEAR Runtime

---

## 1. AUTHORIZED INPUT

The compiler SHALL accept input ONLY in the form of:

- Fully populated LAW_v1_SCHEMA
- Intent acknowledgment = true
- All invariants satisfied

Any other input MUST be rejected.

---

## 2. PROHIBITED INPUT

The following inputs are explicitly forbidden:

- Raw Rust source
- Free-form code
- CLI flags
- Environment variables
- Dynamic configuration
- Post-deploy mutation instructions

---

## 3. COMPILATION PROCESS

The compilation pipeline is fixed:

LAW_v1_SCHEMA
    ↓
Deterministic Schema Parser
    ↓
Invariant Validator
    ↓
Canonical Rust Template
    ↓
WASM Compiler (fixed version)
    ↓
contract.wasm

No step may be skipped or reordered.

---

## 4. CANONICAL RUST TEMPLATE

- Rust source is generated, never authored
- Template is version-locked
- No conditional compilation flags
- No feature toggles
- No unsafe blocks

---

## 5. DETERMINISM GUARANTEES

- Same schema → same Rust
- Same Rust → same WASM
- Same WASM → same contract behavior

Non-determinism is a compile-time failure.

---

## 6. OUTPUT ARTIFACTS

Successful compilation MUST emit:

- contract.wasm
- law_text_hash
- invariant_hash
- schema_hash

All artifacts are bound together.

---

## 7. VERIFICATION REQUIREMENTS

Before deployment:

- WASM hash MUST match local hash
- law_text_hash MUST match UI-rendered law
- invariant_hash MUST match invariant file

Mismatch = hard failure.

---

## 8. DEPLOYMENT RULE

Deployment is permitted IFF:

- Compilation succeeded
- All hashes match
- Final certification completed

Deployment is single-use.

---

## 9. POST-DEPLOY STATE

After deployment:

- Compiler is no longer relevant
- Schema becomes read-only
- Contract state is authoritative

No upgrade paths exist.

---

## 10. SECURITY AXIOM

If behavior is not provable from:
- LAW_v1_SCHEMA
- LAW_v1_INVARIANTS
- This boundary document

Then it MUST NOT exist.
