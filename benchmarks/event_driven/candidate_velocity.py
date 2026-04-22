class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def speed(vector):
    return sum(component * component for component in vector) ** 0.5


def candidate_velocity_event():
    return [
        {"label": "fast", "velocity": (6, 0, 0), "score": 5},
        {"label": "safe", "velocity": (3, 0, 0), "score": 2},
    ]


def enforce_speed_law(candidate, limit):
    return speed(candidate["velocity"]) <= limit


def fire_candidate_resolution(candidates):
    admissible = [candidate for candidate in candidates if enforce_speed_law(candidate, 4)]
    if not admissible:
        return None
    admissible.sort(key=lambda candidate: (-candidate["score"], candidate["label"]))
    return admissible[0]


def simulate():
    sphere = Sphere(position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    selected = fire_candidate_resolution(candidate_velocity_event())
    if selected is not None:
        sphere.velocity = list(selected["velocity"])
    return {
        "A": (tuple(sphere.position), tuple(sphere.velocity)),
        "selected": None if selected is None else selected["label"],
    }

