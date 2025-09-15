import shutil
import subprocess
import sys
from pathlib import Path
from subprocess import PIPE, CompletedProcess

import tomllib


class Paths:
    PACKAGE: Path = Path(__file__).parent
    REPO:    Path = (PACKAGE.parent)
    BUILD:   Path = (REPO/"build")

class Tools:
    PYTHON: tuple[str] = (sys.executable,)
    MESON:  tuple[str] = (*PYTHON, "-m", "mesonbuild.mesonmain")
    NINJA:  tuple[str] = (*PYTHON, "-m", "ninja")
    RUSTUP: tuple[str] = (shutil.which("rustup"),)
    CARGO:  tuple[str] = (shutil.which("cargo"),)


def monorust(args: list[str]=None) -> CompletedProcess:
    """Run the Rust version of Monolithium"""
    args = (args or sys.argv[1:])

    # Have a rust toolchain
    if subprocess.run(
        (*Tools.RUSTUP, "run", "stable", "rustc"),
        stdout=PIPE, stderr=PIPE
    ).returncode != 0:
        subprocess.check_call((
            *Tools.RUSTUP,
            "default", "stable"
        ))

    # Simple features handling via anywhere in args flags
    cargo = tomllib.loads((Paths.REPO/"Cargo.toml").read_text(encoding="utf-8"))
    features = list()

    for feature in cargo["features"]:
        if (flag := f"--{feature}") in args:
            features.append("--features")
            features.append(feature)
            args.remove(flag)

    return subprocess.run((
        *Tools.CARGO, "run",
        "--release", *features,
        "--", *args,
    ), cwd=Paths.PACKAGE)

def monocuda(args: list[str]=None) -> CompletedProcess:
    """Run the CUDA version of Monolithium"""
    args = (args or sys.argv[1:])

    if not shutil.which("nvcc"):
        raise RuntimeError("nvcc wasn't found in path, do you have cuda toolkit?")

    # Generate meson build files
    subprocess.check_call((
        *Tools.MESON,
        "setup", Paths.BUILD,
        "--buildtype", "release",
        "--reconfigure"
    ), cwd=Paths.REPO)

    # Compile the cuda part
    subprocess.check_call((
        *Tools.NINJA, "-C",
        Paths.BUILD
    ))

    return subprocess.run((
        Paths.BUILD/"monolithium",
        *args
    ))
