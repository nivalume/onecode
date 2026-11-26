> **You can fit the whole system into your context window, but the humans maintaining it cannot. Passing compilation and checks is only the floor: add a crate, module, trait, or generic parameter only when removing it would make the code harder to read.**

General Rust project guidelines. Rules in this file take precedence over all skills; when skills conflict, follow **Conflict Resolution**.

Skill definitions and supporting resources live in `.skills/`, shared by `.agents/skills` and `.claude/skills`.

## Project Info

- Project name: `onecode`
- Project type: `workspace` (desktop app built with `gpui-kit`)
- MSRV / edition: `1.85` / `2024`
- Async runtime: GPUI executor (`gpui-kit`); add `tokio` only when concurrent external I/O requires it
- Domain glossary: none yet (`CONTEXT.md` when added); Architecture Decision Records: none yet (`docs/adr/` when added)
- Check commands:

  ```bash
  make check
  # or individually:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace --all-features
  ```

## Skills

| Scenario | Use |
| --- | --- |
| Any task that writes or modifies code | [karpathy-guidelines](.skills/karpathy-guidelines/SKILL.md), always follow |
| Simplifying implementations and preventing over-engineering when writing, editing, or refactoring | [ponytail](.skills/ponytail/SKILL.md), aiming for the smallest change that fully solves the task (`full` level by default) |
| Writing or modifying async / Tokio code | [rust-async-patterns](.skills/rust-async-patterns/SKILL.md) |
| Building GPUI Kit apps, choosing/using components, handling state/theming/overlays/keybindings, following Coding Guides and UI tests | [gpui-kit](.skills/gpui-kit/SKILL.md) |
| Designing or changing any GPUI Kit screen, layout, visual hierarchy, interaction state, overlay, motion, copy, or design review | [gpui-kit-design-guides](.skills/gpui-kit-design-guides/SKILL.md) |
| After completing a feature or significant change, before committing | [code-simplifier](.skills/code-simplifier/SKILL.md) subagent, scoped strictly to files changed in this task |
| When the user requests an architecture review, or when modules feel fragmented / tightly coupled | [improve-codebase-architecture](.skills/improve-codebase-architecture/SKILL.md), run only when requested by the user |

Both `.agents/skills` and `.claude/skills` symlink to `.skills/`. `improve-codebase-architecture` reads context from the domain glossary and ADR files listed under **Project Info**; when those do not yet exist, read existing source code and `README.md` directly and do not create them as a side effect of the review.

### Conflict Resolution

1. `karpathy-guidelines` and `ponytail`'s "no abstraction for single-use code / minimal changes" apply inside a layer; layer-boundary traits and third-party dependency isolation declared in **Project Structure** are exceptions.
2. `karpathy-guidelines`'s "Surgical Changes": do not refactor unrelated code while implementing a feature. List any structural issues you notice at the end of your reply for the user to decide.
3. `ponytail` and `code-simplifier` simplify implementations only, never architecture: do not merge crates, remove layer-boundary traits or dependency-isolation modules, change public APIs, weaken boundary validation, error handling, or type constraints, introduce `unsafe`, suppress clippy with `#[allow]`, or replace clear code with clever one-liners. When `ponytail` takes a shortcut with a known limit, document it with `// shortcut: <limit>, <when to upgrade>`.
4. `rust-async-patterns` applies only to code that genuinely needs concurrent I/O; do not convert synchronous logic to async.
5. `ponytail`'s concise replies are governed by **Vibe Coding Collaboration**: explain the core implementation in one or two sentences, include an ASCII diagram for complex cases, and pause for human confirmation before moving on.

## Vibe Coding Collaboration

- **Proceed in small modules**: break every task into small modules and deliver incrementally instead of writing large swaths of code or crossing multiple modules in one go.
- **Pause to reflect and simplify after each module**: after finishing each small module, **stop and check** for anything that can be simplified or is over-engineered (unnecessary abstractions, unrequested extensibility or config flags, redundant branches, over-splitting; self-review with `ponytail`). **If you find any, simplify immediately** and re-run verification before presenting to the human.
- **Explain the core implementation in one or two sentences**: concisely tell the human what was built and its core mechanism in **one or two sentences** (no verbose play-by-play; mention removed over-engineering or known bounds/risks in one brief sentence if relevant).
- **Include an ASCII diagram for complex cases**: attach a concise **ASCII diagram** (data flow, call chain, state transitions, or module dependencies) whenever any of the following apply:
  - Non-trivial algorithm
  - Complex implementation
  - Multi-step flow (conditional branches, concurrency, retries, etc.)
  - Deep call chain
  - Many dependencies (including one-to-many dependencies)
- **Human review gate**: after reporting, **pause for human review and wait for explicit confirmation before starting the next step** (unless the user explicitly asks to proceed continuously without intermediate checkpoints).

## Workflow

1. **Design confirmation**: present the design first (files and their layers, signatures of new types and traits, data ownership and borrowing, error types, and dependency direction — without implementation) and wait for user confirmation before writing code when any of these apply:
   - Creating a new crate or module
   - Expecting more than 150 lines of new code
   - Touching two or more layers
   - Changing public APIs, or adding traits, generic parameters, or lifetime parameters to public APIs
2. **Implement one small module at a time**: follow `karpathy-guidelines` and `ponytail`, advancing one small module per iteration.
3. **Reflect and simplify**: after each small module, review for over-engineering or simplification opportunities and clean them up immediately.
4. **Run checks and self-review**: run all applicable check commands and verify every item in **Pre-Completion Checklist** (honestly report any unconfigured or unrun checks; never claim they passed). After larger changes, invoke `code-simplifier` and re-run checks.
5. **Report and wait for review**: per **Vibe Coding Collaboration**, summarize the core implementation in one or two sentences (with an ASCII diagram when complex), then pause for explicit human confirmation before continuing.

## Project Structure

While placeholders (`<...>`) remain in the tables below, ask the user where to place any new crate or module before creating it; do not invent layers on your own.

Layout: `workspace`

```text
onecode/
├── Cargo.toml                  # [workspace], centralized dependency versions, lints, and profiles
├── clippy.toml                 # Clippy configuration
├── rustfmt.toml                # Rustfmt configuration
├── Makefile                    # Common fmt / clippy / test / build targets
├── app/
│   └── onecode/                # onecode (binary entrypoint)
└── crates/
    ├── ui/                     # onecode-ui
    ├── ai/                     # onecode-ai
    └── core/                   # onecode-core
```

### Layers

| Layer | Crate | Responsibility | Allowed Dependencies |
| --- | --- | --- | --- |
| `<name>` | `<crates/<name>>` | `<one sentence>` | `<layer names, or "none">` |

### Third-Party Dependency Isolation

| Dependency | Allowed Only In |
| --- | --- |
| `<e.g. gpui-kit>` | `<UI / shell crates once defined>` |

### Additional Constraints

- Depend on `gpui-kit` alone for GPUI, `gpui-base`, `gpui-component`, and default assets; never add a separate `gpui` dependency.
- Member crates must inherit workspace lints (`[lints] workspace = true`) and package metadata (`edition.workspace = true`, `rust-version.workspace = true`, `publish.workspace = true`).
- For UI integration tests, enable `gpui-kit = { workspace = true, features = ["test-support"] }` under `[dev-dependencies]`.

### Rules

- Depend only on layers listed in the **Layers** table.
- In a workspace, Cargo enforces dependency direction: a crate's `[dependencies]` lists only allowed layers, and third-party crates appear only in the crates specified in the isolation table.
- In a single crate, layers communicate only through `pub(crate)` interfaces; never reach into another layer's private internals.
- Define layer-boundary traits only for external I/O: databases, networks, filesystems, clocks. A real implementation plus a test fake counts as two implementations and is not over-abstraction.
- Declare dependency versions centrally in root `Cargo.toml` under `[workspace.dependencies]` and reference them in member crates via `xxx = { workspace = true }`.

## Directories and Files

- Organize submodules with `foo.rs` plus `foo/`; do not use `mod.rs`.
- No `utils.rs`, `helpers.rs`, `common.rs`, or `misc.rs`. Place functions in the module that uses them; when shared across multiple modules, create a purpose-named module.
- Name files after concepts; do not dump every type into `types.rs` or `models.rs`.
- When a directory grows beyond 8 modules, split by domain into submodules. Keep nesting to at most 3 levels.
- `lib.rs` only declares modules and re-exports the public API with `pub use`.
- `main.rs` only handles argument parsing, config loading, dependency wiring, and startup; keep logic in library code so it is testable.

## Module and Function Granularity

Principle: modules should be deep — small public interfaces hiding rich implementations (John Ousterhout, *A Philosophy of Software Design*).

- Default visibility is private; use `pub(crate)` when needed across modules; reserve `pub` for external APIs.
- Extract a function only when at least one of these holds:
  - Reused in two or more places
  - Represents a named concept whose name saves the reader from reading its body
  - Isolates I/O to aid testing
- Do not split functions just to hit a line count. One cohesive 40-line function beats five 8-line fragments.
- Avoid shallow wrappers that only forward arguments verbatim, except at layer boundaries and dependency-isolation modules.
- Keep a feature's logic cohesive; understanding one feature should not require jumping across more than 5 files.

## Types and Traits

Data and behavior:

- Keep data and the behavior maintaining its invariants in the same `struct` and its `impl`. Validate invariants at construction (`new`, `TryFrom`) and do not re-check them afterward.
- Model closed sets of variants with `enum` and `match`, not trait objects.
- Use newtypes only when validation or type safety prevents real mix-ups (e.g. `UserId` vs `OrderId`); do not wrap every field.
- Use `new` or struct literals when constructor parameters are few; introduce a builder only with many optional parameters (~4+).
- Derive traits on demand rather than adding a long default list beyond `Debug`.

Abstraction:

- Define a trait only in two cases: two or more implementations exist (including test fakes), or it is a layer boundary declared in **Project Structure**. Delete single-implementation traits and use the concrete type directly.
- Add generic parameters only when multiple concrete types are actually passed, never "just in case".
- Do not use `Box<dyn Trait>` or `Arc<dyn Trait>` when there is only one implementation.
- Avoid deep trait hierarchies; keep supertraits to at most one level.
- Avoid lifetime parameters on structs except for explicit borrowing view types (e.g. parsers).
- Do not write macros for what functions or generics can express cleanly.

Ownership and dependencies:

- Do not `.clone()` merely to appease the borrow checker. First consider borrowing, moving, or adjusting ownership; cloning is fine when cheap or semantically required.
- Do not use `Rc<RefCell<_>>` or `Arc<Mutex<_>>` to bypass ownership design. Justify any shared mutable state during design.
- Inject dependencies through constructors and store them as fields. Avoid mutable global state (`static mut`, global `static` wrapped in a `Mutex`).
- Avoid vague names like `Manager`, `Helper`, `Handler`, or `Processor`; use domain nouns.

## Error Handling

Trust the type system; validate only at boundaries.

- Boundaries are CLI arguments, config, environment variables, files, network responses, deserialization, and FFI. Convert external data into validated types at the boundary (`TryFrom`, validated `serde`).
- Inside boundaries, do not re-check invariants already guaranteed by types.
- Error types: use `thiserror` for concrete error enums in library crates; use `anyhow` with `.context(...)` in binary crates. Never return `anyhow::Error` or `Box<dyn Error>` from a library's public API.
- Propagate with `?`. Match on errors only to take meaningful action: retry, degrade, translate into a layer error, or clean up resources.
- Forbidden:
  - `unwrap()` in non-test code
  - `expect()` for failures that can actually happen (`expect()` is reserved for truly impossible invariants, with a message explaining why it cannot fail)
  - Swallowing errors that should propagate via `unwrap_or_default()`, `unwrap_or(...)`, or `.ok()`
  - Ignoring a `Result` with `let _ = ...`
  - Logging an error and continuing as if nothing happened
  - Returning `Result` from functions that cannot fail
  - Adding fallback branches for conditions with no evidence they can occur
- Reserve `panic!` for programmer bugs (broken invariants), never recoverable errors.
- Handle timeouts and retries for external calls centrally in the dependency-isolation layer, not scattered across business logic.

## `unsafe`

- Forbidden by default (see **Recommended Tooling Config**). If genuinely required, flag it during design; isolate `unsafe` code in one module and precede every `unsafe` block with a `// SAFETY:` comment explaining why the invariants hold.

## `async` (When Using Tokio)

- Use `async` only at I/O boundaries and where concurrency is needed; keep domain logic and pure computation synchronous.
- Never run blocking operations inside async code; offload to `spawn_blocking` when necessary.
- Create the async runtime only at the binary entrypoint.
- Prefer native `async fn` in traits; use `async-trait` only when `dyn` dispatch is required.

## Code Style

- Document public APIs with `///` doc comments explaining purpose, `# Errors` when fallible, and `# Panics` if panics are possible.
- Write comments to explain *why*, not *what* the code does.
- Never leave `dbg!`, debug `println!`, `todo!()`, or commented-out code behind.
- Do not silence warnings with `#[allow(...)]`. When an exception is genuinely needed, use `#[expect(lint, reason = "...")]` with a clear reason (Rust 1.81+).

## Git Commits

- Write all commit messages in **English** with clear, concise phrasing.
- Format:

  ```text
  <type>(<scope>): <concise summary>

  - <optional key change 1>
  - <optional key change 2>
  ```

- Types: `feat`, `fix`, `hotfix`, `chores` (use `<type>(<scope>): <summary>`).
- Omit the bullet list for small or straightforward changes; include a short `- ...` list only when the change is substantial.
- **No agent co-sign**: never append `Co-Authored-By:` or any AI/agent attribution trailers.

## Pre-Completion Checklist

- [ ] `fmt`, `clippy`, and `test` all pass (any unconfigured checks are reported honestly)
- [ ] After each small module, paused to reflect and simplify/remove over-engineering; explained the core implementation in 1–2 sentences (with an ASCII diagram when complex) and waited for human confirmation before proceeding
- [ ] Removing any newly added crate, module, trait, or generic parameter would make the code noticeably worse
- [ ] New code sits in the right layer and `Cargo.toml` dependencies match the **Layers** table
- [ ] No single-implementation traits (outside layer boundaries), unnecessary generics, or unnecessary `dyn`
- [ ] No new `utils` / `helpers` modules and no `mod.rs`
- [ ] No `unwrap()` in non-test code, no swallowed errors, and no `let _ =` discarding a `Result`
- [ ] No `.clone()` added solely to satisfy the borrow checker
- [ ] No new `unsafe`; if present, every block has a `// SAFETY:` comment
- [ ] No drive-by refactoring of unrelated code; any structural issues discovered are listed in the reply

## Recommended Tooling Config

Root `Cargo.toml`:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
unwrap_used = "warn"
dbg_macro = "warn"
todo = "warn"
mod_module_files = "warn"
```

Each member crate's `Cargo.toml`:

```toml
[lints]
workspace = true
```

For single-crate projects, replace `[workspace.lints.*]` with `[lints.*]`.

`clippy.toml`:

```toml
allow-unwrap-in-tests = true
```
