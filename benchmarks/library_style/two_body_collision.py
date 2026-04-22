from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 2, 0), velocity=(1, 0, 0), radius=1)
    world.sphere("B", position=(4, 2, 0), velocity=(-1, 0, 0), radius=1)
    world.law("elastic_collision", "A", "B")
    world.observe(0)
    world.observe(1)
    world.observe(3)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

