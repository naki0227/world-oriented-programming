#!/usr/bin/env python3

from __future__ import annotations

import argparse
import html
import json
import math
import subprocess
import textwrap
from pathlib import Path

try:
    from PIL import Image, ImageDraw
except ModuleNotFoundError:
    Image = None
    ImageDraw = None


PALETTE = [
    (30, 64, 175),
    (200, 54, 54),
    (27, 133, 95),
    (193, 128, 28),
    (111, 66, 193),
]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Render a static research figure from a sekai scenario."
    )
    parser.add_argument("scene", help="Path to a .sk scene file")
    parser.add_argument(
        "--output",
        help="Output PNG path. Defaults to figures/<scene-name>-<plane>.png",
    )
    parser.add_argument(
        "--caption-output",
        help="Optional markdown caption path. Defaults to figures/<scene-name>-caption.md",
    )
    parser.add_argument(
        "--plane",
        choices=["xy", "xz"],
        default="xy",
        help="Projection plane for the figure",
    )
    parser.add_argument(
        "--no-caption-panel",
        action="store_true",
        help="Render the figure without the embedded caption panel",
    )
    args = parser.parse_args()

    scene_path = Path(args.scene)
    default_suffix = "png" if Image is not None else "svg"
    output_path = Path(args.output) if args.output else Path("figures") / f"{scene_path.stem}-{args.plane}.{default_suffix}"
    if Image is None and output_path.suffix.lower() != ".svg":
        output_path = output_path.with_suffix(".svg")
    caption_path = (
        Path(args.caption_output)
        if args.caption_output
        else Path("figures") / f"{scene_path.stem}-caption.md"
    )

    output_path.parent.mkdir(parents=True, exist_ok=True)
    caption_path.parent.mkdir(parents=True, exist_ok=True)

    report = load_report(scene_path)
    caption = build_caption(report, scene_path.stem, scene_path.name)
    if output_path.suffix.lower() == ".svg" or Image is None:
        figure = render_report_svg(
            report,
            scene_path.name,
            args.plane,
            caption if not args.no_caption_panel else None,
        )
        output_path.write_text(figure, encoding="utf-8")
    else:
        image = render_report(
            report,
            scene_path.name,
            args.plane,
            caption if not args.no_caption_panel else None,
        )
        image.save(output_path)
    caption_path.write_text(caption + "\n", encoding="utf-8")

    print(output_path)
    print(caption_path)
    return 0


def load_report(scene_path: Path) -> dict:
    cmd = [
        "cargo",
        "run",
        "-p",
        "sekai-cli",
        "--",
        "simulate-report",
        str(scene_path),
    ]
    result = subprocess.run(
        cmd,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def render_report(report: dict, title: str, plane: str, caption: str | None) -> Image.Image:
    snapshots = report["snapshots"]
    timeline = report.get("observation_timeline", [])
    panel_count = max(1, len(snapshots))
    panel_width = 360
    panel_height = 360
    header = 70
    gutter = 24
    footer = 36
    legend_width = 180
    timeline_height = 84 if timeline else 0
    caption_height = 130 if caption else 0
    width = panel_count * panel_width + (panel_count + 1) * gutter + legend_width
    height = header + panel_height + footer + gutter + timeline_height + caption_height

    image = Image.new("RGB", (width, height), (248, 246, 240))
    draw = ImageDraw.Draw(image)

    draw.rectangle((0, 0, width, header), fill=(24, 33, 56))
    draw.text((24, 18), f"sekai figure: {title}", fill=(245, 244, 238))
    draw.text(
        (24, 42),
        f"projection={plane}  snapshots={len(snapshots)}",
        fill=(198, 207, 225),
    )

    world_bounds = compute_world_bounds(snapshots, plane)
    legend_x = panel_count * panel_width + (panel_count + 1) * gutter + 18
    draw.text((legend_x, header + 4), "Legend", fill=(24, 33, 56))

    names = sorted(
        {
            sphere["name"]
            for snapshot in snapshots
            for sphere in snapshot["spheres"]
        }
    )
    color_map = {name: PALETTE[index % len(PALETTE)] for index, name in enumerate(names)}

    for index, name in enumerate(names):
        y = header + 30 + index * 24
        color = color_map[name]
        draw.ellipse((legend_x, y, legend_x + 12, y + 12), fill=color)
        draw.text((legend_x + 20, y - 2), name, fill=(40, 40, 40))

    for panel_index, snapshot in enumerate(snapshots):
        x0 = gutter + panel_index * (panel_width + gutter)
        y0 = header
        x1 = x0 + panel_width
        y1 = y0 + panel_height
        draw.rounded_rectangle(
            (x0, y0, x1, y1),
            radius=16,
            fill=(255, 255, 255),
            outline=(222, 219, 212),
        )

        draw.text((x0 + 16, y0 + 12), f"t = {snapshot['time']:.3f}", fill=(24, 33, 56))
        checkpoint = checkpoint_for_time(timeline, snapshot["time"])
        if checkpoint:
            draw.text(
                (x0 + 16, y0 + 30),
                format_checkpoint_label(checkpoint),
                fill=status_color(checkpoint.get("status", "determinate")),
            )

        plot_margin = 34
        plot_box = (x0 + plot_margin, y0 + 58, x1 - plot_margin, y1 - plot_margin)
        draw.rectangle(plot_box, outline=(220, 225, 232))
        draw_axis_guides(draw, plot_box)

        for sphere in snapshot["spheres"]:
            color = color_map[sphere["name"]]
            px, py = project_point(sphere["position"], world_bounds, plot_box, plane)
            vx, vy = project_vector(sphere["velocity"], world_bounds, plot_box, plane)
            radius = 9
            draw.ellipse((px - radius, py - radius, px + radius, py + radius), fill=color)
            arrow_end = (px + vx * 0.22, py + vy * 0.22)
            draw.line((px, py, arrow_end[0], arrow_end[1]), fill=color, width=3)
            draw_arrow_head(draw, (px, py), arrow_end, color)
            draw.text((px + 10, py - 18), sphere["name"], fill=color)

        draw.text((x0 + 16, y1 - 20), axis_label(plane), fill=(108, 110, 118))

    if timeline:
        draw_observation_timeline(
            draw,
            timeline,
            gutter,
            header + panel_height + footer,
            panel_count * panel_width + (panel_count - 1) * gutter,
            timeline_height,
        )

    if caption:
        draw_caption_panel(draw, width, height, caption_height, caption)

    return image


def render_report_svg(report: dict, title: str, plane: str, caption: str | None) -> str:
    snapshots = report["snapshots"]
    timeline = report.get("observation_timeline", [])
    panel_count = max(1, len(snapshots))
    panel_width = 360
    panel_height = 360
    header = 70
    gutter = 24
    footer = 36
    legend_width = 180
    timeline_height = 84 if timeline else 0
    caption_height = 150 if caption else 0
    width = panel_count * panel_width + (panel_count + 1) * gutter + legend_width
    height = header + panel_height + footer + gutter + timeline_height + caption_height
    world_bounds = compute_world_bounds(snapshots, plane)
    names = sorted({sphere["name"] for snapshot in snapshots for sphere in snapshot["spheres"]})
    color_map = {name: rgb_to_hex(PALETTE[index % len(PALETTE)]) for index, name in enumerate(names)}
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="#f8f6f0"/>',
        f'<rect x="0" y="0" width="{width}" height="{header}" fill="#182138"/>',
        svg_text(24, 28, f"sekai figure: {title}", fill="#f5f4ee", size=16, weight="700"),
        svg_text(24, 52, f"projection={plane}  snapshots={len(snapshots)}", fill="#c6cfe1", size=13),
    ]

    legend_x = panel_count * panel_width + (panel_count + 1) * gutter + 18
    parts.append(svg_text(legend_x, header + 18, "Legend", fill="#182138", size=14, weight="700"))
    for index, name in enumerate(names):
        y = header + 30 + index * 24
        color = color_map[name]
        parts.append(f'<circle cx="{legend_x + 6}" cy="{y + 6}" r="6" fill="{color}"/>')
        parts.append(svg_text(legend_x + 20, y + 10, name, fill="#282828", size=13))

    for panel_index, snapshot in enumerate(snapshots):
        x0 = gutter + panel_index * (panel_width + gutter)
        y0 = header
        x1 = x0 + panel_width
        y1 = y0 + panel_height
        parts.append(
            f'<rect x="{x0}" y="{y0}" width="{panel_width}" height="{panel_height}" rx="16" fill="#ffffff" stroke="#dedbd4"/>'
        )
        parts.append(svg_text(x0 + 16, y0 + 25, f"t = {snapshot['time']:.3f}", fill="#182138", size=14, weight="700"))
        checkpoint = checkpoint_for_time(timeline, snapshot["time"])
        if checkpoint:
            parts.append(
                svg_text(
                    x0 + 16,
                    y0 + 43,
                    format_checkpoint_label(checkpoint),
                    fill=rgb_to_hex(status_color(checkpoint.get("status", "determinate"))),
                    size=12,
                )
            )

        plot_margin = 34
        plot_box = (x0 + plot_margin, y0 + 58, x1 - plot_margin, y1 - plot_margin)
        left, top, right, bottom = plot_box
        mid_x = (left + right) / 2
        mid_y = (top + bottom) / 2
        parts.append(f'<rect x="{left}" y="{top}" width="{right-left}" height="{bottom-top}" fill="none" stroke="#dce1e8"/>')
        parts.append(f'<line x1="{left}" y1="{mid_y}" x2="{right}" y2="{mid_y}" stroke="#e8ebf0"/>')
        parts.append(f'<line x1="{mid_x}" y1="{top}" x2="{mid_x}" y2="{bottom}" stroke="#e8ebf0"/>')

        for sphere in snapshot["spheres"]:
            color = color_map[sphere["name"]]
            px, py = project_point(sphere["position"], world_bounds, plot_box, plane)
            vx, vy = project_vector(sphere["velocity"], world_bounds, plot_box, plane)
            end_x = px + vx * 0.22
            end_y = py + vy * 0.22
            parts.append(f'<circle cx="{px:.2f}" cy="{py:.2f}" r="9" fill="{color}"/>')
            parts.append(f'<line x1="{px:.2f}" y1="{py:.2f}" x2="{end_x:.2f}" y2="{end_y:.2f}" stroke="{color}" stroke-width="3"/>')
            parts.append(svg_arrow_head(px, py, end_x, end_y, color))
            parts.append(svg_text(px + 10, py - 12, sphere["name"], fill=color, size=13, weight="700"))

        parts.append(svg_text(x0 + 16, y1 - 16, axis_label(plane), fill="#6c6e76", size=12))

    if timeline:
        parts.extend(svg_observation_timeline(timeline, gutter, header + panel_height + footer, panel_count * panel_width + (panel_count - 1) * gutter))

    if caption:
        caption_top = height - caption_height
        parts.append(f'<rect x="0" y="{caption_top}" width="{width}" height="{caption_height}" fill="#fffdf8"/>')
        parts.append(f'<line x1="0" y1="{caption_top}" x2="{width}" y2="{caption_top}" stroke="#d8d3c9" stroke-width="2"/>')
        for index, line in enumerate(wrap_caption(caption, 150).splitlines()):
            parts.append(svg_text(24, caption_top + 24 + index * 18, line, fill="#2c2c2c", size=13))

    parts.append("</svg>")
    return "\n".join(parts)


def svg_observation_timeline(timeline: list[dict], left: int, top: int, width: int) -> list[str]:
    right = left + width
    line_y = top + 28
    parts = [
        svg_text(left, top + 2, "observation frontiers", fill="#182138", size=14, weight="700"),
        f'<line x1="{left + 12}" y1="{line_y}" x2="{right - 12}" y2="{line_y}" stroke="#a0a6b0" stroke-width="2"/>',
    ]
    count = max(1, len(timeline) - 1)
    for index, checkpoint in enumerate(timeline):
        x = left + 12 + ((right - left - 24) * index / count if count else 0)
        label_x = min(max(left, x - 46), right - 96)
        color = rgb_to_hex(status_color(checkpoint.get("status", "determinate")))
        parts.append(f'<circle cx="{x:.2f}" cy="{line_y}" r="8" fill="{color}"/>')
        parts.append(svg_text(x - 18, line_y + 26, f"t={float(checkpoint.get('time', 0.0)):.0f}", fill="#363a44", size=12))
        parts.append(svg_text(label_x, line_y + 44, checkpoint.get("status", "determinate"), fill=color, size=12, weight="700"))
        parts.append(svg_text(label_x, line_y + 62, f"ambiguous={checkpoint.get('ambiguous_entities', 0)}", fill="#525660", size=12))
    return parts


def svg_arrow_head(start_x: float, start_y: float, end_x: float, end_y: float, color: str) -> str:
    dx = end_x - start_x
    dy = end_y - start_y
    length = math.hypot(dx, dy)
    if length < 1e-6:
        return ""
    ux = dx / length
    uy = dy / length
    size = 8
    left = (end_x - ux * size - uy * size * 0.6, end_y - uy * size + ux * size * 0.6)
    right = (end_x - ux * size + uy * size * 0.6, end_y - uy * size - ux * size * 0.6)
    return (
        f'<polygon points="{end_x:.2f},{end_y:.2f} {left[0]:.2f},{left[1]:.2f} '
        f'{right[0]:.2f},{right[1]:.2f}" fill="{color}"/>'
    )


def svg_text(x: float, y: float, text: str, *, fill: str, size: int, weight: str = "400") -> str:
    return (
        f'<text x="{x:.2f}" y="{y:.2f}" fill="{fill}" font-family="Georgia, Times New Roman, serif" '
        f'font-size="{size}" font-weight="{weight}">{html.escape(str(text))}</text>'
    )


def rgb_to_hex(color: tuple[int, int, int]) -> str:
    return f"#{color[0]:02x}{color[1]:02x}{color[2]:02x}"


def checkpoint_for_time(timeline: list[dict], time: float) -> dict | None:
    for checkpoint in timeline:
        if abs(float(checkpoint.get("time", 0.0)) - float(time)) < 1e-6:
            return checkpoint
    return None


def format_checkpoint_label(checkpoint: dict) -> str:
    status = checkpoint.get("status", "determinate")
    ambiguous = checkpoint.get("ambiguous_entities", 0)
    representative = checkpoint.get("representative_entities", 0)
    return f"{status}; ambiguous={ambiguous}; representative={representative}"


def status_color(status: str) -> tuple[int, int, int]:
    if status == "unresolved":
        return (138, 47, 47)
    if status == "representative":
        return (139, 104, 32)
    return (27, 111, 76)


def draw_observation_timeline(
    draw: ImageDraw.ImageDraw,
    timeline: list[dict],
    left: int,
    top: int,
    width: int,
    height: int,
) -> None:
    if not timeline:
        return

    right = left + width
    line_y = top + 28
    draw.text((left, top), "observation frontiers", fill=(24, 33, 56))
    draw.line((left + 12, line_y, right - 12, line_y), fill=(160, 166, 176), width=2)

    count = max(1, len(timeline) - 1)
    for index, checkpoint in enumerate(timeline):
        x = left + 12 + ((right - left - 24) * index / count if count else 0)
        label_x = min(max(left, x - 46), right - 96)
        color = status_color(checkpoint.get("status", "determinate"))
        draw.ellipse((x - 8, line_y - 8, x + 8, line_y + 8), fill=color)
        draw.text((x - 18, line_y + 14), f"t={float(checkpoint.get('time', 0.0)):.0f}", fill=(54, 58, 68))
        draw.text((label_x, line_y + 34), checkpoint.get("status", "determinate"), fill=color)
        draw.text(
            (label_x, line_y + 52),
            f"ambiguous={checkpoint.get('ambiguous_entities', 0)}",
            fill=(82, 86, 96),
        )


def draw_caption_panel(draw: ImageDraw.ImageDraw, width: int, height: int, caption_height: int, caption: str):
    top = height - caption_height
    draw.rectangle((0, top, width, height), fill=(255, 253, 248))
    draw.line((0, top, width, top), fill=(216, 211, 201), width=2)
    wrapped = wrap_caption(caption, 140)
    draw.text((24, top + 16), wrapped, fill=(44, 44, 44), spacing=6)


def compute_world_bounds(snapshots: list[dict], plane: str) -> tuple[float, float, float, float]:
    coords = []
    for snapshot in snapshots:
        for sphere in snapshot["spheres"]:
            coords.append(select_axes(sphere["position"], plane))
    xs = [coord[0] for coord in coords] or [0.0]
    ys = [coord[1] for coord in coords] or [0.0]
    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)
    span_x = max(max_x - min_x, 1.0)
    span_y = max(max_y - min_y, 1.0)
    pad_x = span_x * 0.18
    pad_y = span_y * 0.18
    return min_x - pad_x, max_x + pad_x, min_y - pad_y, max_y + pad_y


def select_axes(vector: dict, plane: str) -> tuple[float, float]:
    if plane == "xz":
        return vector["x"], vector["z"]
    return vector["x"], vector["y"]


def project_point(position: dict, bounds: tuple[float, float, float, float], box, plane: str):
    min_x, max_x, min_y, max_y = bounds
    x, y = select_axes(position, plane)
    left, top, right, bottom = box
    width = right - left
    height = bottom - top
    px = left + ((x - min_x) / max(max_x - min_x, 1e-9)) * width
    py = bottom - ((y - min_y) / max(max_y - min_y, 1e-9)) * height
    return px, py


def project_vector(velocity: dict, bounds: tuple[float, float, float, float], box, plane: str):
    min_x, max_x, min_y, max_y = bounds
    vx, vy = select_axes(velocity, plane)
    left, top, right, bottom = box
    width = right - left
    height = bottom - top
    sx = width / max(max_x - min_x, 1e-9)
    sy = height / max(max_y - min_y, 1e-9)
    return vx * sx, -vy * sy


def draw_axis_guides(draw: ImageDraw.ImageDraw, box):
    left, top, right, bottom = box
    mid_x = (left + right) / 2
    mid_y = (top + bottom) / 2
    guide = (232, 235, 240)
    draw.line((left, mid_y, right, mid_y), fill=guide, width=1)
    draw.line((mid_x, top, mid_x, bottom), fill=guide, width=1)


def draw_arrow_head(draw: ImageDraw.ImageDraw, start, end, color):
    dx = end[0] - start[0]
    dy = end[1] - start[1]
    length = math.hypot(dx, dy)
    if length < 1e-6:
        return
    ux = dx / length
    uy = dy / length
    size = 8
    left = (
        end[0] - ux * size - uy * size * 0.6,
        end[1] - uy * size + ux * size * 0.6,
    )
    right = (
        end[0] - ux * size + uy * size * 0.6,
        end[1] - uy * size - ux * size * 0.6,
    )
    draw.polygon([end, left, right], fill=color)


def axis_label(plane: str) -> str:
    return "axes: x-horizontal, z-vertical" if plane == "xz" else "axes: x-horizontal, y-vertical"


def build_caption(report: dict, scene_stem: str, scene_name: str) -> str:
    times = [snapshot["time"] for snapshot in report["snapshots"]]
    time_text = format_times(times)

    if scene_stem == "bounce":
        return (
            f"Figure: Declarative bouncing-sphere scenario in `sekai` ({scene_name}). "
            f"A single sphere evolves without any user-authored update loop. "
            f"The panels show snapshots at {time_text}; the downward trajectory is reflected at the floor, "
            f"after which the vertical velocity reverses while the horizontal component is preserved."
        )

    if scene_stem == "two_body_collision":
        return (
            f"Figure: Local synchronization in a two-body collision scenario ({scene_name}). "
            f"Two spheres advance independently until contact, at which point an explicit elastic-collision constraint is applied. "
            f"The panels at {time_text} show that interaction is localized to the collision event, after which the spheres separate with exchanged velocities."
        )

    if scene_stem == "forbidden_region":
        return (
            f"Figure: Forbidden-region constraint example ({scene_name}). "
            f"The world evolves until a sphere reaches a prohibited spatial region; this contradiction is reported as a world-law violation rather than handled through imperative exception code. "
            f"Requested observation times were {time_text}."
        )

    if scene_stem == "visibility_coordination_flagship":
        timeline = report.get("observation_timeline", [])
        timeline_text = format_timeline(timeline)
        return (
            f"Figure: Flagship visibility-coordination scenario in `sekai` ({scene_name}). "
            f"Two entities begin with ambiguous candidate actions while moving targets pass through a corridor-like visibility structure. "
            f"The panels show semantic observation frontiers at {time_text}; the observation timeline records {timeline_text}. "
            f"This makes staggered convergence visible as a property of the executable world report rather than as an external animation script."
        )

    return (
        f"Figure: Snapshot sequence generated from the `sekai` scene `{scene_name}`. "
        f"The panels show the world state at {time_text}, with object positions and velocity directions rendered from the executable world description."
    )


def format_times(times: list[float]) -> str:
    formatted = [f"t={time:.3f}" for time in times]
    if not formatted:
        return "unspecified observation times"
    if len(formatted) == 1:
        return formatted[0]
    if len(formatted) == 2:
        return f"{formatted[0]} and {formatted[1]}"
    return ", ".join(formatted[:-1]) + f", and {formatted[-1]}"


def format_timeline(timeline: list[dict]) -> str:
    if not timeline:
        return "no explicit observation-frontier metadata"
    entries = [
        f"t={float(checkpoint.get('time', 0.0)):.0f} {checkpoint.get('status', 'determinate')} "
        f"with ambiguous={checkpoint.get('ambiguous_entities', 0)}"
        for checkpoint in timeline
    ]
    if len(entries) == 1:
        return entries[0]
    if len(entries) == 2:
        return f"{entries[0]} and {entries[1]}"
    return ", ".join(entries[:-1]) + f", and {entries[-1]}"


def wrap_caption(caption: str, width: int) -> str:
    parts = []
    for paragraph in caption.splitlines():
        if not paragraph.strip():
            parts.append("")
        else:
            parts.append(textwrap.fill(paragraph, width=width))
    return "\n".join(parts)


if __name__ == "__main__":
    raise SystemExit(main())
