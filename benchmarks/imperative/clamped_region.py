class Sphere:
    def __init__(self, position, velocity, radius):
        self.position = list(position)
        self.velocity = list(velocity)
        self.radius = radius


ZONE_MIN_X = 2
ZONE_MAX_X = 4


def clamp_outside_region(sphere):
    if ZONE_MIN_X <= sphere.position[0] <= ZONE_MAX_X:
        left_distance = abs(sphere.position[0] - ZONE_MIN_X)
        right_distance = abs(ZONE_MAX_X - sphere.position[0])
        if left_distance <= right_distance:
            sphere.position[0] = ZONE_MIN_X
        else:
            sphere.position[0] = ZONE_MAX_X
        sphere.velocity[0] = 0


def step(sphere, dt):
    sphere.position[0] += sphere.velocity[0] * dt
    sphere.position[1] += sphere.velocity[1] * dt
    sphere.position[2] += sphere.velocity[2] * dt
    clamp_outside_region(sphere)


def simulate():
    sphere = Sphere(position=(0, 0, 0), velocity=(1, 0, 0), radius=1)
    snapshots = {}
    target_times = [1, 3]
    current_time = 0.0
    dt = 1.0

    while current_time < max(target_times):
        step(sphere, dt)
        current_time += dt
        if current_time in target_times:
            snapshots[current_time] = (tuple(sphere.position), tuple(sphere.velocity))

    return snapshots
