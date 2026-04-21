import matplotlib.pyplot as plt
import numpy as np
from util import hash_seed, center_text
import time

BOLD = "\033[1m"
END = "\033[0m"


def get_params():

    length = 64
    print(BOLD + center_text("Length") + END)
    print(center_text("The length of the canvas."))
    while True:
        try:
            length = int(input("Enter an integer: "))
            break
        except ValueError:
            continue

    width = 64
    print(BOLD + center_text("Width") + END)
    print(center_text("The width of the canvas."))
    while True:
        try:
            width = int(input("Enter an integer: "))
            break
        except ValueError:
            continue

    period = 1.0
    print(BOLD + center_text("Period") + END)
    print(
        center_text(
            "The period of the noise. Higher values will make it more spread out, while lower values will make it more compact."
        )
    )
    while True:
        try:
            period = float(input("Enter a float: "))
            break
        except ValueError:
            continue

    hurst = 1.0
    print(BOLD + center_text("Hurst exponent") + END)
    print(
        center_text(
            "Describes the correlation of the fBm. Higher values will be smoother."
        )
    )
    print(
        center_text(
            "H > 0.5: positively correlated; the standard for terrain generation."
        )
    )
    print(center_text("H = 0.5: standard Brownian motion."))
    print(center_text("H < 0.5: negatively correlated."))
    try:
        hurst = max(
            min(float(input("Enter a float between 0 and 1 (default: 1.0): ")), 1.0),
            0.0,
        )  # clamp to 0-1
    except ValueError:
        hurst = 1.0

    lacunarity = 2.0
    print(BOLD + center_text("Lacunarity") + END)
    print(
        center_text(
            "The amount by which to multiply the frequency with each octave. Lower values will appear smoother."
        )
    )
    try:
        lacunarity = float(input("Enter a float (default: 2.0): "))
    except ValueError:
        lacunarity - 2.0

    octaves = 1
    print(BOLD + center_text("Octaves") + END)
    print(
        center_text(
            "The number of octaves for fBm. A higher number of octaves will add finer grained details."
        )
    )
    try:
        octaves = int(input("Enter an integer (default: 1): "))
    except ValueError:
        octaves = 1

    contrast = 1.0
    print(BOLD + center_text("Contrast") + END)
    print(center_text("A scalar to apply after the fBm calculation."))
    try:
        contrast = float(input("Enter a float (default: 1.0): "))
    except ValueError:
        contrast = 1.0

    exp = 1.0
    print(BOLD + center_text("Exponent") + END)
    print(
        center_text(
            "An exponent to apply after the fBm calculation. Higher values will push the values toward 0, and lower values will push the values toward 1."
        )
    )
    try:
        exp = float(input("Enter a float (default: 1.0): "))
    except ValueError:
        exp = 1.0

    offset = 0.0
    print(BOLD + center_text("Offset") + END)
    print(center_text("A value to add to the height after the fBm calculation."))
    try:
        offset = float(input("Enter a float (default: 0.0): "))
    except ValueError:
        offset = 0.0

    ridges = 0
    print(BOLD + center_text("Mode") + END)
    print(center_text("Standard: Default fBm calculations."))
    print(
        center_text(
            "Ridges: Takes the absolute value of the signed noise and inverts to mimic mountain ridges."
        )
    )
    print(
        center_text(
            "Valleys: Takes the absolute value of the signed noise to mimic valleys."
        )
    )
    ridges_str = input("Choose (S/r/v): ")
    if ridges_str in ["R", "r"]:
        ridges = 1
    elif ridges_str in ["V", "v"]:
        ridges = 2
    else:
        ridges = 0

    seed = 0
    print(BOLD + center_text("Seed") + END)
    print(center_text("The seed for the noise."))
    try:
        seed_str = input("Enter a seed (optional): ")
        seed = int(seed_str)
    except ValueError:
        if seed_str == "":
            seed_str = time.ctime()
        seed = hash_seed(seed_str)

    normalize = True
    print(BOLD + center_text("Normalize heightmap?") + END)
    print(
        center_text(
            "Map the shades of the heightmap relative to the values rather than measuring absolutely."
        )
    )
    normalize_str = input("Choose (Y/n): ")
    if normalize in ["N", "n"]:
        normalize = False
    else:
        normalize = True

    print()

    return (
        length,
        width,
        period,
        hurst,
        lacunarity,
        octaves,
        contrast,
        exp,
        offset,
        ridges,
        seed,
        normalize,
    )


def draw_heightmap(heightmap, length, width, normalize):
    vmin = 0.0
    vmax = 1.0

    if normalize:
        vmin = np.min(heightmap)
        vmax = np.max(heightmap)

    plt.figure(figsize=(length, width), dpi=1, layout="tight", clear=True)
    plt.imshow(heightmap, cmap="gray", vmin=vmin, vmax=vmax)
    plt.axis("off")
    plt.savefig("heightmap.png")
    print("Heightmap saved to heightmap.png")


def draw_line_plot(heightmap, length):
    fig = plt.figure(figsize=(8, 6), clear=True)
    ax = fig.add_subplot()
    ax.set_xlim(left=0, right=(length - 1))

    line = ax.plot(heightmap[0], color="black")
    plt.savefig("plot.png")
    print("2D plot saved to plot.png")


def draw_surface(heightmap, length, width):
    fig = plt.figure(figsize=(8, 5), clear=True)
    ax = fig.add_subplot(111, projection="3d")
    x = np.array(range(0, length))
    y = np.array(range(0, width))
    X, Y = np.meshgrid(x, y)

    surface = ax.plot_surface(X, Y, heightmap, cmap="magma", vmin=0.0, vmax=1.0)
    ax.set_zbound(lower=0.0, upper=1.0)
    plt.colorbar(surface)
