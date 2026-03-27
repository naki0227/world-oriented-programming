class Sphere:
    def __init__(self, name, x, y, z):
        self.name = name
        self.position = {"x": x, "y": y, "z": z}
        self.velocity = {"x": 0.0, "y": 0.0, "z": 0.0}


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


def can_see(observer, target, occluders):
    for box_min, box_max in occluders:
        if line_segment_intersects_box(observer.position, target.position, box_min, box_max):
            return False
    return True


def resolve_velocity(a, b, occluders):
    candidates = [
        {"label": "hold", "velocity": {"x": 0.0, "y": 0.0, "z": 0.0}, "score": 5.0},
        {"label": "pursue", "velocity": {"x": 1.0, "y": 0.0, "z": 0.0}, "score": 5.0},
        {"label": "search", "velocity": {"x": 0.0, "y": 1.0, "z": 0.0}, "score": 5.0},
    ]

    if can_see(a, b, occluders):
        for candidate in candidates:
            if candidate["label"] == "pursue":
                candidate["score"] += 0.1
    else:
        for candidate in candidates:
            if candidate["label"] == "search":
                candidate["score"] += 0.1

    candidates.sort(key=lambda candidate: (-candidate["score"], candidate["label"]))
    return candidates[0]


def simulate():
    a = Sphere("A", 0.0, 0.0, 0.0)
    b = Sphere("B", 6.0, 0.0, 0.0)
    occluders = [
        ({"x": 1.0, "y": 1.0, "z": -1.0}, {"x": 5.0, "y": 3.0, "z": 1.0}),
        ({"x": 1.0, "y": -3.0, "z": -1.0}, {"x": 5.0, "y": -1.0, "z": 1.0}),
        ({"x": 2.5, "y": -0.5, "z": -1.0}, {"x": 3.5, "y": 0.5, "z": 1.0}),
    ]
    selected = resolve_velocity(a, b, occluders)
    a.velocity = selected["velocity"]
    return {"selected": selected["label"], "velocity": a.velocity}


if __name__ == "__main__":
    simulate()
