from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 0, 0))
    world.sphere("B", position=(4, 0, 0))
    world.region("wall", minimum=(1, -1, -1), maximum=(3, 1, 1))
    world.law("visible", "A", "B")
    world.observe(0)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

