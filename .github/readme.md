<div align="center">
  <img src="https://raw.githubusercontent.com/Tremeschin/Monolithium/main/monolithium/resources/images/logo.png" width="210">
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

<img alt="Monolith screenshot" src="https://github.com/user-attachments/assets/cfa62e8f-6367-4768-9e62-c8879aba16b8"/>

<b>Seed:</b> 26829160 • (x: 0, y: 0) • Area: 1,044,848 blocks squared • _Most Aesthetic_
<br><sup><b>Using</b> [Moderner Beta](https://modrinth.com/mod/moderner-beta) Alpha v1.1.2_01 Terrain on MC 1.21 • [Distant Horizons](https://modrinth.com/mod/distanthorizons) • [Bliss Shaders](https://github.com/X0nk/Bliss-Shader/)</sup>

## 🔥 Description

Monoliths are a terrain generation bug that happened in the ancient Minecraft Infdev through Alpha 1.1.2_01 versions. They are a rare, often large, flat and tall structures, almost entirely made of stone, with a hidden ocean of water until bedrock beneath them.

Hidden in plain sight for 15 years, I wanted to know how rare they actually are, and find the biggest one!

- This repository contains a brute-force search algorithm in Rust with a basic Python package for statistical analysis, alongside the most interesting findings of the journey.
- Feel free to contribute speed, documentation, cuda port improvements.

_**Warn**: This is a side project, I may have time to port the readme to a mkdocs website in the future._

<sup><b>Note:</b> For a more technical explanation, see [`kahomayo/monolith-renderer`](https://kahomayo.github.io/monolith-renderer/) 😉</sup>

## 📦 Installation

Install the [Rust](https://www.rust-lang.org/tools/install) programming language and [git](https://git-scm.com/downloads), open a terminal in some directory and run:

- `git clone https://github.com/Tremeschin/Monolithium`
- `cd Monolithium && rustup default stable`
- `cargo run --release -- (arguments)`

There are multiple commands available; run `cargo run --release -- (subcommand) --help` for options:

<sup><b>Warning:</b> The code <i>will</i> shred your cpu, make sure you have a good cooling solution, it may be unusable while running</sup>

### 🔴 Find all Monoliths in a world

This will search a 8,388,608 blocks square in both positive X and Z directions. Note that all monoliths repeats every such value on any coordinate - there are 9 copies of each within the Far Lands on any given world!

- `cargo run --release -- find --seed 617`

### 🟡 Find seeds with spawn monoliths

This will search for seeds that contains monoliths close to spawn.

- Search 0 through 100k seeds: `cargo run --release -- spawn linear -c 100000`
- Search 50k random seeds: `cargo run --release -- spawn random -n 50000`

## 🚀 Speeds

Monolithium is written in heavily parallelized [Rust](https://www.rust-lang.org/) with the help of crates like [Rayon](https://crates.io/crates/rayon) for fearless concurrency and [Ahash](https://crates.io/crates/ahash) fast hashing, fully utilizing all available CPU resources one throws at it.

🦀 For a Ryzen 9 5900X 12c/24t, 2x3200 MT/s DDR4 CL16 2Rx8 system, one might expect:

- **3.75 minutes** to find all monoliths in a seed, probing every 128 blocks.
- **Search 1,150,000** seeds per second for spawn monoliths (approximated)
- **Search 440,000** seeds per second for spawn monoliths (accurate)

Such speeds scales about linearly with your hardware - for better or worse!

## ⭐️ Showcase

Hall of fame for the timeline of computations:

<div align="center">
  <table>
    <thead>
      <tr>
        <th><b>Date</b></th>
        <th><b>CPU</b></th>
        <th><b>Time</b></th>
        <th><b>Seeds</b></th>
        <th><b>% Total</b></th>
        <th><b>Type</b></th>
        <th><b>User</b></th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>August 2025</td>
        <td>R9 5900x</td>
        <td>8 hours</td>
        <td align="right">5,000,000,000</td>
        <td align="right">0.00177%</td>
        <td align="right">Accurate</td>
        <td><b><a href="https://github.com/Tremeschin/">Tremeschin</a></b></td>
      </tr>
    </tbody>
  </table>
</div>

<sup><b>Note</b>: For sure throwing more CPU and/or Time at the code will break new records!</sup>

### 🔵 Largest monoliths near spawn

<div align="center">
  <table>
    <thead>
      <tr>
        <th><b>Area</b></th>
        <th><b>Seed</b></th>
        <th><b>Date</b></th>
        <th><b>Comment</b></th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>1,745,664</td><td>4609608251</td><td>August 2025</td>
        <td align="left">🥇 Largest, high monolith density in world</td>
      </tr>
      <tr>
        <td>1,584,112</td><td>1847066092</td><td>August 2025</td>
        <td align="left">🥈 Second largest, hourglass shaped</td>
      </tr>
      <tr>
        <td>1,420,816</td><td>2045872561</td><td>August 2025</td>
        <td align="left">🥉 Third largest, shattered complex</td>
      </tr>
      <tr>
        <td>1,371,824</td><td>3847304212</td><td>August 2025</td>
        <td align="left">Balanced monolith complex</td>
      </tr>
      <tr>
        <td>1,369,360</td><td>1593912439</td><td>August 2025</td>
        <td align="left">Contiguous (technically <i>The Largest</i>)</td>
      </tr>
      <tr>
        <td>1,345,520</td><td>4563197188</td><td>August 2025</td>
        <td align="left">-</td>
      </tr>
      <tr>
        <td>1,305,472</td><td>4432659853</td><td>August 2025</td>
        <td align="left">-</td>
      </tr>
      <tr>
        <td>1,295,568</td><td>4801496200</td><td>August 2025</td>
        <td align="left">Closest to a circle?</td>
      </tr>
      <tr>
        <td>1,268,432</td><td>2273407323</td><td>August 2025</td>
        <td align="left">-</td>
      </tr>
      <tr>
        <td>1,264,720</td><td>3054588959</td><td>August 2025</td>
        <td align="left">Similar shape to the Iberian Peninsula</td>
      </tr>
      <tr>
        <td>1,262,784</td><td>4613464116</td><td>August 2025</td>
        <td align="left">-</td>
      </tr>
      <tr>
        <td>1,242,256</td><td>4983179351</td><td>August 2025</td>
        <td align="left">Florida</td>
      </tr>
    </tbody>
  </table>
</div>

<sup><b>Note:</b> The area calculation is within 2% error, nearby monoliths are part of the same complex.</sup>

### 🔵 Lowest seed visible from spawn

*Drum rolls..* 617. This seed contains a visible Monolith from spawn 🤯

### 🔵 Monoliths repeat every 8,388,608 blocks

For every monolith in a world there's 9x exact copies of them within the Far Lands:

- A monolith at spawn appears on:
- `(-x,  x) • ( 0,  x) • ( x,  x)`
- `(-x,  0) • ( 0,  0) • ( x,  0)`
- `(-x, -x) • ( 0, -x) • ( x, -x)`

Sadly, the Far Lands override the monoliths, there's no such thing as a Far Monolith 😿

> [!NOTE]
> _For the keen among you, that value is `2**23` - this happens for a couple of reasons:_
> - Ken Perlin's noise, unscaled, repeats every 256 units on any coordinate • `(2**8)`
> - There are 16 octaves on the hill noise, each octave halves the previous frequency, so the highest one repeats every `(2**15)` blocks (starting from multiplier 1).
> - Minecraft samples every 4 blocks, the depth scale is 100 but `& 0xFF` truncations cancels it at `2**20`
> Multiplying factors, `2**(8+15) = 2**23`

### 🔵 How rare are they?

_Spoiler: Not much_, certain seeds are more likely to generate monoliths (anthropic principle confirmed?), but most contains at least half a million monolith _complexes_ within the Far Lands (12,550,824 blocks squared).

## 🔎 Future work

- Investigate the correlation of Perlin coefficients to the likeliness and size of Monoliths.
- Is it more efficient for CUDA to stream perlin coefficients than inline JavaRNG on CPU?
- Make statistical analysis (Average size, Distribution) of Monoliths in seeds.
- Make a `HashMap<(int, int), Monolith>` to avoid recomputing areas
- Throw 2x Epyc 9965 at the code. I have a spare one for sure iirc.

## ♻️ Credits

- **User [@kahomayo](https://github.com/kahomayo)** for [`monolith-renderer`](https://github.com/kahomayo/monolith-renderer) to understand the underlying mathematics.
- **YouTuber [@AntVenom](https://www.youtube.com/@AntVenom/)** For the [Breaking Minecraft](https://www.youtube.com/playlist?list=PLR50dP3MW9ZWMSVz2LkRoob_KRf72xcEx) playlist inspiration.
