class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


ZONE_MIN_X = 2
ZONE_MAX_X = 4


def advance(sphere, time):
    sphere.position[0] += sphere.velocity[0] * time
    sphere.position[1] += sphere.velocity[1] * time
    sphere.position[2] += sphere.velocity[2] * time


def time_to_region_entry(sphere):
    if sphere.velocity[0] <= 0:
        return None
    return (ZONE_MIN_X - sphere.position[0]) / sphere.velocity[0]


def fire_region_clamp(sphere):
    sphere.position[0] = ZONE_MIN_X
    sphere.velocity[0] = 0


def snapshot(sphere):
    return (tuple(sphere.position), tuple(sphere.velocity))


def simulate():
    sphere = Sphere(position=(0, 0, 0), velocity=(1, 0, 0), radius=1)
    observations = [1, 3]
    snapshots = {}
    current_time = 0.0

    for target_time in observations:
        event_delta = time_to_region_entry(sphere)
        remaining = target_time - current_time
        if event_delta is not None and event_delta <= remaining:
            advance(sphere, event_delta)
            current_time += event_delta
            fire_region_clamp(sphere)
            remaining = target_time - current_time
        advance(sphere, remaining)
        current_time = target_time
        snapshots[target_time] = snapshot(sphere)

    return snapshots

