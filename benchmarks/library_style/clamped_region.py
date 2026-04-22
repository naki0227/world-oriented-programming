from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 0, 0), velocity=(1, 0, 0), radius=1)
    world.plane("floor", normal=(0, 1, 0), offset=-10)
    world.region("zone", minimum=(2, -1, -1), maximum=(4, 1, 1))
    world.law("not_inside", "A", "zone", policy="clamp")
    world.observe(1)
    world.observe(3)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

