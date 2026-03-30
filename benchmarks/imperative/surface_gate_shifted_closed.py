from dataclasses import dataclass


@dataclass
class Sphere:
    x: float
    y: float
    vx: float
    vy: float
    radius: float


WALL_X = 0.0
INITIAL_GATE = (4.0, 6.0)
SHIFTED_GATE = (1.0, 3.0)


def shifted_gate_open(current_time: float) -> bool:
    return current_time >= 4.0


def gate_bounds(current_time: float):
    if shifted_gate_open(current_time):
        return SHIFTED_GATE
    return INITIAL_GATE


def choose_velocity(current_time: float):
    if shifted_gate_open(current_time):
        return -2.0, 0.0
    return 0.0, 0.0


def point_in_gate(y: float, bounds):
    return bounds[0] <= y <= bounds[1]


def enforce_gate(sphere: Sphere, previous_x: float, current_time: float):
    crossed = previous_x - sphere.radius >= WALL_X and sphere.x - sphere.radius < WALL_X
    if crossed and not point_in_gate(sphere.y, gate_bounds(current_time)):
        sphere.x = WALL_X + sphere.radius
        if sphere.vx < 0.0:
            sphere.vx = 0.0


def simulate():
    sphere = Sphere(x=2.0, y=2.0, vx=0.0, vy=0.0, radius=0.5)
    dt = 0.25
    current_time = 0.0
    while current_time < 3.0:
        if abs(current_time - 1.0) < 1e-9:
            sphere.vx, sphere.vy = choose_velocity(current_time)
        previous_x = sphere.x
        sphere.x += sphere.vx * dt
        sphere.y += sphere.vy * dt
        current_time = round(current_time + dt, 10)
        enforce_gate(sphere, previous_x, current_time)
    return sphere


if __name__ == "__main__":
    final = simulate()
    print({"position": (final.x, final.y), "velocity": (final.vx, final.vy)})
