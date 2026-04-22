class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def advance(sphere, time):
    sphere.position[0] += sphere.velocity[0] * time
    sphere.position[1] += sphere.velocity[1] * time
    sphere.position[2] += sphere.velocity[2] * time


def time_to_collision(a, b):
    relative_position = b.position[0] - a.position[0]
    relative_velocity = a.velocity[0] - b.velocity[0]
    if relative_velocity <= 0:
        return None
    return (relative_position - a.radius - b.radius) / relative_velocity


def fire_elastic_collision(a, b):
    a.velocity[0], b.velocity[0] = b.velocity[0], a.velocity[0]


def snapshot(a, b):
    return {
        "A": (tuple(a.position), tuple(a.velocity)),
        "B": (tuple(b.position), tuple(b.velocity)),
    }


def simulate():
    a = Sphere(position=(0, 2, 0), velocity=(1, 0, 0), radius=1)
    b = Sphere(position=(4, 2, 0), velocity=(-1, 0, 0), radius=1)
    observations = [0, 1, 3]
    snapshots = {0: snapshot(a, b)}
    current_time = 0.0

    for target_time in observations[1:]:
        while True:
            event_delta = time_to_collision(a, b)
            remaining = target_time - current_time
            if event_delta is None or event_delta > remaining:
                advance(a, remaining)
                advance(b, remaining)
                current_time = target_time
                break
            advance(a, event_delta)
            advance(b, event_delta)
            current_time += event_delta
            fire_elastic_collision(a, b)
        snapshots[target_time] = snapshot(a, b)

    return snapshots

