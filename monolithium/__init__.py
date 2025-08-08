import sys
from pathlib import Path


class Paths:
    PACKAGE: Path = Path(__file__).parent
    REPO:    Path = (PACKAGE.parent)
    BUILD:   Path = (REPO/"build")

class Tools:
    PYTHON: tuple[str] = (sys.executable,)
    MESON:  tuple[str] = (*PYTHON, "-m", "mesonbuild.mesonmain")
    NINJA:  tuple[str] = (*PYTHON, "-m", "ninja")
