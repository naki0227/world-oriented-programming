class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def speed(vector):
    return sum(component * component for component in vector) ** 0.5


def resolve_candidate_velocity():
    candidates = [
        ("fast", (6, 0, 0), 5),
        ("safe", (3, 0, 0), 2),
    ]

    admissible = []
    for label, velocity, score in candidates:
        if speed(velocity) <= 4:
            admissible.append((label, velocity, score))

    if not admissible:
        return None

    admissible.sort(key=lambda item: (-item[2], item[0]))
    return admissible[0]


def simulate():
    sphere = Sphere(position=(0, 2, 0), velocity=(0, 0, 0), radius=1)
    chosen = resolve_candidate_velocity()
    if chosen is not None:
        _, velocity, _ = chosen
        sphere.velocity = list(velocity)
    return {
        "A": (tuple(sphere.position), tuple(sphere.velocity)),
    }
