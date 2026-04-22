class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def speed(vector):
    return sum(component * component for component in vector) ** 0.5


def candidate_velocity_event():
    return [
        {"label": "alpha", "velocity": (3, 0, 0), "score": 5},
        {"label": "beta", "velocity": (2, 0, 0), "score": 5},
    ]


def analyze_resolution(candidates):
    admissible = [candidate for candidate in candidates if speed(candidate["velocity"]) <= 4]
    admissible.sort(key=lambda candidate: (-candidate["score"], candidate["label"]))
    top_score = admissible[0]["score"]
    top = [candidate for candidate in admissible if candidate["score"] == top_score]
    if len(top) > 1:
        return {"status": "deferred", "top_labels": [candidate["label"] for candidate in top]}
    return {"status": "resolved", "selected": top[0]}


def simulate():
    sphere = Sphere(position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    resolution = analyze_resolution(candidate_velocity_event())
    if resolution["status"] == "resolved":
        sphere.velocity = list(resolution["selected"]["velocity"])
    return {
        "A": (tuple(sphere.position), tuple(sphere.velocity)),
        "resolution": resolution,
    }

