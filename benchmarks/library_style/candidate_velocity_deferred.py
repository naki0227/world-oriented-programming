from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    world.candidate_velocity("A", "alpha", velocity=(3, 0, 0), score=5)
    world.candidate_velocity("A", "beta", velocity=(2, 0, 0), score=5)
    world.law("speed_limit", "A", policy={"max": 4})
    world.law("defer_on_ambiguous_top", "A")
    world.observe(0)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

