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

<b>Seed:</b> 26829160 • (x: 0, y: 0) • Area: 1,044,848 blocks squared • _First Millionaire_
<br><sup><b>Using</b> [Moderner Beta](https://modrinth.com/mod/moderner-beta) Alpha v1.1.2_01 Terrain on MC 1.21 • [Distant Horizons](https://modrinth.com/mod/distanthorizons) • [Bliss Shaders](https://github.com/X0nk/Bliss-Shader/)</sup>

## 🔥 Description

Monoliths are a terrain generation bug that happened in the ancient Minecraft Infdev through Alpha 1.1.2_01 versions. They are a rare, often large, flat and tall structures, almost entirely made of stone, with a hidden ocean of water until bedrock beneath them.

Hidden in plain sight for 15 years, I wanted to know how rare they actually are, and find the biggest one!

- This repository contains a brute-force search algorithm in Rust with a basic Python package for statistical analysis, alongside the most interesting findings of the journey.
- Feel free to contribute speed, documentation, cuda port improvements.

_**Warn**: This is a side project, I may have time to port the readme to a mkdocs website in the future._

<sup><b>Note:</b> For a more technical explanation, see [`kahomayo/monolith-renderer`](https://kahomayo.github.io/monolith-renderer/) 😉</sup>

## 📦 Installation

> [!WARNING]
> - The code _will_ shred your cpu, make sure you have a good cooling solution!
> - Large directories are created where you run it - any of `target`, `build`, `release`.

### • Stable releases

Install [astral-sh/uv](https://docs.astral.sh/uv/), open a terminal in some directory and run:
- **Rust code**: `uvx --from monolithium rustlith (commands)`
- **Cuda code**: `uvx --from monolithium cudalith (commands)`

### • Latest Git

Install [git](https://git-scm.com/downloads) and [astral-sh/uv](https://docs.astral.sh/uv/), open a terminal in some directory and run:
- `git clone https://github.com/Tremeschin/Monolithium`
- `cd Monolithium` • `uv run rustlith (commands)`

You can pass any `--<feature>` explained in [`Cargo.toml`](../monolithium/Cargo.toml) for speedups, like `--fast`!

### 🔴 Find all Monoliths in a world

This will search a 8,388,608 blocks square in both positive X and Z directions. Note that all monoliths repeats every such value on any coordinate - there are 9 copies of each within the Far Lands on any given world!

- `rustlith find --seed 617`

### 🟡 Find seeds with spawn monoliths

This will search for seeds that contains monoliths close to spawn.

- Search 0 through 100k seeds: `rustlith spawn linear -c 100000`
- Search 50k random seeds: `rustlith spawn random -n 50000`

## 🚀 Speeds

Monolithium is written in heavily parallelized [Rust](https://www.rust-lang.org/) with the help of crates like [Rayon](https://crates.io/crates/rayon) for fearless concurrency and [Ahash](https://crates.io/crates/ahash) fast hashing, fully utilizing all available CPU resources one throws at it.

🦀 For a Ryzen 9 5900X 12c/24t, 2x3200 MT/s DDR4 CL16 2Rx8 system, one might expect:

- **3.75 minutes** to find all monoliths in a seed, probing every 128 blocks.
- **Search 1,150,000** seeds per second for spawn monoliths (approximated)
- **Search 440,000** seeds per second for spawn monoliths (accurate)

Such speeds scales about linearly with your hardware - for better or worse!

## ⭐️ Showcase

> [!NOTE]
> Area calculations are within 1% error, nearby monoliths are part of the same complex.

Hall of fame for the timeline of computations:

<div align="center">

<!--
Methodology:
- (2025/09/20): (rustlith spawn --chunks 1000 --radius 262144 --step 1024 random --total 1000000000000) • (skip-rejection, skip-table, only-hill, filter-fracts, scaled-deviation) • (Hill quality: 280.0) • (find --step 512 --seed <best>)
- (2025/09/18): (rustlith spawn --chunks 1000 --radius 3000 --step 500 random --total 75000000000) • (skip-rejection, skip-table, only-hill, filter-fracts, scaled-deviation) • (Hill quality: 380.0) • (find --step 512 --seed <best>)
- (2025/08/13): (rustlith spawn --radius 200 --step 100 linear --total 5000000000)
-->

| Date       | Hardware | Time | Seeds               |  Total (%) | Type        | User                        |
| :--------: | :------: | :--: | ------------------: | ---------: | :---------: | :-------------------------: |
| 2025/09/20 | i7 12700 | 14h  |   1,000,000,000,000 |   0.35527% | Lossy/2Pass | [**akatz-ai**](https://github.com/akatz-ai/) |
| 2025/09/18 | R9 5900x | 45m  |      75,000,000,000 |   0.02664% | Lossy/2Pass | [**Tremeschin**](https://github.com/Tremeschin/) |
| 2025/08/13 | R9 5900x | 8h   |       5,000,000,000 |   0.00177% | Accurate    | [**Tremeschin**](https://github.com/Tremeschin/) |

</div>

### 🔵 Largest monoliths anywhere

<div align="center">

| Area      | Seed               | X       | Z       | Date       | Found by   |
| :-------: | :----------------: | ------: | ------: | :--------: | :--------: |
| 3,119,151 | 94116384388573     | 3185395 | 1401244 | 2025/10/06 | Tremeschin |
| 2,890,848 | 143779371652733    |  265920 | 5994240 | 2025/09/20 | akatz-ai   |
| 2,649,984 | 130449915832690    |  786608 | 4163520 | 2025/09/20 | akatz-ai   |
| 2,316,064 | 19907909658842     | 1572070 | 3668360 | 2025/09/18 | Tremeschin |
| 2,033,040 | 250673273362854    | 7337600 | 5768320 | 2025/09/18 | Tremeschin |
| 1,992,096 | 281351900698438    | 4716910 | 4718668 | 2025/09/18 | Tremeschin |

</div>

### 🔵 Largest monoliths near spawn

<div align="center">

| Area      | Seed       | Date       | Found by   |
| :-------: | :--------: | :--------: | :--------: |
| 1,745,664 | 4609608251 | 2025/08/13 | Tremeschin |
| 1,584,112 | 1847066092 | 2025/08/13 | Tremeschin |
| 1,420,816 | 2045872561 | 2025/08/13 | Tremeschin |
| 1,371,824 | 3847304212 | 2025/08/13 | Tremeschin |
| 1,369,360 | 1593912439 | 2025/08/13 | Tremeschin |
| 1,345,520 | 4563197188 | 2025/08/13 | Tremeschin |
| 1,305,472 | 4432659853 | 2025/08/13 | Tremeschin |

</div>

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
