# 09 — Build and Deploy

The project is a standard Rust crate that builds to **both** a native desktop
program and a WebAssembly (WASM) web page. This page covers how to build it, how
it is published, and how the automation works.

## Requirements

- Rust (stable). The declared minimum supported version is **1.85**
  (`rust-version` in `Cargo.toml`).
- For the browser build: the WASM target, added with
  `rustup target add wasm32-unknown-unknown`.
- A working OpenGL driver for the desktop build.

## Run on the desktop

```sh
cargo run --release                  # production build
cargo run --features dev-overlay     # adds a "DEV BUILD" watermark + FPS counter
```

The `dev-overlay` feature is a compile-time switch (`[features]` in `Cargo.toml`)
that only affects the on-screen HUD; the simulation is identical.

## Build for the browser (WASM)

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp index.html web/mq_js_bundle.js target/wasm32-unknown-unknown/release/
# then serve that folder over HTTP, for example:
python3 -m http.server -d target/wasm32-unknown-unknown/release
```

Open the printed address in any browser that supports **WebGL2** (desktop or
mobile). The `mq_js_bundle.js` is Macroquad's JavaScript glue that loads the
`.wasm` file.

## Where it is published

GitHub Pages hosts two versions from the same repository:

| Branch | URL | Build |
|---|---|---|
| `main` | https://g-njeru.github.io/retro-orbit-visualizer/ | production |
| `dev` | https://g-njeru.github.io/retro-orbit-visualizer/dev/ | with `dev-overlay` |

Pushing to a branch triggers a build and republishes automatically.

## Branches and releases

- **`dev`** — the working branch. Every push builds a live preview at `/dev/`.
- **`main`** — production. Merging here publishes the root site and cuts a
  release.

When `main` is updated, [`release-plz`](https://release-plz.dev) reads the commit
messages, decides the next version, updates `CHANGELOG.md`, creates a `vX.Y.Z`
git tag, and publishes a GitHub Release. Its configuration is tiny
(`release-plz.toml`):

```toml
[workspace]
publish = false   # do not publish to crates.io
git_only = true   # use git tags (not crates.io) as the "latest version"
```

`git_only = true` is needed because the crate is not published to crates.io, so
the version history must come from the git tags.

## Conventional Commits

Every commit message must follow the
[Conventional Commits](https://www.conventionalcommits.org) format, e.g.
`feat: add pinch zoom`, `fix: correct escape speed`, `docs: explain Verlet`.
The prefix decides the version bump:

| Prefix | Bump (when on `main`) |
|---|---|
| `feat:` | minor (`0.1.0 → 0.2.0`) |
| `fix:` | patch (`0.1.0 → 0.1.1`) |
| `feat!:` / `BREAKING CHANGE` | major |
| `chore:`, `docs:`, `ci:`, `build:` | no bump on their own |

A commit-lint check enforces the format on pull requests.

## Continuous Integration (GitHub Actions)

| Workflow | Trigger | Jobs |
|---|---|---|
| `ci.yml` | PR→`main`, push→`dev` | `check` (fmt, clippy `-D warnings`, tests, release build), `wasm` (WASM build), `msrv` (build with Rust 1.85) |
| `check.yml` | PR→`main`/`dev` | `commitlint` |
| `deploy.yml` | push→`main`/`dev` | build both WASM variants, assemble, publish to Pages |
| `release.yml` | push→`main` | `release-plz` release-PR + release |

The `msrv` job is what keeps the claimed minimum version honest: if someone uses a
Rust feature newer than 1.85, that job fails.

## Branch protection

`main` is protected by a repository **ruleset** (`protect-main`):

- Changes must arrive through a **pull request**.
- Required status checks: `check`, `wasm`, `msrv`, `commitlint`.
- The branch must be **up to date** with `main` before merging.
- Force-pushes and deletion of `main` are blocked.

So the normal flow is: work on `dev` (or a feature branch), open a PR to `main`,
let CI go green, then merge.

## License

Licensed under **MIT OR Apache-2.0** (see `LICENSE-MIT` and `LICENSE-APACHE`).

Next: [10 — Glossary and References](10-glossary-and-references.md).
