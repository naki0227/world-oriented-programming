class Sphere:
    def __init__(self, name, x, y, z):
        self.name = name
        self.position = {"x": x, "y": y, "z": z}


def line_segment_intersects_box(start, end, box_min, box_max):
    delta = {
        "x": end["x"] - start["x"],
        "y": end["y"] - start["y"],
        "z": end["z"] - start["z"],
    }
    t_min = 0.0
    t_max = 1.0

    for axis in ("x", "y", "z"):
        if abs(delta[axis]) < 1e-9:
            if start[axis] < box_min[axis] or start[axis] > box_max[axis]:
                return False
            continue

        inv_delta = 1.0 / delta[axis]
        t1 = (box_min[axis] - start[axis]) * inv_delta
        t2 = (box_max[axis] - start[axis]) * inv_delta
        near = min(t1, t2)
        far = max(t1, t2)
        t_min = max(t_min, near)
        t_max = min(t_max, far)

        if t_min > t_max:
            return False

    return True


def assert_visible(observer, target, wall_min, wall_max):
    if line_segment_intersects_box(observer.position, target.position, wall_min, wall_max):
        raise RuntimeError(f"{observer.name} cannot see {target.name} through wall")


def simulate():
    a = Sphere("A", 0.0, 0.0, 0.0)
    b = Sphere("B", 4.0, 0.0, 0.0)
    wall_min = {"x": 1.0, "y": -1.0, "z": -1.0}
    wall_max = {"x": 3.0, "y": 1.0, "z": 1.0}
    assert_visible(a, b, wall_min, wall_max)
    return {"A": a.position, "B": b.position}


if __name__ == "__main__":
    simulate()
