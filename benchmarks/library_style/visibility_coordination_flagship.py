from worldlib import World


def build_world():
    world = World()
    world.sphere("A", position=(0, 0.5, 0), velocity=(0, 0, 0), radius=0.5)
    world.sphere("D", position=(0, -0.5, 0), velocity=(0, 0, 0), radius=0.5)
    world.sphere("B", position=(6, 2, 0), velocity=(0, -2, 0), radius=0.5)
    world.sphere("C", position=(6, -4, 0), velocity=(0, 2, 0), radius=0.5)
    world.plane("floor", normal=(0, 1, 0), offset=0)
    world.region("wall_top", minimum=(1, 1, -1), maximum=(5, 3, 1))
    world.region("wall_bottom", minimum=(1, -3, -1), maximum=(5, -1, 1))
    world.law("speed_limit", "A", policy={"max": 2})
    world.law("speed_limit", "D", policy={"max": 2})

    world.candidate_velocity("A", "hold_a", velocity=(0, 0, 0), score=5)
    world.candidate_velocity("A", "pursue_b", velocity=(1, 1, 0), score=5)
    world.candidate_velocity("A", "pursue_c", velocity=(1, -1, 0), score=5)
    world.law("defer_on_ambiguous_top", "A")
    world.law("resolve_deferred_at", "A", policy={"time": 1})
    world.prefer_if_visible("A", "pursue_b", "B")
    world.prefer_if_visible("A", "pursue_c", "C")

    world.candidate_velocity("D", "hold_d", velocity=(0, 0, 0), score=5)
    world.candidate_velocity("D", "support_b", velocity=(1, 1, 0), score=5)
    world.candidate_velocity("D", "support_c", velocity=(1, -1, 0), score=5)
    world.law("defer_on_ambiguous_top", "D")
    world.law("resolve_deferred_at", "D", policy={"time": 2})
    world.prefer_if_visible("D", "support_b", "B")
    world.prefer_if_visible("D", "support_c", "C")

    world.observe(0)
    world.observe(1)
    world.observe(2)
    return world


def simulate():
    return build_world().simulate()


if __name__ == "__main__":
    simulate()

