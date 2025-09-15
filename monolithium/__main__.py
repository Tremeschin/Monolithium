import sys
from typing import NoReturn

from monolithium import cudalith, rustlith


def _rustlith() -> NoReturn:
    sys.exit(rustlith().returncode)

def _cudalith() -> NoReturn:
    sys.exit(cudalith().returncode)
