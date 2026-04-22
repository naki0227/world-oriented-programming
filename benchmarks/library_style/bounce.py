from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 10, 0), velocity=(1, -3, 0), radius=1)
    world.plane("floor", normal=(0, 1, 0), offset=0)
    world.law("reflect_on_collision", "A", "floor")
    world.law("speed_limit", "A", policy={"max": 3.5})
    world.observe(0)
    world.observe(1)
    world.observe(3)
    world.observe(4)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

