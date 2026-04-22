from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 0, 0))
    world.sphere("B", position=(6, 0, 0))
    world.region("wall_top", minimum=(1, 1, -1), maximum=(5, 3, 1))
    world.region("wall_bottom", minimum=(1, -3, -1), maximum=(5, -1, 1))
    world.region("blocker", minimum=(2.5, -0.5, -1), maximum=(3.5, 0.5, 1))
    world.candidate_velocity("A", "hold", velocity=(0, 0, 0), score=5)
    world.candidate_velocity("A", "pursue", velocity=(1, 0, 0), score=5)
    world.candidate_velocity("A", "search", velocity=(0, 1, 0), score=5)
    world.prefer_if_visible("A", "pursue", "B")
    world.prefer_if_occluded("A", "search", "B")
    world.observe(0)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

