class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def advance(sphere, time):
    sphere.position[0] += sphere.velocity[0] * time
    sphere.position[1] += sphere.velocity[1] * time
    sphere.position[2] += sphere.velocity[2] * time


def speed(vector):
    return sum(component * component for component in vector) ** 0.5


def clamp_speed(sphere, limit):
    magnitude = speed(sphere.velocity)
    if magnitude <= limit or magnitude == 0:
        return
    scale = limit / magnitude
    sphere.velocity = [component * scale for component in sphere.velocity]


def time_to_floor(sphere):
    if sphere.velocity[1] >= 0:
        return None
    return (sphere.radius - sphere.position[1]) / sphere.velocity[1]


def fire_floor_reflection(sphere):
    sphere.position[1] = sphere.radius
    sphere.velocity[1] = -sphere.velocity[1]


def snapshot(sphere):
    return (tuple(sphere.position), tuple(sphere.velocity))


def simulate():
    sphere = Sphere(position=(0, 10, 0), velocity=(1, -3, 0), radius=1)
    observations = [0, 1, 3, 4]
    snapshots = {0: snapshot(sphere)}
    current_time = 0.0

    for target_time in observations[1:]:
        while True:
            event_delta = time_to_floor(sphere)
            remaining = target_time - current_time
            if event_delta is None or event_delta > remaining:
                advance(sphere, remaining)
                current_time = target_time
                break
            advance(sphere, event_delta)
            current_time += event_delta
            fire_floor_reflection(sphere)
            clamp_speed(sphere, 3.5)
        snapshots[target_time] = snapshot(sphere)

    return snapshots

