# Drips Wave — issue labels (SS-contracts)

## Wave 1 focus (implementation)

**Product:** Decentralized invoice factoring on Stellar — SMBs **tokenize invoices** as on-chain assets, **investors** buy at a discount for **yield**, smart contracts handle **escrow** and **settlement** (replacing centralized factors).

**This wave:** **bugs, enhancements, tests, CI/security tooling** — not standalone documentation issues. Doc-only GitHub issues were **closed as deferred** (see closing comments); they can be reopened or recreated in a later wave.

---

This repo uses GitHub labels that map to the Drips complexity / points guidance:

| Label | Meaning |
|--------|--------|
| `points-100-trivial` | Trivial (~100 points) — small, bounded work |
| `points-150-medium` | Medium (~150 points) — standard scope, multiple files/tests |
| `points-200-high` | High (~200 points) — integrations, security-sensitive, or architectural |

**Reference:** [Creating meaningful issues (Drips)](https://www.drips.network/blog/posts/creating-meaningful-issues)

## Maintainer workflow

1. Open or triage the issue; ensure it has exactly **one** `points-*` label (adjust if scope changes).
2. Keep issues **Wave-sized**; split if too large.
3. PRs should include `Closes #<issue>` and meet the **PR must include** checklist on the issue.

## Issue description format

Each issue should start with:

1. **Complexity / points** line + maintainer `points-*` label name  
2. **PR must include** checklist (exact 5 items; `Closes #<n>` matches the issue)

Bulk editing via local scripts is **not part of this repo** (`/scripts/` is gitignored). Use the GitHub UI or your own private maintainer tooling if you need to reformat many issues.
