class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def speed(vector):
    return sum(component * component for component in vector) ** 0.5


def analyze_candidates():
    candidates = [
        ("alpha", (3, 0, 0), 5),
        ("beta", (2, 0, 0), 5),
    ]

    admissible = []
    for label, velocity, score in candidates:
        if speed(velocity) <= 4:
            admissible.append((label, velocity, score))

    admissible.sort(key=lambda item: (-item[2], item[0]))
    top_score = admissible[0][2]
    top_labels = [label for label, _, score in admissible if score == top_score]

    unresolved = len(top_labels) > 1
    selected = None if unresolved else admissible[0]
    return {
        "top_labels": top_labels,
        "unresolved": unresolved,
        "selected": selected,
    }


def simulate():
    sphere = Sphere(position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    resolution = analyze_candidates()
    if resolution["selected"] is not None:
        _, velocity, _ = resolution["selected"]
        sphere.velocity = list(velocity)
    return {
        "A": (tuple(sphere.position), tuple(sphere.velocity)),
        "resolution": resolution,
    }
