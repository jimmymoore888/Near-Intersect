# OIM Upstream Authority (Pinned)

Near-Intersect tokens MUST permanently include the OutpaceInflation Module (OIM).

**Upstream canonical authority:** RagTuff `OIM/OIM_v1_SPEC.md`  
**Pinned conformance tag:** oim-v1.0.0 *(GitHub Release tag in RagTuff)*

Near-Intersect MUST:
- refuse token creation if OIM is absent,
- refuse runtime upgrade paths that remove OIM,
- treat OIM invariants as non-bypassable constraints,
- expose OIM state/config views for auditability.

Note: “OIM” defines constraints and allowed mechanisms only. It does not guarantee returns.

https://github.com/jimmymoore888/ragtuff/blob/oim-v1.0.0/OIM/OIM_v1_SPEC.md
