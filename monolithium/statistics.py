import json
import sys
from abc import ABC, abstractmethod
from subprocess import PIPE
from typing import Iterable

import numpy
from attrs import Factory, define

from monolithium import rustlith

# Block Altair from importing jupyter
sys.modules["anywidget"] = None

import altair

MILLION: int = 1_000_000

# ---------------------------------------------------------------------------- #

@define(eq=False)
class Monolith:
    area: int
    seed: int

    # Position
    minx: int
    maxx: int
    minz: int
    maxz: int

    # Note: Area is approximate, not on hash
    def __hash__(self) -> int:
        return hash((
            self.seed,
            self.minx, self.maxx,
            self.minz, self.maxz,
        ))

# ---------------------------------------------------------------------------- #

class FitModel(ABC):

    @abstractmethod
    def __call__(self, *args: float) -> float:
        """Model function to fit against data"""
        ...

    @abstractmethod
    def function(self, *scalars: float) -> str:
        """Return a math function string from fitted scalars"""
        ...

class Models:
    class Exponential(FitModel):
        """Simple y = exp(ax)"""

        def __call__(self, x: float, a: float, b: float) -> float:
            return a*numpy.exp(b*x)

        def function(self, *scalars: float) -> str:
            a, b = scalars
            return f"y = ({a:.2f}) e^({b:.8f}x)"

# ---------------------------------------------------------------------------- #

@define
class Distribution:
    """Investigate the distribution of monoliths in one or many worlds"""

    monoliths: list[Monolith] = Factory(list)

    @property
    def sorted_areas(self) -> Iterable[int]:
        yield from sorted(mono.area for mono in self.monoliths)

    def filter_unique(self) -> None:
        self.monoliths = list(set(self.monoliths))

    def smart_rustlith(self, *args: list[str]) -> None:
        process = rustlith(*args, stdout=PIPE, Popen=True)

        for line in process.stdout.readlines():
            line = line.decode("utf-8").strip()
            if not line.startswith("json"):
                continue
            line = line.removeprefix("json")
            mono = Monolith(**json.loads(line))
            self.monoliths.append(mono)

    # -------------------------------- #

    def multi(self) -> None:
        self.smart_rustlith(
            "spawn", "--radius", 200, "--step", 50,
            "random", "--total", int(50e6),
            "--fast",
        )

        def points() -> Iterable[tuple[float, float]]:
            for i, area in enumerate(self.sorted_areas):
                y = 100*(1 - i/len(self.monoliths))
                x = area
                yield (x, y)

        self.simple_chart(
            points=points(),
            title="Monolith Area Distribution (multi world)",
            xname="Area",
            yname="Monoliths (%)",
            fit=Models.Exponential(),
            fit_scale_x=(1/MILLION),
            fit_scale_y=(1/100),
        ).save(
            fp="/tmp/multi.png",
            scale_factor=3.0
        )

    def world(self,
        seed: int=617,
        step: int=512,
    ) -> None:
        self.smart_rustlith("find",
            "--step", step,
            "--seed", seed,
        )
        self.filter_unique()

        def points() -> Iterable[tuple[float, float]]:
            for i, area in enumerate(self.sorted_areas):
                y = len(self.monoliths) - i
                x = area
                yield (x, y)

        self.simple_chart(
            points=points(),
            title=f"Monolith Area Distribution (seed={seed})",
            xname="Area",
            yname="Number of Monoliths",
            fit=Models.Exponential(),
            fit_scale_x=(1/MILLION),
        ).save(
            fp="/tmp/world.png",
            scale_factor=3.0
        )

    # -------------------------------- #

    def simple_chart(self,
        points: Iterable[tuple[float, float]]=None,
        title: str="Untitled Chart",
        xname: str="Untitled X Axis",
        yname: str="Untitled Y Axis",
        fit: FitModel=None,
        fit_scale_x: float=1.0,
        fit_scale_y: float=1.0,
    ) -> altair.Chart:
        """Make a simple scatter plot with a fit line"""
        from scipy.optimize import curve_fit

        # Unpack iterable of points
        points = list(points or [])
        x = tuple(xi for xi, _ in points)
        y = tuple(yi for _, yi in points)

        # Apply fitting and show on title
        if (fit is not None):
            scalars, covariance = curve_fit(f=fit,
                xdata=list(n*fit_scale_x for n in x),
                ydata=list(n*fit_scale_y for n in y),
            )
            title = f"{title} • {fit.function(*scalars)}"

        # Convert to a dataframe-like structure
        chart = altair.Chart(altair.Data(values=[
            dict(x=xi, y=yi) for xi, yi in zip(x, y)
        ])).mark_line().encode(
            x=altair.X("x:Q", title=xname),
            y=altair.Y("y:Q", title=yname),
        ).properties(
            title=title,
            width=1920/2,
            height=720/2,
        )

        return chart

    # -------------------------------- #

    def heatmap(self,

    ) -> altair.Chart:
        self.smart_rustlith(
            "spawn",
            # "--chunks", 1,
            # "--radius", 262144*2,
            "--radius", 2**19,
            "--step", 256,
            "linear",
            "--total", 1000,
            # "--candidates",
            # "--only-hill",
            # "--fast"
        )

        # Plot the (X, Z) positions of monoliths, with circle size based on area
        chart = altair.Chart(altair.Data(values=[
            dict(x=mono.minx, z=mono.minz, a=mono.area) for mono in self.monoliths
        ])).mark_circle().encode(
            x=altair.X("x:Q", title="X"),
            y=altair.Y("z:Q", title="Z"),
            size=altair.Size("a:Q", title="Area", scale=altair.Scale(range=[1, 100]))
        ).properties(
            title="Monolith Positions",
            width=1920/2,
            height=1080/2,
        )

        # Write a red bounding box
        for v, c in ((2**18, "red"), (2**19, "green")):
            chart += altair.Chart(altair.Data(values=[
                dict(x= v, z= v),
                dict(x= v, z=-v),
                dict(x=-v, z=-v),
                dict(x=-v, z= v),
                dict(x= v, z= v),
                dict(x= v, z=-v),
            ])).mark_line(color=c).encode(
                x=altair.X("x:Q"),
                y=altair.Y("z:Q"),
            )

        return chart

# ---------------------------------------------------------------------------- #

def main() -> None:
    stats = Distribution()
    # stats.multi()
    # stats.world()
    stats.heatmap().save(
        fp="/tmp/heatmap.png",
        scale_factor=3.0
    )
