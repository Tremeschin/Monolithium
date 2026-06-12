import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path
from subprocess import PIPE, CompletedProcess, Popen
from typing import Union


class Paths:
    PACKAGE: Path = Path(__file__).parent
    REPO:    Path = (PACKAGE.parent)
    BUILD:   Path = (Path.cwd()/"build")

# ---------------------------------------------------------------------------- #

def rustlith(
    *args: str,
    Popen: bool=False,
    toolchain: str="stable",
    **kwargs
) -> Union[CompletedProcess, Popen]:
    """Run the Rust version of Monolithium"""
    args: list[str] = list(map(str, (args or sys.argv[1:])))

    # Have a rust toolchain
    if subprocess.run(
        ("rustup", "run", toolchain, "rustc", "--version"),
        stdout=PIPE, stderr=PIPE
    ).returncode != 0:
        subprocess.check_call((
            "rustup",
            "default", toolchain
        ))

    # Simple features handling via anywhere in args flags
    cargo = tomllib.loads((Paths.PACKAGE/"Cargo.toml").read_text(encoding="utf-8"))
    features = list()

    for feature in cargo["features"]:
        if (flag := f"--{feature}") in args:
            features.append("--features")
            features.append(feature)
            args.remove(flag)

    os.environ.update(RUSTFLAGS="-C target-cpu=native")

    return (subprocess.Popen if Popen else subprocess.run)((
        "cargo", "run",
        "--manifest-path", (Paths.PACKAGE/"Cargo.toml"),
        "--target-dir", str(Path.cwd()/"target"),
        "--release", *features,
        "--", *args,
    ), **kwargs)

# ---------------------------------------------------------------------------- #

def cudalith(
    *args: str,
    Popen: bool=False,
    **kwargs
) -> Union[CompletedProcess, Popen]:
    """Run the CUDA version of Monolithium"""
    args = tuple(map(str, (args or sys.argv[1:])))

    if not shutil.which("nvcc"):
        raise RuntimeError("nvcc wasn't found in path, do you have cuda toolkit?")

    # Generate meson build files
    subprocess.check_call((
        sys.executable,
        "-m", "mesonbuild.mesonmain",
        "setup", Paths.BUILD,
        "--buildtype", "release",
        "--reconfigure"
    ), cwd=Paths.PACKAGE)

    # Compile the cuda part
    subprocess.check_call((
        sys.executable,
        "-m", "ninja",
        "-C", Paths.BUILD
    ))

    return (subprocess.Popen if Popen else subprocess.run)((
        Paths.BUILD/"cudalith",
        *args
    ), **kwargs)
