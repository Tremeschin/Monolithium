<div align="center">
  <img src="./monolithium/resources/images/logo.png" width="210">
  <h1 style="margin-top: 0">Monolithium</h1>
  <span>🗿 Finding the Largest Minecraft Infdev/Alpha Monoliths 🗿</span>
  <br>
  <br>
    <a href="https://crates.io/crates/monolithium/"><img src="https://img.shields.io/crates/v/monolithium?label=Crates.io&color=orange"></a>
    <a href="https://crates.io/crates/monolithium/"><img src="https://img.shields.io/crates/d/monolithium?label=Downloads&color=orange"></a>
    <a href="https://pypi.org/project/monolithium/"><img src="https://img.shields.io/pypi/v/monolithium?label=PyPI&color=blue"></a>
    <a href="https://pypi.org/project/monolithium/"><img src="https://img.shields.io/pypi/dw/monolithium?label=Installs&color=blue"></a>
    <a href="https://github.com/Tremeschin/Monolithium/"><img src="https://img.shields.io/github/v/tag/Tremeschin/Monolithium?label=GitHub&color=orange"></a>
    <a href="https://github.com/Tremeschin/Monolithium/stargazers/"><img src="https://img.shields.io/github/stars/Tremeschin/Monolithium?label=Stars&style=flat&color=orange"></a>
    <a href="https://discord.gg/KjqvcYwRHm"><img src="https://img.shields.io/discord/1184696441298485370?label=Discord&style=flat&color=purple"></a>
  <br>
  <br>
</div>

<img width="2473" height="1262" alt="Monolith" src="https://github.com/user-attachments/assets/cfa62e8f-6367-4768-9e62-c8879aba16b8"/>

<b>Seed:</b> 26829160 • (x: 0, y: 0) • Area: 1,044,848 blocks squared • _Most Aesthetic_
<br><sup><b>Using</b> [Moderner Beta](https://modrinth.com/mod/moderner-beta) Alpha v1.1.2_01 Terrain on MC 1.21 • [Distant Horizons](https://modrinth.com/mod/distanthorizons) • [Bliss Shaders](https://github.com/X0nk/Bliss-Shader/)</sup>

## 🔥 Description

Monoliths are..

## 📦 Installation

Install the [Rust](https://www.rust-lang.org/tools/install) programming language and [git](https://git-scm.com/downloads) (winget recommended), open a terminal in some directory and run:

- `git clone https://github.com/Tremeschin/Monolithium`
- `cd Monolithium && rustup default stable`
- `cargo run --release -- (arguments)`

There are multiple subcommands available, you can run `cargo run --release -- (subcommand) --help` to see options:

<sup><b>Warning:</b> The code <i>will</i> shred your cpu, make sure you have a good cooling solution, it may be unusable while running</sup>

### Find all Monoliths in a world

This will search a 8,388,608 blocks square in both positive X and Z directions. Note that all monoliths repeats every such value - there are 9 copies of each within the Far Lands!

- `cargo run --release -- find --seed 617`

### Find seeds with spawn monoliths

This will search for seeds that contains monoliths close to spawn.

- `cargo run --release -- spawn random -n 1000000`

## ⭐️ Showcase

## ♻️ Credits

- **User [@kahomayo](https://github.com/kahomayo)** for [`monolith-renderer`](https://github.com/kahomayo/monolith-renderer), which this rewritten code is heavily inspired by.
