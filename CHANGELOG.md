# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/g-njeru/retro-orbit-visualizer/compare/v0.1.1...v0.1.2) - 2026-10-13

### Other

- explainer documentation set ([#7](https://github.com/g-njeru/retro-orbit-visualizer/pull/7))

## [0.1.1](https://github.com/g-njeru/retro-orbit-visualizer/compare/v0.1.0...v0.1.1) - 2026-10-12

### Fixed

- pointer hit-testing, globe meridians; add tests, MSRV 1.85, CI and pages reliability ([#2](https://github.com/g-njeru/retro-orbit-visualizer/pull/2))

### Other

- use git tags as release-plz baseline (git_only) ([#3](https://github.com/g-njeru/retro-orbit-visualizer/pull/3))

## [0.1.0](https://github.com/g-njeru/retro-orbit-visualizer/releases/tag/v0.1.0) - 2026-10-09

### Added

- make original 1963 model the first scene, add touch model navigation and dev-overlay flag
- add 6-model orbital mechanics suite with tilt and zoom camera
- 1963 retro orbit visualizer for desktop and wasm (macroquad)

### Fixed

- deploy distinct prod and dev wasm artifacts
- quote release job conditions to satisfy yaml parser

### Other

- disable crates.io publishing in release-plz config
- keep dev preview live on every deploy and fix release jobs token scoping
- document models, touch controls, branch flow and releases
- enforce conventional commits and automate releases with release-plz
- deploy main and dev branches to Pages with dev preview subpath
