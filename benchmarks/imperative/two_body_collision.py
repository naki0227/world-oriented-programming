class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def collide(a, b):
    distance = abs(a.position[0] - b.position[0])
    if distance <= a.radius + b.radius:
        a.velocity[0], b.velocity[0] = b.velocity[0], a.velocity[0]


def step(spheres, dt):
    for sphere in spheres:
        sphere.position[0] += sphere.velocity[0] * dt
        sphere.position[1] += sphere.velocity[1] * dt
        sphere.position[2] += sphere.velocity[2] * dt

    collide(spheres[0], spheres[1])


def simulate():
    a = Sphere(position=(0, 2, 0), velocity=(1, 0, 0), radius=1)
    b = Sphere(position=(4, 2, 0), velocity=(-1, 0, 0), radius=1)
    snapshots = {}
    target_times = [0, 1, 3]
    current_time = 0.0
    dt = 1.0

    snapshots[0] = {
        "A": (tuple(a.position), tuple(a.velocity)),
        "B": (tuple(b.position), tuple(b.velocity)),
    }

    while current_time < max(target_times):
        step([a, b], dt)
        current_time += dt
        if current_time in target_times:
            snapshots[current_time] = {
                "A": (tuple(a.position), tuple(a.velocity)),
                "B": (tuple(b.position), tuple(b.velocity)),
            }

    return snapshots
