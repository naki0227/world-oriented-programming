class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


def clamp_speed(vector, limit):
    magnitude = sum(component * component for component in vector) ** 0.5
    if magnitude <= limit or magnitude == 0:
        return vector
    scale = limit / magnitude
    return [component * scale for component in vector]


def step(sphere, dt):
    sphere.position[0] += sphere.velocity[0] * dt
    sphere.position[1] += sphere.velocity[1] * dt
    sphere.position[2] += sphere.velocity[2] * dt

    if sphere.position[1] - sphere.radius <= 0:
        sphere.position[1] = sphere.radius
        sphere.velocity[1] = -sphere.velocity[1]

    sphere.velocity = clamp_speed(sphere.velocity, 3.5)


def simulate():
    sphere = Sphere(position=(0, 10, 0), velocity=(1, -3, 0), radius=1)
    snapshots = {}
    target_times = [0, 1, 3, 4]
    current_time = 0.0
    dt = 1.0

    snapshots[0] = (tuple(sphere.position), tuple(sphere.velocity))
    while current_time < max(target_times):
        step(sphere, dt)
        current_time += dt
        if current_time in target_times:
            snapshots[current_time] = (tuple(sphere.position), tuple(sphere.velocity))

    return snapshots
