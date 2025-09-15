import json
import sys
from subprocess import PIPE

from attrs import Factory, define, field

from monolithium import rustlith

# Block Altair from importing jupyter
sys.modules["anywidget"] = None

import altair

# ---------------------------------------------------------------------------- #

@define
class Monolith:
    area: int
    seed: int

    # Position
    minx: int
    maxx: int
    minz: int
    maxz: int

# ---------------------------------------------------------------------------- #

@define
class Distribution:
    """Investigate the distribution of monoliths in one or many worlds"""

    monoliths: list[Monolith] = Factory(list)

    @classmethod
    def multi(cls) -> None:
        self = cls()
        run = rustlith("spawn", "linear", "-t", int(50e3), stdout=PIPE)

        # Fixme: Not saying it's ideal to read stdout, could fill buffer
        for line in run.stdout.decode("utf-8").splitlines():
            if not line.startswith("json"):
                continue
            line = line.removeprefix("json")
            mono = Monolith(**json.loads(line))
            self.monoliths.append(mono)

    @classmethod
    def world(cls) -> None:
        ...
