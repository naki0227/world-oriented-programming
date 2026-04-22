from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    world.candidate_velocity("A", "fast", velocity=(6, 0, 0), score=5)
    world.candidate_velocity("A", "safe", velocity=(3, 0, 0), score=2)
    world.law("speed_limit", "A", policy={"max": 4})
    world.observe(0)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

