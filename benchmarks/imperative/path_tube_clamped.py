from dataclasses import dataclass
from math import sqrt


@dataclass
class Sphere:
    x: float
    y: float
    vx: float
    vy: float


def closest_point_on_segment(px, py, ax, ay, bx, by):
    sx = bx - ax
    sy = by - ay
    length_sq = sx * sx + sy * sy
    if length_sq == 0:
        dx = px - ax
        dy = py - ay
        return ax, ay, sqrt(dx * dx + dy * dy)
    t = ((px - ax) * sx + (py - ay) * sy) / length_sq
    t = max(0.0, min(1.0, t))
    cx = ax + sx * t
    cy = ay + sy * t
    dx = px - cx
    dy = py - cy
    return cx, cy, sqrt(dx * dx + dy * dy)


def clamp_inside_tube(sphere, ax, ay, bx, by, width):
    cx, cy, distance = closest_point_on_segment(sphere.x, sphere.y, ax, ay, bx, by)
    if distance <= width:
        return
    if distance == 0:
        nx, ny = 0.0, 1.0
    else:
        nx = (sphere.x - cx) / distance
        ny = (sphere.y - cy) / distance
    sphere.x = cx + nx * width
    sphere.y = cy + ny * width
    sx = bx - ax
    sy = by - ay
    length_sq = sx * sx + sy * sy
    along = (sphere.vx * sx + sphere.vy * sy) / length_sq
    sphere.vx = sx * along
    sphere.vy = sy * along


def simulate():
    sphere = Sphere(x=0.0, y=2.0, vx=1.0, vy=-0.8)
    ax, ay = -2.0, 0.0
    bx, by = 6.0, 0.0
    width = 1.0

    snapshots = []
    for _ in range(4):
        clamp_inside_tube(sphere, ax, ay, bx, by, width)
        snapshots.append((round(sphere.x, 3), round(sphere.y, 3)))
        sphere.x += sphere.vx
        sphere.y += sphere.vy
    return snapshots


if __name__ == "__main__":
    print(simulate())
