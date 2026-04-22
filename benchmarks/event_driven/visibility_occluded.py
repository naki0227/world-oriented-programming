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


def visibility_event(observer, target, wall_min, wall_max):
    blocked = line_segment_intersects_box(observer.position, target.position, wall_min, wall_max)
    return {"kind": "visibility", "blocked": blocked, "targets": [observer.name, target.name]}


def fire_visibility_law(event):
    if event["blocked"]:
        raise RuntimeError(f"{event['targets'][0]} cannot see {event['targets'][1]}")


def simulate():
    a = Sphere("A", 0.0, 0.0, 0.0)
    b = Sphere("B", 4.0, 0.0, 0.0)
    wall_min = {"x": 1.0, "y": -1.0, "z": -1.0}
    wall_max = {"x": 3.0, "y": 1.0, "z": 1.0}
    event = visibility_event(a, b, wall_min, wall_max)
    fire_visibility_law(event)
    return {"A": a.position, "B": b.position}

