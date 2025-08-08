import subprocess
import sys

from monolithium import Paths, Tools


def cuda():

    # Generate meson build files
    if code := subprocess.run((
        *Tools.MESON,
        "setup", Paths.BUILD,
        "--buildtype", "release",
        "--reconfigure"
    )).returncode != 0:
        sys.exit(code)

    # Compile the cuda part
    if code := subprocess.run((
        *Tools.NINJA, "-C",
        Paths.BUILD
    )).returncode != 0:
        sys.exit(code)

    # Execute the built binary
    if code := subprocess.run((
        Paths.BUILD/"monolithium",
        *sys.argv[1:],
    )).returncode != 0:
        sys.exit(code)
