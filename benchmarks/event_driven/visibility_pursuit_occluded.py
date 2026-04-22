class Sphere:
    def __init__(self, name, x, y, z):
        self.name = name
        self.position = {"x": x, "y": y, "z": z}
        self.velocity = {"x": 0.0, "y": 0.0, "z": 0.0}


def line_segment_intersects_box(start, end, box_min, box_max):
    delta = {axis: end[axis] - start[axis] for axis in ("x", "y", "z")}
    t_min = 0.0
    t_max = 1.0
    for axis in ("x", "y", "z"):
        if abs(delta[axis]) < 1e-9:
            if start[axis] < box_min[axis] or start[axis] > box_max[axis]:
                return False
            continue
        inv_delta = 1.0 / delta[axis]
        near = min((box_min[axis] - start[axis]) * inv_delta, (box_max[axis] - start[axis]) * inv_delta)
        far = max((box_min[axis] - start[axis]) * inv_delta, (box_max[axis] - start[axis]) * inv_delta)
        t_min = max(t_min, near)
        t_max = min(t_max, far)
        if t_min > t_max:
            return False
    return True


def visibility_event(observer, target, wall_min, wall_max):
    return not line_segment_intersects_box(observer.position, target.position, wall_min, wall_max)


def candidate_event():
    return [
        {"label": "hold", "velocity": {"x": 0.0, "y": 0.0, "z": 0.0}, "score": 5},
        {"label": "pursue", "velocity": {"x": 1.0, "y": 0.0, "z": 0.0}, "score": 5},
    ]


def fire_visibility_preference(candidates, visible):
    if visible:
        for candidate in candidates:
            if candidate["label"] == "pursue":
                candidate["score"] += 0.1
    candidates.sort(key=lambda candidate: (-candidate["score"], candidate["label"]))
    return candidates[0]


def simulate():
    a = Sphere("A", 0.0, 0.0, 0.0)
    b = Sphere("B", 4.0, 0.0, 0.0)
    wall_min = {"x": 1.0, "y": -1.0, "z": -1.0}
    wall_max = {"x": 3.0, "y": 1.0, "z": 1.0}
    selected = fire_visibility_preference(candidate_event(), visibility_event(a, b, wall_min, wall_max))
    a.velocity = selected["velocity"]
    return {"selected": selected["label"], "velocity": a.velocity}

