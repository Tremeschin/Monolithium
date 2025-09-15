import shutil
import subprocess
import sys
from pathlib import Path
from subprocess import PIPE, CompletedProcess
from typing import Optional


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


def monorust(args: Optional[list[str]]=None) -> CompletedProcess:
    """Run the Rust version of Monolithium"""

    # Have a rust toolchain
    if subprocess.run(
        (*Tools.RUSTUP, "run", "stable", "rustc"),
        stdout=PIPE, stderr=PIPE
    ).returncode != 0:
        subprocess.check_call((
            *Tools.RUSTUP,
            "default", "stable"
        ))

    # Run the project
    return subprocess.run((
        *Tools.CARGO,
        "run", "--release",
        "--", *(args or sys.argv[1:]),
    ), cwd=Paths.PACKAGE)

def monocuda(args: Optional[list[str]]=None) -> CompletedProcess:
    """Run the CUDA version of Monolithium"""

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

    # Execute the built binary
    return subprocess.run((
        Paths.BUILD/"monolithium",
        *(args or sys.argv[1:]),
    ))
