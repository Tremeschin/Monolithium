<div align="center">
  <img src="https://raw.githubusercontent.com/Tremeschin/Monolithium/main/monolithium/resources/images/logo.png" width="210">
  <h1 style="margin-top: 0">Monolithium</h1>
  <span>🗿 Finding the Largest Minecraft Alpha Monoliths 🗿</span>
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

> [!NOTE]
> Windows users will need to install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) for linkers, or alternatively a [MSYS2](https://www.msys2.org/) environment with the package `mingw-w64-ucrt-x86_64-gcc` and compile in a UCRT64 shell.

Install [Git](https://git-scm.com/downloads) and [astral-sh/uv](https://docs.astral.sh/uv/), open a terminal in some directory and run:
- `git clone https://github.com/Tremeschin/Monolithium && cd Monolithium`
- Rust part: `uv run rustlith (commands)`

You can pass any `--<feature>` explained in [`Cargo.toml`](./monolithium/Cargo.toml) for speedups, like `--fast`!

## 🔥 Commands

> [!WARNING]
> - The code _will_ shred your cpu, make sure you have a good cooling solution!
> - Large directories are created where you run it - any of `target`, `build`, `release`.

You can run any of `rustlith (command) --help` for options and information!

### 🔴 Find all Monoliths in a world

This will search a 8,388,608 blocks square in both positive X and Z directions. Note that all monoliths repeats every such value on any coordinate - there are 9 copies of each within the Far Lands on any given world!

- `rustlith search --depth seed --value 617`

### 🟡 Find seeds with spawn monoliths

This will search for seeds that contains monoliths close to spawn.

- Search 0 through 100k seeds: `rustlith search linear -t 100000`
- Search 50k random seeds: `rustlith search random -t 50000`

### 🟢 2Pass heuristic method

This heuristic finds seeds with _great potential_ for large monoliths, by only looking at the much rarer `hill` noise values, and discarding most seeds with poor `x, y, z` fractional parts offsets.

- `rustlith search --radius 8192 random -t 5000000 --candidates --fast` (potential)
- For each output, run `rustlith search --step 512 seed -v <n>` (real area)

There's a couple improvements to this method:

- Since this discards millions seeds a second, rayon work-stealing parallelism becomes an overhead. Passing `--chunks 1000` to `search` makes each work block process multiple seeds instead of one.
- The `hill` wraps around `2**19` blocks, so `--radius 262144` searches _"the whole world"'s_ potential.
- For long searches, filter out candidates with at least `--area n` to ignore bad shuffling seeds.
- Largest monoliths are basically guaranteed to hit a lattice point multiple of `1024`
- At [`world::good_perlin_fracts`](./monolithium/monolithium/world.rs), one can tweak the treshholds for a "good" seed.

Full command idea that broke many records:

```sh
# or 'cargo run --release --features candidates --features fast' in monolithium dir
$ QUALITY=250.0 rustlith --candidates --fast \
  search --chunks 1000 --hill --step 1024 --area 2200000 \
  random --total 100000000 \
> candidates.txt

# Many minutes/hours later..
$ rustlith search --depth --step 512 --threaded \
  file -i candidates.txt \
> final.txt
```

With enough `--total` seeds, checking the best ones almost guarantees a record :)

### 🟠 Fast spawn search

Instead of initializing a world from zero per work `chunks` loop, this method reutilizes most perlin noises data in a incremental way, rolling down all octaves and appending a new one with the current RNG state.

As such, `--chunks` plays more than one role here, theoretical max performance at `total_seeds/cpu_threads`:

```sh
$ AREA_STEP=8 rustlith --fast --sister-perlin --deque-octaves \
  search --chunks 1000 --area 500000 \
  random -t 50000000
```

## ⭐️ Showcase

> [!NOTE]
> Area calculations are within 1% error, nearby monoliths are part of the same complex.

### 🔵 Largest monoliths anywhere

<div align="center">

| Area      | Seed               | X       | Z       | Date       | Found by   |
| :-------: | :----------------: | ------: | ------: | :--------: | :--------: |
| 3,119,151 |     94116384388573 | 3185395 | 1401244 | 2025/10/06 | Tremeschin |
| 3,099,536 |      1184681998931 | 7361728 | 4530048 | 2025/10/18 | Ferra      |
| 3,047,088 |        24388692462 | 2942560 | 4232256 | 2025/10/18 | Ferra      |
| 3,012,432 |      1102581481210 | 6025536 | 4676000 | 2025/10/18 | Ferra      |
| 2,987,360 |        34786730572 | 4236288 | 5081952 | 2025/10/18 | Ferra      |
| 2,950,096 |       530268432158 | 1384896 | 8037440 | 2025/10/18 | Ferra      |
| 2,892,928 |       873667255313 | 5711040 | 1334432 | 2025/10/18 | Ferra      |
| 2,890,848 |    143779371652733 |  265920 | 5994240 | 2025/09/20 | akatz-ai   |
| 2,649,984 |    130449915832690 |  786608 | 4163520 | 2025/09/20 | akatz-ai   |
| 2,316,064 |     19907909658842 | 1572070 | 3668360 | 2025/09/18 | Tremeschin |
| 2,033,040 |    250673273362854 | 7337600 | 5768320 | 2025/09/18 | Tremeschin |
| 1,992,096 |    281351900698438 | 4716910 | 4718668 | 2025/09/18 | Tremeschin |

</div>

### 🔵 Largest monoliths near spawn

<div align="center">

| Area      | Seed            | Date       | Found by   |
| :-------: | :-------------: | :--------: | :--------: |
| 2,420,304 | 213457002840989 | 2025/10/29 | NathanVH8  |
| 2,240,624 |  38078992166078 | 2025/10/19 | NathanVH8  |
| 1,993,040 | 269943915147763 | 2025/10/18 | NathanVH8  |
| 1,975,712 |  35668441367715 | 2025/10/08 | NathanVH8  |
| 1,745,664 |      4609608251 | 2025/08/13 | Tremeschin |
| 1,584,112 |      1847066092 | 2025/08/13 | Tremeschin |
| 1,420,816 |      2045872561 | 2025/08/13 | Tremeschin |
| 1,371,824 |      3847304212 | 2025/08/13 | Tremeschin |
| 1,369,360 |      1593912439 | 2025/08/13 | Tremeschin |
| 1,345,520 |      4563197188 | 2025/08/13 | Tremeschin |
| 1,305,472 |      4432659853 | 2025/08/13 | Tremeschin |

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

## 🚀 Speeds

Monolithium is written in heavily parallelized [Rust](https://www.rust-lang.org/) with the help of crates like [Rayon](https://crates.io/crates/rayon) for fearless concurrency and [Ahash](https://crates.io/crates/ahash) fast hashing, fully utilizing all available CPU resources one throws at it.

🦀 For a Ryzen 9 5900X 12c/24t, 2x3200 MT/s DDR4 CL16 2Rx8 system, one might expect:

- **3.75 minutes** to find all monoliths in a seed, probing every 128 blocks.
- **Search 1,150,000** seeds per second for spawn monoliths (approximated)
- **Search 440,000** seeds per second for spawn monoliths (accurate)

Such speeds scales about linearly with your hardware - for better or worse!

## 🔎 Future work

- Investigate the correlation of Perlin coefficients to the likeliness and size of Monoliths.
- Is it more efficient for CUDA to stream perlin coefficients than inline JavaRNG on CPU?
- Make statistical analysis (Average size, Distribution) of Monoliths in seeds.
- Make a `HashMap<(int, int), Monolith>` to avoid recomputing areas
- Throw 2x Epyc 9965 at the code. I have a spare one for sure iirc.

## ♻️ Credits

- **User [@kahomayo](https://github.com/kahomayo)** for [`monolith-renderer`](https://github.com/kahomayo/monolith-renderer) to understand the underlying mathematics.
- **YouTuber [@AntVenom](https://www.youtube.com/@AntVenom/)** For the [Breaking Minecraft](https://www.youtube.com/playlist?list=PLR50dP3MW9ZWMSVz2LkRoob_KRf72xcEx) playlist inspiration.
