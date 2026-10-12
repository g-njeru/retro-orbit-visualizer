# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
