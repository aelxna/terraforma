import os
import numpy as np
import matplotlib.pyplot as plt
from interface import get_params, draw_heightmap, draw_line_plot, draw_surface


def main():
    print("terraforma v0.1.0")
    (
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
    ) = get_params()

    pid = os.fork()
    if pid == 0:
        args = [
            "./target/release/terraforma",
            str(length),
            str(width),
            str(period),
            str(hurst),
            str(lacunarity),
            str(octaves),
            str(contrast),
            str(exp),
            str(offset),
            str(ridges),
            str(seed),
        ]
        print(" ".join(args))
        os.execv("./target/release/terraforma", args)
    else:
        os.wait()
        print()

    with open("config.txt", "w") as fd:
        fd.write(f"{length}\n")
        fd.write(f"{width}\n")
        fd.write(f"{period}\n")
        fd.write(f"{hurst}\n")
        fd.write(f"{lacunarity}\n")
        fd.write(f"{octaves}\n")
        fd.write(f"{contrast}\n")
        fd.write(f"{exp}\n")
        fd.write(f"{offset}\n")
        fd.write(f"{"s" if ridges == 0 else "r" if ridges == 1 else "v"}\n")
        fd.write(f"{seed}\n")
        fd.write(f"{"y" if normalize else "n"}\n")
    print("Configuration saved to config.txt")

    heightmap = np.loadtxt("heightmap.csv", delimiter=",")

    draw_heightmap(heightmap, length, width, normalize)

    draw_line_plot(heightmap, length)

    draw_surface(heightmap, length, width)

    plt.show()

    return 0


if __name__ == "__main__":
    main()
