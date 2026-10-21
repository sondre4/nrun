<!-- # nrun
i cant remember what the npm scripts are called -->

<h1 align="center">nrun</h1>

<p align="center">
  <em>I can't remember what the npm scripts are called</em>
</p>

<!-- ![Demo of TUI](docs/assets/tui-demo.png) -->

<!-- Badges -->

<p align="center">
  <img src="https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-334155" alt="platforms">
  <a href="#compatibility">
    <img src="https://img.shields.io/badge/supports-npm%20%7C%20bun%20%7C%20yarn%20%7C%20pnpm-334155"
        alt="Supports npm, Bun, Yarn, and pnpm,">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/sondre4/nrun" alt="License">
  </a>
  <a href="https://github.com/sondre4/nrun/actions/workflows/ci.yml">
    <img src="https://github.com/sondre4/nrun/actions/workflows/ci.yml/badge.svg" alt="CI status">
  </a>
  <a href="https://github.com/sondre4/nrun/releases/latest">
    <img src="https://img.shields.io/github/v/release/sondre4/nrun" alt="Latest release">
  </a>
</p>

<!-- Demo image -->

<p align="center">
  <img
    src="docs/assets/tui-demo.png"
    alt="TUI Demo"
    width="1200"
  />
</p>

## Features
- TUI
  - Browse project-defined scripts from all packages in e.g. a Node monorepo
  - Run selected script by pressing `Enter`
  - Expand/collapse selected package by pressing `Enter`
  - Search for package/script names

- CLI
  - Display all project-defined scripts in e.g. a Node monorepo
  - Search scripts from the command line
  - Output formatted packages/scripts overview
  - Optional JSON output format
  - Run `nrun --help` for more information

## Compatibility
- Works with **npm**, **Bun**, **Yarn**, and **pnpm**
- Works for Node **projects** and **monorepos**
- Runs natively on **Linux**, **macOS** and **Windows**.

### Package-manager specifics tested
- Project and monorepo discovery for **npm**, **Bun**, **Yarn**, and **pnpm**
- Running project-defined scripts
- Tested on Linux, macOS and Windows in CI

<!-- ### Format and lint command
`cargo fmt`
`cargo clippy --workspace --all-targets --all-features` -->
