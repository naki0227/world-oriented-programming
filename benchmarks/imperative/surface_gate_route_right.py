from dataclasses import dataclass


@dataclass
class Sphere:
    x: float
    y: float
    vx: float
    vy: float
    radius: float


LEFT_WALL_X = -4.0
RIGHT_WALL_X = 4.0
LEFT_GATE = (1.0, 3.0)
RIGHT_GATE = (1.0, 3.0)


def gate_open(name: str, current_time: float) -> bool:
    if name == "left":
        return current_time >= 4.0
    if name == "right":
        return current_time >= 1.0
    return False


def choose_velocity(current_time: float):
    if gate_open("left", current_time):
        return -3.0, 0.0
    if gate_open("right", current_time):
        return 3.0, 0.0
    return 0.0, 0.0


def point_in_gate(y: float, gate_bounds):
    return gate_bounds[0] <= y <= gate_bounds[1]


def enforce_gate(sphere: Sphere, previous_x: float, current_time: float):
    crossed_left = previous_x - sphere.radius >= LEFT_WALL_X and sphere.x - sphere.radius < LEFT_WALL_X
    crossed_right = previous_x + sphere.radius <= RIGHT_WALL_X and sphere.x + sphere.radius > RIGHT_WALL_X

    if crossed_left:
        if not gate_open("left", current_time) or not point_in_gate(sphere.y, LEFT_GATE):
            sphere.x = LEFT_WALL_X + sphere.radius
            if sphere.vx < 0.0:
                sphere.vx = 0.0

    if crossed_right:
        if not gate_open("right", current_time) or not point_in_gate(sphere.y, RIGHT_GATE):
            sphere.x = RIGHT_WALL_X - sphere.radius
            if sphere.vx > 0.0:
                sphere.vx = 0.0


def simulate():
    sphere = Sphere(x=0.0, y=2.0, vx=0.0, vy=0.0, radius=0.5)
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
