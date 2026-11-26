---
name: improve-codebase-architecture
description: Review architecture only when the user explicitly requests it; identify deepening opportunities and discuss changes before implementation.
license: MIT
metadata:
  source: https://github.com/mattpocock/skills/blob/5863806dd4e15c63174bee4e7515a4256792ecfa/skills/engineering/improve-codebase-architecture/SKILL.md
  upstream_commit: 5863806dd4e15c63174bee4e7515a4256792ecfa
  adaptation: Inlined required vocabulary and discussion workflow; replaced external skill calls and assumed document paths with existing project context.
---

# Improve Codebase Architecture

Adapted from the pinned upstream skill into a self-contained review workflow.
Read AGENTS.md first. This skill proposes architecture work; it does not authorize
unrelated edits, dependency changes or issue creation.

## Vocabulary and project context

A **module** hides an implementation behind an **interface**. A **deep** module
hides substantial complexity behind a small interface; a **shallow** one exposes
almost as much complexity as it contains. A **seam** permits substitution, and an
**adapter** isolates an external dependency. **Locality** means related behavior
and its changes stay together; **leverage** means one interface supports useful
work at multiple call sites.

Apply the deletion test: would removing a module concentrate complexity and make
understanding harder, or merely remove an extra hop? Preserve real external
boundaries and extension contracts even with one real implementation plus a fake.

Read the existing README.md, relevant docs/ pages and any applicable design
records. Use their domain terms and constraints. Do not assume a glossary, ADR
folder or RFC folder exists, or create one as a side effect of review.

## Explore

- Start with the user's named area. Otherwise inspect recent history to identify
  frequently changed code, then widen only if there is no useful focus.
- Trace complete features through their public entry points and tests. Look for
  repeated navigation between shallow modules, leaking external types, duplicated
  lifecycle rules and seams that make public behavior hard to test.
- When delegation is available, use a fresh-context subagent for independent
  read-only exploration of the selected area. Otherwise perform the same review
  directly. Keep all exploration within the user's requested scope.
- Gather concrete examples and apply the deletion test before proposing a change.
  Distinguish necessary dependency isolation from redundant in-process wrappers.

## Present candidates

Create a visual HTML report in the OS temporary directory, named
`architecture-review-<timestamp>.html`, rather than adding review output to the
repository. Use [HTML-REPORT.md](HTML-REPORT.md) for report structure and diagrams;
its vocabulary is defined above, without requiring another skill.

For each candidate show the affected files, observed friction, proposed change,
benefits for locality, leverage and public-interface testing, before/after visuals,
and recommendation strength: Strong, Worth exploring or Speculative. End with a
top recommendation. Mark any conflict with an existing decision and explain why
revisiting it might be justified. Do not invent public signatures at this stage.

Give the user the report path and ask which candidate to explore. Writing a
report does not authorize implementation or publication.

## Refine the selected candidate

Discuss the goal and success criteria, constraints, dependencies, public interface,
resource ownership, failure behavior and which tests survive. Resolve material
tradeoffs with the user. Compare alternative interfaces when that choice changes
complexity or compatibility; parallel exploration is optional, not a dependency.

Keep accepted domain names aligned with existing project documents. Propose a
record for a durable decision using the project's current documentation layout;
do not automatically create a glossary, ADR, RFC or GitHub issue. Follow the
AGENTS.md design and approval workflow before implementing an accepted change.
Preserve behavior during structural refactoring and verify it with public tests.
