---
name: code-simplifier
description: Review and simplify recently changed feature code while preserving behavior and project contracts; use after implementation and before committing, not for documentation-only changes.
license: Apache-2.0
metadata:
  source: https://github.com/anthropics/claude-plugins-official/blob/df9699a55c56707e9948a32951ea356638ea8519/plugins/code-simplifier/agents/code-simplifier.md
  upstream_commit: df9699a55c56707e9948a32951ea356638ea8519
  adaptation: Adapted agent definition into a Rust skill; replaced JS rules, fixed model and automatic broad edits with project-scoped review.
---

# Code Simplifier

Adapted from Anthropic's official code-simplifier agent for project-local Rust
work. No fixed model or Claude-specific tool is required. Read AGENTS.md and the
changed code first; project rules override generic simplification preferences.

## Scope and review

Use a subagent when available, supplying the changed paths or diff and relevant
project contracts. Ask it to review only this task's changed code. If delegation
is unavailable, perform the same review directly and disclose that limitation.
Documentation-only edits and skill installation do not require this code review.

Preserve observable functionality, error semantics, public interfaces, data
formats and lifecycle behavior. Prefer explicit readable code over compact or
clever expressions. Look for unnecessary nesting, duplicated validation or cleanup,
misleading names, dead code introduced by this change and abstractions that add
navigation without hiding complexity.

Apply only findings within the authorized task. Report unrelated structural
issues without changing them. Do not measure simplicity by line count or extract
helpers merely to shorten functions.

## Preserve project contracts

- Keep external dependency isolation, existing domain contracts and extension
  points, including interfaces with one implementation plus a test fake.
- Do not turn domain objects into static-method containers, inline their business
  flows into callers, or replace existing plugin/toolset objects with closures.
- Do not introduce new registries, configuration sources or generic wrapper layers.
- Do not add `#[allow(...)]` lint suppressions, `unsafe` blocks, `unwrap()` in
  non-test code, swallowed errors, or broad fallback branches to make checks pass.
- Keep dependencies injected and resource ownership explicit. Do not move I/O into
  construction or package imports.

## Validate and report

Make supported simplifications, then run the repository's applicable checks from
AGENTS.md.
If review produces no change, avoid repeating checks already completed for the
same code. Explain only material changes and disclose failed or unrun checks.
