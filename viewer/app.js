const COLORS = ["#2440af", "#c83636", "#1b855f", "#c1801c", "#6f42c1"];

const state = {
  report: null,
  snapshotIndex: 0,
  viewMode: "3d",
  playing: false,
  colorMap: new Map(),
  frameHandle: null,
  camera: {
    yaw: -35,
    pitch: 22,
    zoom: 110,
  },
  editor: {
    enabled: false,
    spheres: [],
    selectedId: null,
    floorEnabled: true,
    floorOffset: 0,
    regionEnabled: false,
    region: {
      minX: -1,
      minY: 1,
      maxX: 1,
      maxY: 3,
    },
    nextIndex: 1,
    acceptedConstraints: new Set(),
  },
};

const canvas = document.getElementById("world-canvas");
const context = canvas.getContext("2d");
const fileInput = document.getElementById("file-input");
const playToggle = document.getElementById("play-toggle");
const resetView = document.getElementById("reset-view");
const editToggle = document.getElementById("edit-toggle");
const runDraftButton = document.getElementById("run-draft");
const viewModeSelect = document.getElementById("view-mode");
const sampleSelect = document.getElementById("sample-select");
const timeSlider = document.getElementById("time-slider");
const timeLabel = document.getElementById("time-label");
const sourceLabel = document.getElementById("source-label");
const snapshotCount = document.getElementById("snapshot-count");
const projectionLabel = document.getElementById("projection-label");
const observationStatus = document.getElementById("observation-status");
const snapshotList = document.getElementById("snapshot-list");
const constraintList = document.getElementById("constraint-list");
const analyticsList = document.getElementById("analytics-list");
const candidateResolutionList = document.getElementById("candidate-resolution-list");
const candidateComparisonList = document.getElementById("candidate-comparison-list");
const activityList = document.getElementById("activity-list");
const comparisonList = document.getElementById("comparison-list");
const yawSlider = document.getElementById("yaw-slider");
const pitchSlider = document.getElementById("pitch-slider");
const zoomSlider = document.getElementById("zoom-slider");
const yawLabel = document.getElementById("yaw-label");
const pitchLabel = document.getElementById("pitch-label");
const zoomLabel = document.getElementById("zoom-label");
const draftSphereSelect = document.getElementById("draft-sphere-select");
const draftName = document.getElementById("draft-name");
const draftRadius = document.getElementById("draft-radius");
const draftVx = document.getElementById("draft-vx");
const draftVy = document.getElementById("draft-vy");
const draftVz = document.getElementById("draft-vz");
const draftFloorEnabled = document.getElementById("draft-floor-enabled");
const draftFloorOffset = document.getElementById("draft-floor-offset");
const draftRegionEnabled = document.getElementById("draft-region-enabled");
const draftRegionMinX = document.getElementById("draft-region-min-x");
const draftRegionMinY = document.getElementById("draft-region-min-y");
const draftRegionMaxX = document.getElementById("draft-region-max-x");
const draftRegionMaxY = document.getElementById("draft-region-max-y");
const removeSphereButton = document.getElementById("remove-sphere");
const clearDraftButton = document.getElementById("clear-draft");
const draftOutput = document.getElementById("draft-output");
const draftHint = document.getElementById("draft-hint");
const constraintCandidates = document.getElementById("constraint-candidates");
const draftStatus = document.getElementById("draft-status");

const POLICY_COMPARISON_SAMPLES = [
  { label: "reject", path: "./samples/forbidden_region.json" },
  { label: "clamp", path: "./samples/clamped_region.json" },
  { label: "reflect", path: "./samples/reflected_region.json" },
];

const CANDIDATE_COMPARISON_SAMPLES = [
  { label: "fallback", path: "./samples/candidate_velocity.json" },
  { label: "repaired", path: "./samples/candidate_velocity_clamped.json" },
  { label: "deferred", path: "./samples/candidate_velocity_deferred.json" },
  { label: "tie", path: "./samples/candidate_velocity_tied.json" },
  { label: "equivalent tie", path: "./samples/candidate_velocity_equivalent_tie.json" },
];

fileInput.addEventListener("change", async (event) => {
  const file = event.target.files?.[0];
  if (!file) return;
  const text = await file.text();
  loadReport(JSON.parse(text), file.name);
});

sampleSelect.addEventListener("change", async (event) => {
  const path = event.target.value;
  if (!path) return;
  await loadSample(path);
});

async function loadSample(path) {
  const response = await fetch(path);
  const report = await response.json();
  loadReport(report, path);
}

viewModeSelect.addEventListener("change", (event) => {
  state.viewMode = event.target.value;
  syncViewLabels();
  render();
});

timeSlider.addEventListener("input", (event) => {
  state.snapshotIndex = Number(event.target.value);
  stopPlayback();
  render();
});

playToggle.addEventListener("click", () => {
  if (!state.report || state.report.status !== "ok") return;
  state.playing = !state.playing;
  playToggle.textContent = state.playing ? "Pause" : "Play";
  if (state.playing) {
    schedulePlayback();
  } else if (state.frameHandle !== null) {
    window.clearTimeout(state.frameHandle);
  }
});

resetView.addEventListener("click", () => {
  state.camera = { yaw: -35, pitch: 22, zoom: 110 };
  yawSlider.value = String(state.camera.yaw);
  pitchSlider.value = String(state.camera.pitch);
  zoomSlider.value = String(state.camera.zoom);
  syncCameraLabels();
  render();
});

editToggle.addEventListener("click", () => {
  state.editor.enabled = !state.editor.enabled;
  editToggle.textContent = state.editor.enabled ? "Editing Draft" : "Edit Draft";
  render();
});

runDraftButton.addEventListener("click", async () => {
  if (state.editor.spheres.length === 0) {
    setDraftStatus("Place at least one sphere before running the draft.", "error");
    return;
  }

  runDraftButton.disabled = true;
  setDraftStatus("Running generated draft through the local sekai runtime...", "busy");

  try {
    const response = await fetch("/api/simulate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: "viewer-draft.sk",
        source: generateDraftDSL(),
      }),
    });
    const report = await response.json();
    loadReport(report, report.source || "viewer-draft.sk");
    if (report.status === "ok") {
      setDraftStatus("Round-trip complete. The viewer is now showing the executed draft result.", "ok");
    } else {
      setDraftStatus(`Draft executed and returned a contradiction: ${report.error || "unknown error"}`, "error");
    }
  } catch (error) {
    setDraftStatus(
      "Run Draft needs the local viewer server. Start `python3 scripts/viewer_server.py` and reload the page.",
      "error",
    );
  } finally {
    runDraftButton.disabled = false;
  }
});

yawSlider.addEventListener("input", (event) => {
  state.camera.yaw = Number(event.target.value);
  syncCameraLabels();
  render();
});

pitchSlider.addEventListener("input", (event) => {
  state.camera.pitch = Number(event.target.value);
  syncCameraLabels();
  render();
});

zoomSlider.addEventListener("input", (event) => {
  state.camera.zoom = Number(event.target.value);
  syncCameraLabels();
  render();
});

canvas.addEventListener("click", (event) => {
  if (!state.editor.enabled || state.viewMode !== "xy") return;

  const world = canvasToWorldXY(event);
  const hit = findDraftSphereAt(world);
  if (hit) {
    state.editor.selectedId = hit.id;
    syncDraftControls();
    render();
    return;
  }

  const sphere = {
    id: crypto.randomUUID ? crypto.randomUUID() : `draft-${Date.now()}-${Math.random()}`,
    name: `S${state.editor.nextIndex}`,
    position: { x: world.x, y: world.y, z: 0 },
    velocity: { x: 0, y: 0, z: 0 },
    radius: 1,
  };
  state.editor.nextIndex += 1;
  state.editor.spheres.push(sphere);
  state.editor.selectedId = sphere.id;
  syncDraftControls();
  render();
});

draftSphereSelect.addEventListener("change", (event) => {
  state.editor.selectedId = event.target.value || null;
  syncDraftControls();
  render();
});

draftName.addEventListener("input", () => updateSelectedSphere((sphere) => {
  sphere.name = draftName.value || sphere.name;
}));
draftRadius.addEventListener("input", () => updateSelectedSphere((sphere) => {
  sphere.radius = Number(draftRadius.value) || 1;
}));
draftVx.addEventListener("input", () => updateSelectedSphere((sphere) => {
  sphere.velocity.x = Number(draftVx.value) || 0;
}));
draftVy.addEventListener("input", () => updateSelectedSphere((sphere) => {
  sphere.velocity.y = Number(draftVy.value) || 0;
}));
draftVz.addEventListener("input", () => updateSelectedSphere((sphere) => {
  sphere.velocity.z = Number(draftVz.value) || 0;
}));

draftFloorEnabled.addEventListener("change", () => {
  state.editor.floorEnabled = draftFloorEnabled.checked;
  syncDraftOutput();
  render();
});

draftFloorOffset.addEventListener("input", () => {
  state.editor.floorOffset = Number(draftFloorOffset.value) || 0;
  syncDraftOutput();
  render();
});

draftRegionEnabled.addEventListener("change", () => {
  state.editor.regionEnabled = draftRegionEnabled.checked;
  syncDraftOutput();
  render();
});

draftRegionMinX.addEventListener("input", () => updateRegionValue("minX", draftRegionMinX.value));
draftRegionMinY.addEventListener("input", () => updateRegionValue("minY", draftRegionMinY.value));
draftRegionMaxX.addEventListener("input", () => updateRegionValue("maxX", draftRegionMaxX.value));
draftRegionMaxY.addEventListener("input", () => updateRegionValue("maxY", draftRegionMaxY.value));

removeSphereButton.addEventListener("click", () => {
  if (!state.editor.selectedId) return;
  state.editor.spheres = state.editor.spheres.filter((sphere) => sphere.id !== state.editor.selectedId);
  state.editor.selectedId = state.editor.spheres[0]?.id || null;
  syncDraftControls();
  render();
});

clearDraftButton.addEventListener("click", () => {
  state.editor.spheres = [];
  state.editor.selectedId = null;
  state.editor.acceptedConstraints = new Set();
  syncDraftControls();
  render();
});

function loadReport(report, sourceName) {
  state.report = normalizeReport(report);
  state.snapshotIndex = 0;
  stopPlayback();
  sourceLabel.textContent = sourceName || state.report.source || "Unknown source";
  snapshotCount.textContent = String(state.report.snapshots.length);
  observationStatus.textContent =
    state.report.observation_summary?.status || "determinate";
  timeSlider.max = String(Math.max(0, state.report.snapshots.length - 1));
  timeSlider.value = "0";
  state.colorMap = buildColorMap(state.report);
  syncViewLabels();
  syncCameraLabels();
  syncDraftControls();
  render();
}

function stopPlayback() {
  state.playing = false;
  playToggle.textContent = "Play";
  if (state.frameHandle !== null) {
    window.clearTimeout(state.frameHandle);
    state.frameHandle = null;
  }
}

function normalizeReport(report) {
  if (report.status) {
    return {
      source: report.source || "unknown",
      status: report.status,
      error: report.error || null,
      analytics: report.analytics || defaultAnalytics(report.constraints || []),
      observation_summary:
        report.observation_summary ||
        defaultObservationSummary(report.candidate_resolutions || []),
      constraints: report.constraints || [],
      candidate_inventory: report.candidate_inventory || [],
      action_directive_inventory: report.action_directive_inventory || [],
      candidate_resolutions: report.candidate_resolutions || [],
      activities: report.activities || [],
      snapshots: report.snapshots || [],
    };
  }
  return {
    source: report.source || "unknown",
    status: "ok",
    error: null,
    analytics: report.analytics || defaultAnalytics(report.constraints || []),
    observation_summary:
      report.observation_summary ||
      defaultObservationSummary(report.candidate_resolutions || []),
    constraints: report.constraints || [],
    candidate_inventory: report.candidate_inventory || [],
    action_directive_inventory: report.action_directive_inventory || [],
    candidate_resolutions: report.candidate_resolutions || [],
    activities: report.activities || [],
    snapshots: report.snapshots || [],
  };
}

function defaultObservationSummary(candidateResolutions) {
  const representativeEntities = candidateResolutions.filter(
    (resolution) => resolution.observation_mode === "representative"
  ).length;
  const ambiguousEntities = candidateResolutions.filter(
    (resolution) => resolution.observation_mode === "ambiguous"
  ).length;
  return {
    status:
      ambiguousEntities > 0
        ? "unresolved"
        : representativeEntities > 0
          ? "representative"
          : "determinate",
    representative_entities: representativeEntities,
    ambiguous_entities: ambiguousEntities,
  };
}

function defaultAnalytics(constraints) {
  const analytics = {
    total_constraints: 0,
    invariant_constraints: 0,
    boundary_constraints: 0,
    interaction_constraints: 0,
    idle_constraints: 0,
    fired_constraints: 0,
    repaired_constraints: 0,
    contradicted_constraints: 0,
  };

  constraints.forEach((constraint) => {
    analytics.total_constraints += 1;
    const categoryKey = `${constraint.category || "unknown"}_constraints`;
    if (categoryKey in analytics) analytics[categoryKey] += 1;
    const outcomeKey = `${constraint.outcome || "idle"}_constraints`;
    if (outcomeKey in analytics) analytics[outcomeKey] += 1;
  });

  return analytics;
}

function buildColorMap(report) {
  const names = Array.from(
    new Set(report.snapshots.flatMap((snapshot) => snapshot.spheres.map((sphere) => sphere.name))),
  ).sort();
  const map = new Map();
  names.forEach((name, index) => map.set(name, COLORS[index % COLORS.length]));
  return map;
}

function hasStableSnapshots(report = state.report) {
  return Boolean(report && Array.isArray(report.snapshots) && report.snapshots.length > 0);
}

function activeSnapshot() {
  if (!hasStableSnapshots()) return null;
  const maxIndex = state.report.snapshots.length - 1;
  const index = Math.min(state.snapshotIndex, maxIndex);
  return state.report.snapshots[index];
}

function lastStableSnapshot() {
  if (!hasStableSnapshots()) return null;
  return state.report.snapshots[state.report.snapshots.length - 1];
}

function schedulePlayback() {
  if (!state.playing || !state.report || state.report.status !== "ok") return;
  state.frameHandle = window.setTimeout(() => {
    state.snapshotIndex = (state.snapshotIndex + 1) % state.report.snapshots.length;
    timeSlider.value = String(state.snapshotIndex);
    render();
    schedulePlayback();
  }, 950);
}

function syncViewLabels() {
  projectionLabel.textContent = state.viewMode.toUpperCase();
}

function syncCameraLabels() {
  yawLabel.textContent = `${state.camera.yaw}°`;
  pitchLabel.textContent = `${state.camera.pitch}°`;
  zoomLabel.textContent = `${(state.camera.zoom / 100).toFixed(2)}x`;
}

function render() {
  renderCanvas();
  renderSidebar();
  renderConstraintCandidates();
  syncDraftOutput();
}

function renderCanvas() {
  context.clearRect(0, 0, canvas.width, canvas.height);
  drawCanvasFrame();

  if (!state.report) {
    context.fillStyle = "#6d7280";
    context.font = '24px Georgia, "Times New Roman", serif';
    context.fillText("Load a JSON report or choose a sample.", 48, 92);
    return;
  }

  if (state.report.status !== "ok") {
    const snapshot = activeSnapshot();
    if (snapshot) {
      const bounds = compute3DBounds(state.report.snapshots, state.editor.spheres, state.editor.floorEnabled ? state.editor.floorOffset : null);
      const plot = { left: 78, top: 150, right: canvas.width - 78, bottom: canvas.height - 92 };
      if (state.viewMode === "3d") {
        draw3DScene(snapshot, bounds, plot);
      } else {
        draw2DScene(snapshot, bounds, plot, state.viewMode);
      }
      if (state.editor.spheres.length > 0) {
        drawDraftOverlay(bounds, plot);
      }

      context.fillStyle = "#8a2f2f";
      context.font = '28px Georgia, "Times New Roman", serif';
      context.fillText("World Contradiction", 48, 92);
      context.fillStyle = "#5e4747";
      context.font = '20px Georgia, "Times New Roman", serif';
      context.fillText(`Last stable snapshot at t = ${snapshot.time.toFixed(3)}`, 48, 126);
      context.font = '18px Georgia, "Times New Roman", serif';
      context.fillText(axisLabel(state.viewMode), 48, canvas.height - 38);

      context.fillStyle = "rgba(255, 247, 242, 0.90)";
      context.fillRect(40, 20, canvas.width - 80, 132);
      context.strokeStyle = "#d7b7ab";
      context.lineWidth = 1.5;
      context.strokeRect(40, 20, canvas.width - 80, 132);

      context.fillStyle = "#8a2f2f";
      context.font = '28px Georgia, "Times New Roman", serif';
      context.fillText("World Contradiction", 56, 60);
      context.fillStyle = "#5e4747";
      context.font = '20px Georgia, "Times New Roman", serif';
      context.fillText(state.report.error || "Unknown error", 56, 94);
      context.font = '18px Georgia, "Times New Roman", serif';
      context.fillText(`Showing last stable state before failure at t = ${snapshot.time.toFixed(3)}.`, 56, 124);
      return;
    }

    context.fillStyle = "#8a2f2f";
    context.font = '28px Georgia, "Times New Roman", serif';
    context.fillText("World Contradiction", 48, 92);
    context.fillStyle = "#5e4747";
    context.font = '22px Georgia, "Times New Roman", serif';
    context.fillText(state.report.error || "Unknown error", 48, 134);
    context.font = '18px Georgia, "Times New Roman", serif';
    context.fillText("This scenario terminated before producing a stable snapshot sequence.", 48, 172);
    return;
  }

  const snapshot = activeSnapshot();
  const bounds = compute3DBounds(state.report.snapshots, state.editor.spheres, state.editor.floorEnabled ? state.editor.floorOffset : null);
  const plot = { left: 78, top: 88, right: canvas.width - 78, bottom: canvas.height - 92 };

  if (state.viewMode === "3d") {
    draw3DScene(snapshot, bounds, plot);
  } else {
    draw2DScene(snapshot, bounds, plot, state.viewMode);
  }

  if (state.editor.spheres.length > 0) {
    drawDraftOverlay(bounds, plot);
  }

  context.fillStyle = "#24395e";
  context.font = '26px Georgia, "Times New Roman", serif';
  context.fillText(`t = ${snapshot.time.toFixed(3)}`, 48, 52);
  context.font = '18px Georgia, "Times New Roman", serif';
  context.fillStyle = "#6d7280";
  context.fillText(axisLabel(state.viewMode), 48, canvas.height - 38);
}

function drawCanvasFrame() {
  context.strokeStyle = "#d8d1c4";
  context.lineWidth = 2;
  roundRect(context, 16, 16, canvas.width - 32, canvas.height - 32, 24);
  context.stroke();
}

function draw2DScene(snapshot, bounds, plot, projection) {
  drawGrid(plot);
  drawAxes2D(plot);

  snapshot.spheres.forEach((sphere) => {
    const color = state.colorMap.get(sphere.name) || COLORS[0];
    const point = projectPoint2D(sphere.position, bounds, plot, projection);
    const velocity = projectVector2D(sphere.velocity, bounds, plot, projection);
    drawSphereGlyph(point, velocity, color, sphere.name);
  });
}

function drawDraftOverlay(bounds, plot) {
  if (state.viewMode === "3d") {
    drawDraft3D(bounds, plot);
    return;
  }
  if (state.viewMode !== "xy") return;

  if (state.editor.floorEnabled) {
    const a = projectPoint2D({ x: bounds.minX, y: state.editor.floorOffset, z: 0 }, bounds, plot, "xy");
    const b = projectPoint2D({ x: bounds.maxX, y: state.editor.floorOffset, z: 0 }, bounds, plot, "xy");
    context.strokeStyle = "#8d8c88";
    context.lineWidth = 3;
    context.beginPath();
    context.moveTo(a.x, a.y);
    context.lineTo(b.x, b.y);
    context.stroke();
  }

  if (state.editor.regionEnabled) {
    const region = normalizedRegion();
    const a = projectPoint2D({ x: region.minX, y: region.minY, z: 0 }, bounds, plot, "xy");
    const b = projectPoint2D({ x: region.maxX, y: region.maxY, z: 0 }, bounds, plot, "xy");
    context.save();
    context.fillStyle = "rgba(200, 54, 54, 0.10)";
    context.strokeStyle = "#c83636";
    context.lineWidth = 2.5;
    context.setLineDash([8, 6]);
    context.beginPath();
    context.rect(a.x, b.y, b.x - a.x, a.y - b.y);
    context.fill();
    context.stroke();
    context.restore();
  }

  state.editor.spheres.forEach((sphere) => {
    const selected = sphere.id === state.editor.selectedId;
    const point = projectPoint2D(sphere.position, bounds, plot, "xy");
    const velocity = projectVector2D(sphere.velocity, bounds, plot, "xy");
    context.save();
    context.setLineDash([7, 5]);
    drawSphereGlyph(point, velocity, selected ? "#111111" : "#aa5b21", sphere.name);
    context.restore();
  });
}

function drawDraft3D(bounds, plot) {
  if (state.editor.floorEnabled) {
    const floorLines = [];
    for (let i = 0; i <= 5; i += 1) {
      const t = i / 5;
      const x = lerp(bounds.minX, bounds.maxX, t);
      const z = lerp(bounds.minZ, bounds.maxZ, t);
      floorLines.push(make3DLine(
        { x, y: state.editor.floorOffset, z: bounds.minZ },
        { x, y: state.editor.floorOffset, z: bounds.maxZ },
        bounds,
        plot,
        "#cfb98c",
        1.5,
      ));
      floorLines.push(make3DLine(
        { x: bounds.minX, y: state.editor.floorOffset, z },
        { x: bounds.maxX, y: state.editor.floorOffset, z },
        bounds,
        plot,
        "#cfb98c",
        1.5,
      ));
    }
    floorLines.forEach((line) => {
      context.strokeStyle = line.color;
      context.lineWidth = line.width;
      context.beginPath();
      context.moveTo(line.a.x, line.a.y);
      context.lineTo(line.b.x, line.b.y);
      context.stroke();
    });
  }

  if (state.editor.regionEnabled) {
    const region = normalizedRegion();
    const yBase = bounds.minY;
    const yTop = Math.max(bounds.minY + 0.6, region.maxY);
    const corners = [
      { x: region.minX, y: yBase, z: 0 },
      { x: region.maxX, y: yBase, z: 0 },
      { x: region.maxX, y: yTop, z: 0 },
      { x: region.minX, y: yTop, z: 0 },
    ];
    [
      [corners[0], corners[1]],
      [corners[1], corners[2]],
      [corners[2], corners[3]],
      [corners[3], corners[0]],
    ]
      .map(([a, b]) => make3DLine(a, b, bounds, plot, "#c83636", 2))
      .forEach((line) => {
        context.strokeStyle = line.color;
        context.lineWidth = line.width;
        context.beginPath();
        context.moveTo(line.a.x, line.a.y);
        context.lineTo(line.b.x, line.b.y);
        context.stroke();
      });
  }

  state.editor.spheres
    .map((sphere) => {
      const point3D = projectPoint3D(sphere.position, bounds, plot);
      const velocityEnd = {
        x: sphere.position.x + sphere.velocity.x * 0.24,
        y: sphere.position.y + sphere.velocity.y * 0.24,
        z: sphere.position.z + sphere.velocity.z * 0.24,
      };
      const arrow3D = projectPoint3D(velocityEnd, bounds, plot);
      return {
        sphere,
        point: point3D.point,
        depth: point3D.depth,
        arrowEnd: arrow3D.point,
        arrowVector: {
          x: arrow3D.point.x - point3D.point.x,
          y: arrow3D.point.y - point3D.point.y,
        },
      };
    })
    .sort((a, b) => a.depth - b.depth)
    .forEach(({ sphere, point, arrowEnd, arrowVector }) => {
      const selected = sphere.id === state.editor.selectedId;
      context.save();
      context.setLineDash([7, 5]);
      context.fillStyle = selected ? "#111111" : "#aa5b21";
      context.beginPath();
      context.arc(point.x, point.y, selected ? 14 : 12, 0, Math.PI * 2);
      context.fill();
      context.strokeStyle = selected ? "#111111" : "#aa5b21";
      context.lineWidth = 4;
      context.beginPath();
      context.moveTo(point.x, point.y);
      context.lineTo(arrowEnd.x, arrowEnd.y);
      context.stroke();
      drawArrowHead2D(arrowEnd, arrowVector, selected ? "#111111" : "#aa5b21");
      context.restore();
      context.fillStyle = selected ? "#111111" : "#aa5b21";
      context.fillText(sphere.name, point.x + 15, point.y - 15);
    });
}

function draw3DScene(snapshot, bounds, plot) {
  const scene = build3DScene(snapshot, bounds, plot);

  scene.gridLines.forEach((line) => {
    context.strokeStyle = line.color;
    context.lineWidth = line.width;
    context.beginPath();
    context.moveTo(line.a.x, line.a.y);
    context.lineTo(line.b.x, line.b.y);
    context.stroke();
  });

  scene.axisLines.forEach((line) => {
    context.strokeStyle = line.color;
    context.lineWidth = line.width;
    context.beginPath();
    context.moveTo(line.a.x, line.a.y);
    context.lineTo(line.b.x, line.b.y);
    context.stroke();
    context.fillStyle = line.color;
    context.font = '16px Georgia, "Times New Roman", serif';
    context.fillText(line.label, line.b.x + 8, line.b.y - 4);
  });

  scene.spheres
    .sort((a, b) => a.depth - b.depth)
    .forEach((sphere) => {
      context.fillStyle = sphere.color;
      context.beginPath();
      context.arc(sphere.point.x, sphere.point.y, sphere.radius, 0, Math.PI * 2);
      context.fill();

      context.strokeStyle = sphere.color;
      context.lineWidth = 4;
      context.beginPath();
      context.moveTo(sphere.point.x, sphere.point.y);
      context.lineTo(sphere.arrowEnd.x, sphere.arrowEnd.y);
      context.stroke();
      drawArrowHead2D(sphere.arrowEnd, sphere.arrowVector, sphere.color);

      context.fillStyle = sphere.color;
      context.font = '18px Georgia, "Times New Roman", serif';
      context.fillText(sphere.name, sphere.point.x + 15, sphere.point.y - 16);
    });
}

function build3DScene(snapshot, bounds, plot) {
  const gridLines = [];
  const axisLines = [];
  const spheres = [];
  const gridSteps = 6;

  const min = { x: bounds.minX, y: bounds.minY, z: bounds.minZ };
  const max = { x: bounds.maxX, y: bounds.maxY, z: bounds.maxZ };

  for (let i = 0; i <= gridSteps; i += 1) {
    const t = i / gridSteps;
    const x = lerp(min.x, max.x, t);
    const z = lerp(min.z, max.z, t);
    gridLines.push(make3DLine({ x, y: min.y, z: min.z }, { x, y: min.y, z: max.z }, bounds, plot, "#e4e8ee", 1));
    gridLines.push(make3DLine({ x: min.x, y: min.y, z }, { x: max.x, y: min.y, z }, bounds, plot, "#e4e8ee", 1));
  }

  axisLines.push(make3DLine({ x: min.x, y: min.y, z: min.z }, { x: max.x, y: min.y, z: min.z }, bounds, plot, "#2440af", 2.5, "x"));
  axisLines.push(make3DLine({ x: min.x, y: min.y, z: min.z }, { x: min.x, y: max.y, z: min.z }, bounds, plot, "#1b855f", 2.5, "y"));
  axisLines.push(make3DLine({ x: min.x, y: min.y, z: min.z }, { x: min.x, y: min.y, z: max.z }, bounds, plot, "#c1801c", 2.5, "z"));

  snapshot.spheres.forEach((sphere) => {
    const color = state.colorMap.get(sphere.name) || COLORS[0];
    const point3D = projectPoint3D(sphere.position, bounds, plot);
    const velocityEnd = {
      x: sphere.position.x + sphere.velocity.x * 0.24,
      y: sphere.position.y + sphere.velocity.y * 0.24,
      z: sphere.position.z + sphere.velocity.z * 0.24,
    };
    const arrow3D = projectPoint3D(velocityEnd, bounds, plot);
    spheres.push({
      name: sphere.name,
      color,
      point: point3D.point,
      arrowEnd: arrow3D.point,
      arrowVector: {
        x: arrow3D.point.x - point3D.point.x,
        y: arrow3D.point.y - point3D.point.y,
      },
      radius: Math.max(8, 12 + point3D.depth * 2.4),
      depth: point3D.depth,
    });
  });

  return {
    gridLines,
    axisLines,
    spheres,
  };
}

function make3DLine(a, b, bounds, plot, color, width, label = "") {
  const pa = projectPoint3D(a, bounds, plot).point;
  const pb = projectPoint3D(b, bounds, plot).point;
  return { a: pa, b: pb, color, width, label };
}

function drawGrid(plot) {
  context.strokeStyle = "#e4e8ee";
  context.lineWidth = 1;
  for (let i = 0; i <= 6; i += 1) {
    const x = plot.left + ((plot.right - plot.left) / 6) * i;
    const y = plot.top + ((plot.bottom - plot.top) / 6) * i;
    context.beginPath();
    context.moveTo(x, plot.top);
    context.lineTo(x, plot.bottom);
    context.stroke();
    context.beginPath();
    context.moveTo(plot.left, y);
    context.lineTo(plot.right, y);
    context.stroke();
  }
}

function drawAxes2D(plot) {
  context.strokeStyle = "#b6bcc7";
  context.lineWidth = 2;
  context.beginPath();
  context.moveTo(plot.left, plot.bottom);
  context.lineTo(plot.right, plot.bottom);
  context.stroke();
  context.beginPath();
  context.moveTo(plot.left, plot.top);
  context.lineTo(plot.left, plot.bottom);
  context.stroke();
}

function drawSphereGlyph(point, velocity, color, name) {
  context.fillStyle = color;
  context.beginPath();
  context.arc(point.x, point.y, 13, 0, Math.PI * 2);
  context.fill();

  const end = { x: point.x + velocity.x * 0.24, y: point.y + velocity.y * 0.24 };
  context.strokeStyle = color;
  context.lineWidth = 4;
  context.beginPath();
  context.moveTo(point.x, point.y);
  context.lineTo(end.x, end.y);
  context.stroke();
  drawArrowHead2D(end, velocity, color);

  context.fillStyle = color;
  context.font = '18px Georgia, "Times New Roman", serif';
  context.fillText(name, point.x + 16, point.y - 16);
}

function renderSidebar() {
  if (!state.report) {
    timeLabel.textContent = "t = 0.000";
    snapshotList.innerHTML = '<p class="muted">Load a report to inspect world state.</p>';
    constraintList.innerHTML = '<p class="muted">Load a report to inspect active constraints.</p>';
    analyticsList.innerHTML = '<p class="muted">Load a report to inspect law totals and outcome distribution.</p>';
    activityList.innerHTML = '<p class="muted">Load a report to inspect fired or repaired laws.</p>';
    comparisonList.innerHTML = '<p class="muted">Load a forbidden-region report to compare reject, clamp, and reflect.</p>';
    return;
  }

  if (state.report.status !== "ok") {
    const stableSnapshot = activeSnapshot() || lastStableSnapshot();
    timeLabel.textContent = stableSnapshot ? `t = ${stableSnapshot.time.toFixed(3)} (last stable)` : "t = error";
    snapshotList.innerHTML = `
      <article class="sphere-card">
        <h3>Execution Status</h3>
        <p>status = ${state.report.status}</p>
        <p>error = ${state.report.error || "unknown error"}</p>
      </article>
    `;
    if (stableSnapshot) {
      stableSnapshot.spheres.forEach((sphere) => {
        const color = state.colorMap.get(sphere.name) || COLORS[0];
        const card = document.createElement("article");
        card.className = "sphere-card";
        card.innerHTML = `
          <h3><span class="swatch" style="background:${color}"></span>${sphere.name} (last stable)</h3>
          <p>position = (${sphere.position.x.toFixed(3)}, ${sphere.position.y.toFixed(3)}, ${sphere.position.z.toFixed(3)})</p>
          <p>velocity = (${sphere.velocity.x.toFixed(3)}, ${sphere.velocity.y.toFixed(3)}, ${sphere.velocity.z.toFixed(3)})</p>
        `;
        snapshotList.appendChild(card);
      });
    } else {
      const note = document.createElement("p");
      note.className = "muted";
      note.textContent = "No stable snapshot was produced before contradiction.";
      snapshotList.appendChild(note);
    }
    renderConstraintList();
    renderAnalyticsList();
    renderCandidateResolution();
    renderCandidateComparison();
    renderActivityList();
    renderComparisonList();
    return;
  }

  const snapshot = activeSnapshot();
  timeLabel.textContent = `t = ${snapshot.time.toFixed(3)}`;

  snapshotList.innerHTML = "";
  snapshot.spheres.forEach((sphere) => {
    const color = state.colorMap.get(sphere.name) || COLORS[0];
    const card = document.createElement("article");
    card.className = "sphere-card";
    card.innerHTML = `
      <h3><span class="swatch" style="background:${color}"></span>${sphere.name}</h3>
      <p>position = (${sphere.position.x.toFixed(3)}, ${sphere.position.y.toFixed(3)}, ${sphere.position.z.toFixed(3)})</p>
      <p>velocity = (${sphere.velocity.x.toFixed(3)}, ${sphere.velocity.y.toFixed(3)}, ${sphere.velocity.z.toFixed(3)})</p>
    `;
    snapshotList.appendChild(card);
  });

  renderConstraintList();
  renderAnalyticsList();
  renderCandidateResolution();
  renderCandidateComparison();
  renderActivityList();
  renderComparisonList();
}

function renderConstraintList() {
  if (!state.report) {
    constraintList.innerHTML = '<p class="muted">Load a report to inspect active constraints.</p>';
    return;
  }

  const constraints = state.report.constraints || [];
  if (constraints.length === 0) {
    constraintList.innerHTML = '<p class="muted">This report does not expose constraint metadata.</p>';
    return;
  }

  constraintList.innerHTML = "";
  constraints.forEach((constraint) => {
    const card = document.createElement("article");
    card.className = "sphere-card";

    const title = document.createElement("h3");
    title.textContent = constraint.kind || "constraint";

    const targets = document.createElement("p");
    targets.textContent = Array.isArray(constraint.targets) ? constraint.targets.join(", ") : "";

    const category = document.createElement("p");
    category.className = "muted";
    category.textContent = `category: ${constraint.category || "unknown"}`;

    const policy = document.createElement("p");
    policy.className = "muted";
    policy.textContent = `policy: ${constraint.policy || "implicit"}`;

    const outcome = document.createElement("p");
    outcome.className = "muted";
    outcome.textContent = `outcome: ${constraint.outcome || "idle"}`;

    const supported = document.createElement("p");
    supported.className = "muted";
    const supportedPolicies = Array.isArray(constraint.supported_policies)
      ? constraint.supported_policies.join(", ")
      : "";
    supported.textContent = `supports: ${supportedPolicies || "implicit-only"}`;

    const activity = document.createElement("p");
    activity.className = "muted";
    activity.textContent = `fired: ${constraint.fired_count ?? 0}, repaired: ${constraint.repaired_count ?? 0}`;

    card.appendChild(title);
    card.appendChild(targets);
    card.appendChild(category);
    card.appendChild(policy);
    card.appendChild(outcome);
    card.appendChild(supported);
    card.appendChild(activity);
    constraintList.appendChild(card);
  });
}

function renderActivityList() {
  if (!state.report) {
    activityList.innerHTML = '<p class="muted">Load a report to inspect fired or repaired laws.</p>';
    return;
  }

  const activities = state.report.activities || [];
  if (activities.length === 0) {
    activityList.innerHTML = '<p class="muted">No law activity was recorded for this report.</p>';
    return;
  }

  activityList.innerHTML = "";
  activities.forEach((activity) => {
    const card = document.createElement("article");
    card.className = "sphere-card";

    const title = document.createElement("h3");
    title.textContent = `${activity.action || "activity"} @ t=${Number(activity.time || 0).toFixed(3)}`;

    const kind = document.createElement("p");
    kind.textContent = `${activity.kind || "constraint"} (${activity.policy || "implicit"})`;

    const targets = document.createElement("p");
    targets.className = "muted";
    targets.textContent = Array.isArray(activity.targets) ? activity.targets.join(", ") : "";

    card.appendChild(title);
    card.appendChild(kind);
    card.appendChild(targets);
    activityList.appendChild(card);
  });
}

function renderCandidateResolution() {
  if (!state.report) {
    candidateResolutionList.innerHTML =
      '<p class="muted">Load a Phase I report to inspect candidate selection.</p>';
    return;
  }

  const candidateResolutions = state.report.candidate_resolutions || [];
  if (candidateResolutions.length === 0) {
    const candidateInventory = state.report.candidate_inventory || [];
    const actionDirectiveInventory = state.report.action_directive_inventory || [];
    if (candidateInventory.length === 0 && actionDirectiveInventory.length === 0) {
      candidateResolutionList.innerHTML =
        '<p class="muted">This report has no candidate-resolution metadata.</p>';
      return;
    }
    candidateResolutionList.innerHTML = "";
    const summaryCard = document.createElement("article");
    summaryCard.className = "sphere-card";
    summaryCard.innerHTML = `
      <h3>Static Phase I Inventory</h3>
      <p>candidate entities = ${candidateInventory.length}</p>
      <p class="muted">action directives = ${actionDirectiveInventory.length}</p>
      <p class="muted">execution = not run</p>
    `;
    candidateResolutionList.appendChild(summaryCard);
    candidateInventory.forEach((inventory) => {
      const directives = actionDirectiveInventory
        .filter((directive) => directive.entity === inventory.entity)
        .map((directive) => directive.kind);
      const card = document.createElement("article");
      card.className = "sphere-card";
      card.innerHTML = `
        <h3>${inventory.entity}</h3>
        <p>candidates = ${inventory.total_candidates}</p>
        <p class="muted">top score = ${inventory.top_score || "n/a"}</p>
        <p class="muted">top labels = ${(inventory.top_labels || []).join(", ") || "none"}</p>
        <p class="muted">top score tied = ${inventory.top_score_tied ? "yes" : "no"}</p>
        <p class="muted">defer on ambiguous top = ${inventory.defer_on_ambiguous_top ? "yes" : "no"}</p>
        <p class="muted">resolution hint = ${inventory.resolution_hint || "n/a"}</p>
        <p class="muted">directives = ${directives.join(", ") || "none"}</p>
      `;
      candidateResolutionList.appendChild(card);
    });
    return;
  }

  candidateResolutionList.innerHTML = "";
  const convergenceAnalytics = state.report.convergence_analytics || {};
  const summaryCard = document.createElement("article");
  summaryCard.className = "sphere-card";
  summaryCard.innerHTML = `
    <h3>Run Summary</h3>
    <p>candidate entities = ${convergenceAnalytics.candidate_entities ?? candidateResolutions.length}</p>
    <p class="muted">direct = ${convergenceAnalytics.direct_entities ?? 0}</p>
    <p class="muted">fallback = ${convergenceAnalytics.fallback_entities ?? 0}</p>
    <p class="muted">repaired = ${convergenceAnalytics.repaired_entities ?? 0}</p>
    <p class="muted">tie broken = ${convergenceAnalytics.tie_broken_entities ?? 0}</p>
    <p class="muted">equivalent tie = ${convergenceAnalytics.equivalent_tie_entities ?? 0}</p>
    <p class="muted">observation determinate = ${convergenceAnalytics.determinate_entities ?? 0}</p>
    <p class="muted">observation representative = ${convergenceAnalytics.representative_entities ?? 0}</p>
    <p class="muted">observation ambiguous = ${convergenceAnalytics.ambiguous_entities ?? 0}</p>
    <p class="muted">symbolically underdetermined = ${convergenceAnalytics.symbolically_underdetermined_entities ?? 0}</p>
    <p class="muted">observationally underdetermined = ${convergenceAnalytics.observationally_underdetermined_entities ?? 0}</p>
  `;
  candidateResolutionList.appendChild(summaryCard);
  candidateResolutions.forEach((candidateResolution) => {
    const card = document.createElement("article");
    card.className = "sphere-card";
    card.innerHTML = `
      <h3>${candidateResolution.entity}</h3>
      <p>candidates = ${candidateResolution.total_candidates}</p>
      <p class="muted">rejected = ${candidateResolution.rejected_candidates}</p>
      <p class="muted">skipped = ${candidateResolution.skipped_candidates ?? 0}</p>
      <p class="muted">mode = ${candidateResolution.convergence_mode || "direct"}</p>
      <p class="muted">observation mode = ${candidateResolution.observation_mode || "determinate"}</p>
      <p class="muted">observation labels = ${(candidateResolution.observation_labels || []).join(", ") || "none"}</p>
      <p class="muted">symbolically underdetermined = ${candidateResolution.symbolically_underdetermined ? "yes" : "no"}</p>
      <p class="muted">observationally underdetermined = ${candidateResolution.observationally_underdetermined ? "yes" : "no"}</p>
      <p class="muted">selected = ${candidateResolution.selected_candidate || "none"}</p>
      <p class="muted">score = ${candidateResolution.selected_score || "n/a"}</p>
      <p class="muted">top score = ${candidateResolution.top_score || "n/a"}</p>
      <p class="muted">top labels = ${(candidateResolution.top_labels || []).join(", ") || "none"}</p>
      <p class="muted">tie broken = ${candidateResolution.tie_broken ? "yes" : "no"}</p>
      <p class="muted">equivalent top labels = ${(candidateResolution.equivalent_top_labels || []).join(", ") || "none"}</p>
      <p class="muted">observationally equivalent tie = ${candidateResolution.observationally_equivalent_tie ? "yes" : "no"}</p>
      <p class="muted">repaired after selection = ${candidateResolution.repaired_after_selection ? "yes" : "no"}</p>
    `;
    candidateResolutionList.appendChild(card);
  });
}

function renderCandidateComparison() {
  if (!state.report) {
    candidateComparisonList.innerHTML =
      '<p class="muted">Load a Phase I report to compare fallback and repaired selection.</p>';
    return;
  }

  const candidateResolutions = state.report.candidate_resolutions || [];
  if (candidateResolutions.length === 0) {
    if ((state.report.candidate_inventory || []).length > 0) {
      candidateComparisonList.innerHTML =
        '<p class="muted">Static analyze reports show candidate inventories and resolution hints, but not runtime comparison outcomes.</p>';
    } else {
      candidateComparisonList.innerHTML =
        '<p class="muted">Comparison is available for candidate-resolution reports.</p>';
    }
    return;
  }

  candidateComparisonList.innerHTML = "";

  const candidateResolution = candidateResolutions[0];
  const summary = document.createElement("article");
  summary.className = "sphere-card";
  summary.innerHTML = `
    <h3>Current Pattern</h3>
    <p>mode = ${candidateResolution.convergence_mode || "direct"}</p>
    <p class="muted">observation mode = ${candidateResolution.observation_mode || "determinate"}</p>
    <p class="muted">observation labels = ${(candidateResolution.observation_labels || []).join(", ") || "none"}</p>
    <p>selected = ${candidateResolution.selected_candidate || "none"}</p>
    <p class="muted">rejected = ${candidateResolution.rejected_candidates}</p>
    <p class="muted">skipped = ${candidateResolution.skipped_candidates ?? 0}</p>
    <p class="muted">top labels = ${(candidateResolution.top_labels || []).join(", ") || "none"}</p>
    <p class="muted">tie broken = ${candidateResolution.tie_broken ? "yes" : "no"}</p>
    <p class="muted">symbolically underdetermined = ${candidateResolution.symbolically_underdetermined ? "yes" : "no"}</p>
    <p class="muted">observationally underdetermined = ${candidateResolution.observationally_underdetermined ? "yes" : "no"}</p>
    <p class="muted">observationally equivalent tie = ${candidateResolution.observationally_equivalent_tie ? "yes" : "no"}</p>
    <p class="muted">repaired after selection = ${candidateResolution.repaired_after_selection ? "yes" : "no"}</p>
  `;
  candidateComparisonList.appendChild(summary);

  CANDIDATE_COMPARISON_SAMPLES.forEach((sample) => {
    const card = document.createElement("article");
    card.className = "sphere-card";

    const title = document.createElement("h3");
    title.textContent = sample.label;

    const note = document.createElement("p");
    note.className = "muted";
    note.textContent =
      sample.label === "fallback"
        ? "The highest-scoring candidate is rejected, so a lower-scoring admissible candidate is selected."
        : sample.label === "repaired"
          ? "The highest-scoring candidate is selected and repaired into admissibility by the hard law layer."
          : sample.label === "deferred"
            ? "A top-score ambiguity is deferred explicitly, so the entity remains unresolved at the observation layer."
          : sample.label === "tie"
            ? "Two candidates share the top score, so deterministic tie-breaking selects one and records the other as skipped."
            : "Two candidates share the top score and also collapse to the same observed result, exposing a small observational-equivalence case.";

    const button = document.createElement("button");
    button.type = "button";
    button.textContent = sample.path === sampleSelect.value ? "Loaded" : `Load ${sample.label}`;
    button.disabled = sample.path === sampleSelect.value;
    button.addEventListener("click", async () => {
      sampleSelect.value = sample.path;
      await loadSample(sample.path);
    });

    card.appendChild(title);
    card.appendChild(note);
    card.appendChild(button);
    candidateComparisonList.appendChild(card);
  });
}

function renderAnalyticsList() {
  if (!state.report) {
    analyticsList.innerHTML = '<p class="muted">Load a report to inspect law totals and outcome distribution.</p>';
    return;
  }

  const analytics = state.report.analytics || defaultAnalytics(state.report.constraints || []);
  analyticsList.innerHTML = `
    <article class="sphere-card">
      <h3>Constraint Totals</h3>
      <p>total = ${analytics.total_constraints ?? 0}</p>
      <p class="muted">invariant = ${analytics.invariant_constraints ?? 0}, boundary = ${analytics.boundary_constraints ?? 0}, interaction = ${analytics.interaction_constraints ?? 0}</p>
      <p class="muted">idle = ${analytics.idle_constraints ?? 0}, fired = ${analytics.fired_constraints ?? 0}, repaired = ${analytics.repaired_constraints ?? 0}, contradicted = ${analytics.contradicted_constraints ?? 0}</p>
    </article>
  `;
}

function renderComparisonList() {
  if (!state.report) {
    comparisonList.innerHTML = '<p class="muted">Load a forbidden-region report to compare reject, clamp, and reflect.</p>';
    return;
  }

  const regionLaw = (state.report.constraints || []).find((constraint) => constraint.kind === "not_inside");
  if (!regionLaw) {
    comparisonList.innerHTML = '<p class="muted">Comparison is available for forbidden-region laws.</p>';
    return;
  }

  comparisonList.innerHTML = "";

  const summary = document.createElement("article");
  summary.className = "sphere-card";
  const outcome =
    regionLaw.outcome || (state.report.status === "error" ? "contradicted" : "idle");
  summary.innerHTML = `
    <h3>Current Outcome</h3>
    <p>policy = ${regionLaw.policy || "implicit"}</p>
    <p class="muted">category = ${regionLaw.category || "boundary"}</p>
    <p class="muted">outcome = ${outcome}</p>
  `;
  comparisonList.appendChild(summary);

  POLICY_COMPARISON_SAMPLES.forEach((sample) => {
    const card = document.createElement("article");
    card.className = "sphere-card";

    const title = document.createElement("h3");
    title.textContent = sample.label;

    const note = document.createElement("p");
    note.className = "muted";
    note.textContent =
      sample.label === "reject"
        ? "Stops the world when the forbidden boundary is crossed."
        : sample.label === "clamp"
          ? "Projects the sphere back to the nearest admissible boundary."
          : "Reflects the sphere off the forbidden boundary.";

    const button = document.createElement("button");
    button.type = "button";
    button.textContent = sample.path === sampleSelect.value ? "Loaded" : `Load ${sample.label}`;
    button.disabled = sample.path === sampleSelect.value;
    button.addEventListener("click", async () => {
      sampleSelect.value = sample.path;
      await loadSample(sample.path);
    });

    card.appendChild(title);
    card.appendChild(note);
    card.appendChild(button);
    comparisonList.appendChild(card);
  });
}

function compute3DBounds(snapshots, draftSpheres = [], floorOffset = null) {
  const points = [
    ...snapshots.flatMap((snapshot) => snapshot.spheres.map((sphere) => sphere.position)),
    ...draftSpheres.map((sphere) => sphere.position),
  ];
  if (points.length === 0) {
    points.push({ x: -5, y: -5, z: -5 }, { x: 5, y: 5, z: 5 });
  }
  if (floorOffset !== null) {
    points.push({ x: 0, y: floorOffset, z: 0 });
  }
  if (state.editor.regionEnabled) {
    const region = normalizedRegion();
    points.push({ x: region.minX, y: region.minY, z: 0 });
    points.push({ x: region.maxX, y: region.maxY, z: 0 });
  }
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const zs = points.map((point) => point.z);
  const minX = Math.min(...xs);
  const maxX = Math.max(...xs);
  const minY = Math.min(...ys);
  const maxY = Math.max(...ys);
  const minZ = Math.min(...zs);
  const maxZ = Math.max(...zs);
  const spanX = Math.max(maxX - minX, 1);
  const spanY = Math.max(maxY - minY, 1);
  const spanZ = Math.max(maxZ - minZ, 1);
  return {
    minX: minX - spanX * 0.15,
    maxX: maxX + spanX * 0.15,
    minY: minY - spanY * 0.15,
    maxY: maxY + spanY * 0.15,
    minZ: minZ - spanZ * 0.15,
    maxZ: maxZ + spanZ * 0.15,
  };
}

function projectPoint2D(position, bounds, plot, projection) {
  const [x, y] = selectAxes2D(position, projection);
  return {
    x: plot.left + ((x - bounds[`min${projectionAxis(projection, 0)}`]) / axisSpan(bounds, projectionAxis(projection, 0))) * (plot.right - plot.left),
    y: plot.bottom - ((y - bounds[`min${projectionAxis(projection, 1)}`]) / axisSpan(bounds, projectionAxis(projection, 1))) * (plot.bottom - plot.top),
  };
}

function projectVector2D(velocity, bounds, plot, projection) {
  const [x, y] = selectAxes2D(velocity, projection);
  return {
    x: x * ((plot.right - plot.left) / axisSpan(bounds, projectionAxis(projection, 0))),
    y: -y * ((plot.bottom - plot.top) / axisSpan(bounds, projectionAxis(projection, 1))),
  };
}

function selectAxes2D(vector, projection) {
  return projection === "xz" ? [vector.x, vector.z] : [vector.x, vector.y];
}

function projectionAxis(projection, index) {
  if (projection === "xz") return index === 0 ? "X" : "Z";
  return index === 0 ? "X" : "Y";
}

function axisSpan(bounds, axisName) {
  return bounds[`max${axisName}`] - bounds[`min${axisName}`] || 1;
}

function projectPoint3D(position, bounds, plot) {
  const normalized = {
    x: normalize(position.x, bounds.minX, bounds.maxX) - 0.5,
    y: normalize(position.y, bounds.minY, bounds.maxY) - 0.5,
    z: normalize(position.z, bounds.minZ, bounds.maxZ) - 0.5,
  };

  const yaw = degToRad(state.camera.yaw);
  const pitch = degToRad(state.camera.pitch);

  const yawed = {
    x: normalized.x * Math.cos(yaw) - normalized.z * Math.sin(yaw),
    y: normalized.y,
    z: normalized.x * Math.sin(yaw) + normalized.z * Math.cos(yaw),
  };

  const pitched = {
    x: yawed.x,
    y: yawed.y * Math.cos(pitch) - yawed.z * Math.sin(pitch),
    z: yawed.y * Math.sin(pitch) + yawed.z * Math.cos(pitch),
  };

  const perspective = 1.2 / (pitched.z + 2.8);
  const scale = ((plot.right - plot.left) * 0.86) * (state.camera.zoom / 100);
  return {
    point: {
      x: (plot.left + plot.right) / 2 + pitched.x * scale * perspective,
      y: (plot.top + plot.bottom) / 2 - pitched.y * scale * perspective,
    },
    depth: pitched.z,
  };
}

function normalize(value, min, max) {
  return (value - min) / (max - min || 1);
}

function drawArrowHead2D(end, vector, color) {
  const length = Math.hypot(vector.x, vector.y);
  if (length < 1e-6) return;
  const ux = vector.x / length;
  const uy = vector.y / length;
  const size = 10;
  const left = {
    x: end.x - ux * size - uy * size * 0.6,
    y: end.y - uy * size + ux * size * 0.6,
  };
  const right = {
    x: end.x - ux * size + uy * size * 0.6,
    y: end.y - uy * size - ux * size * 0.6,
  };

  context.fillStyle = color;
  context.beginPath();
  context.moveTo(end.x, end.y);
  context.lineTo(left.x, left.y);
  context.lineTo(right.x, right.y);
  context.closePath();
  context.fill();
}

function axisLabel(viewMode) {
  if (viewMode === "3d") {
    return "view: 3D perspective with x/y/z axes";
  }
  return viewMode === "xz" ? "axes: x-horizontal, z-vertical" : "axes: x-horizontal, y-vertical";
}

function lerp(a, b, t) {
  return a + (b - a) * t;
}

function degToRad(value) {
  return (value * Math.PI) / 180;
}

function roundRect(ctx, x, y, width, height, radius) {
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.arcTo(x + width, y, x + width, y + height, radius);
  ctx.arcTo(x + width, y + height, x, y + height, radius);
  ctx.arcTo(x, y + height, x, y, radius);
  ctx.arcTo(x, y, x + width, y, radius);
  ctx.closePath();
}

function updateSelectedSphere(mutator) {
  const sphere = state.editor.spheres.find((item) => item.id === state.editor.selectedId);
  if (!sphere) return;
  mutator(sphere);
  syncDraftControls();
  render();
}

function syncDraftControls() {
  draftSphereSelect.innerHTML = "";
  state.editor.spheres.forEach((sphere) => {
    const option = document.createElement("option");
    option.value = sphere.id;
    option.textContent = sphere.name;
    draftSphereSelect.appendChild(option);
  });
  if (!state.editor.selectedId && state.editor.spheres.length > 0) {
    state.editor.selectedId = state.editor.spheres[0].id;
  }
  draftSphereSelect.value = state.editor.selectedId || "";

  const sphere = state.editor.spheres.find((item) => item.id === state.editor.selectedId);
  draftName.value = sphere?.name || "";
  draftRadius.value = String(sphere?.radius ?? 1);
  draftVx.value = String(sphere?.velocity.x ?? 0);
  draftVy.value = String(sphere?.velocity.y ?? 0);
  draftVz.value = String(sphere?.velocity.z ?? 0);
  draftFloorEnabled.checked = state.editor.floorEnabled;
  draftFloorOffset.value = String(state.editor.floorOffset);
  draftRegionEnabled.checked = state.editor.regionEnabled;
  draftRegionMinX.value = String(state.editor.region.minX);
  draftRegionMinY.value = String(state.editor.region.minY);
  draftRegionMaxX.value = String(state.editor.region.maxX);
  draftRegionMaxY.value = String(state.editor.region.maxY);
  draftHint.textContent = state.editor.enabled
    ? "Edit Draft is on. In XY view, click empty space to place a sphere or click a draft sphere to select it."
    : "Turn on Edit Draft, switch to XY, then click the canvas to place spheres.";
  if (!state.editor.enabled && state.editor.spheres.length === 0) {
    setDraftStatus("Run Draft sends the generated scene through the local runtime.", "muted");
  }
}

function syncDraftOutput() {
  draftOutput.value = generateDraftDSL();
}

function generateDraftDSL() {
  const lines = [];
  state.editor.spheres.forEach((sphere) => {
    lines.push(`sphere ${sphere.name}`);
  });

  if (state.editor.floorEnabled) {
    lines.push("plane floor");
  }

  if (state.editor.regionEnabled) {
    lines.push("region forbidden_zone");
  }

  if (lines.length > 0) lines.push("");

  state.editor.spheres.forEach((sphere) => {
    lines.push(`position(${sphere.name}) = (${formatNum(sphere.position.x)}, ${formatNum(sphere.position.y)}, ${formatNum(sphere.position.z)})`);
    lines.push(`velocity(${sphere.name}) = (${formatNum(sphere.velocity.x)}, ${formatNum(sphere.velocity.y)}, ${formatNum(sphere.velocity.z)})`);
    lines.push(`radius(${sphere.name}) = ${formatNum(sphere.radius)}`);
    lines.push("");
  });

  if (state.editor.floorEnabled) {
    lines.push("normal(floor) = (0, 1, 0)");
    lines.push(`offset(floor) = ${formatNum(state.editor.floorOffset)}`);
    lines.push("");
  }

  if (state.editor.regionEnabled) {
    const region = normalizedRegion();
    lines.push(`min(forbidden_zone) = (${formatNum(region.minX)}, ${formatNum(region.minY)}, 0)`);
    lines.push(`max(forbidden_zone) = (${formatNum(region.maxX)}, ${formatNum(region.maxY)}, 0)`);
    lines.push("");
  }

  lines.push("constraint:");
  const accepted = buildCandidateConstraints().filter((candidate) =>
    state.editor.acceptedConstraints.has(candidate.id),
  );
  if (accepted.length > 0) {
    accepted.forEach((candidate) => {
      lines.push(`    ${candidate.expression}`);
    });
  } else {
    lines.push("    # add constraints here");
  }

  lines.push("");
  lines.push("observe:");
  lines.push("    snapshot at 0");
  lines.push("    snapshot at 1");
  lines.push("    snapshot at 2");
  lines.push("    snapshot at 3");

  return lines.join("\n");
}

function formatNum(value) {
  return Number(value).toFixed(2).replace(/\.00$/, "");
}

function canvasToWorldXY(event) {
  const rect = canvas.getBoundingClientRect();
  const x = ((event.clientX - rect.left) / rect.width) * canvas.width;
  const y = ((event.clientY - rect.top) / rect.height) * canvas.height;
  const plot = { left: 78, top: 88, right: canvas.width - 78, bottom: canvas.height - 92 };
  const bounds = compute3DBounds(state.report?.snapshots || [], state.editor.spheres, state.editor.floorEnabled ? state.editor.floorOffset : null);
  const worldX = bounds.minX + ((x - plot.left) / (plot.right - plot.left)) * (bounds.maxX - bounds.minX);
  const worldY = bounds.minY + ((plot.bottom - y) / (plot.bottom - plot.top)) * (bounds.maxY - bounds.minY);
  return { x: clamp(worldX, bounds.minX, bounds.maxX), y: clamp(worldY, bounds.minY, bounds.maxY) };
}

function findDraftSphereAt(world) {
  return state.editor.spheres.find((sphere) => {
    const dx = sphere.position.x - world.x;
    const dy = sphere.position.y - world.y;
    return Math.hypot(dx, dy) <= Math.max(sphere.radius, 0.6);
  });
}

function clamp(value, min, max) {
  return Math.max(min, Math.min(max, value));
}

function updateRegionValue(key, rawValue) {
  const value = Number(rawValue);
  if (Number.isNaN(value)) return;
  state.editor.region[key] = value;
  syncDraftOutput();
  render();
}

function normalizedRegion() {
  return {
    minX: Math.min(state.editor.region.minX, state.editor.region.maxX),
    minY: Math.min(state.editor.region.minY, state.editor.region.maxY),
    maxX: Math.max(state.editor.region.minX, state.editor.region.maxX),
    maxY: Math.max(state.editor.region.minY, state.editor.region.maxY),
  };
}

function renderConstraintCandidates() {
  const candidates = buildCandidateConstraints();
  constraintCandidates.innerHTML = "";

  if (candidates.length === 0) {
    constraintCandidates.innerHTML = '<p class="muted">Place objects to generate candidate constraints.</p>';
    return;
  }

  const validIds = new Set(candidates.map((candidate) => candidate.id));
  state.editor.acceptedConstraints.forEach((id) => {
    if (!validIds.has(id)) {
      state.editor.acceptedConstraints.delete(id);
    }
  });

  candidates.forEach((candidate) => {
    const article = document.createElement("article");
    article.className = "candidate-card";
    const checked = state.editor.acceptedConstraints.has(candidate.id) ? "checked" : "";
    article.innerHTML = `
      <label>
        <input type="checkbox" data-candidate-id="${candidate.id}" ${checked}>
        <span>
          <strong>${candidate.expression}</strong>
          <p>${candidate.reason}</p>
        </span>
      </label>
    `;
    constraintCandidates.appendChild(article);
  });

  constraintCandidates.querySelectorAll("input[type='checkbox']").forEach((checkbox) => {
    checkbox.addEventListener("change", (event) => {
      const id = event.target.getAttribute("data-candidate-id");
      if (!id) return;
      if (event.target.checked) {
        state.editor.acceptedConstraints.add(id);
      } else {
        state.editor.acceptedConstraints.delete(id);
      }
      syncDraftOutput();
    });
  });
}

function setDraftStatus(message, tone) {
  draftStatus.textContent = message;
  draftStatus.classList.remove("status-ok", "status-error", "status-busy");
  if (tone === "ok") {
    draftStatus.classList.add("status-ok");
  } else if (tone === "error") {
    draftStatus.classList.add("status-error");
  } else if (tone === "busy") {
    draftStatus.classList.add("status-busy");
  }
}

function buildCandidateConstraints() {
  const candidates = [];

  if (state.editor.floorEnabled) {
    state.editor.spheres.forEach((sphere) => {
      candidates.push({
        id: `reflect:${sphere.id}`,
        expression: `reflect_on_collision(${sphere.name}, floor)`,
        reason: `A floor is present, so ${sphere.name} can be given a reflection law on contact.`,
      });
    });
  }

  state.editor.spheres.forEach((sphere) => {
    const speed = Math.hypot(sphere.velocity.x, sphere.velocity.y, sphere.velocity.z);
    if (speed > 0.05) {
      const limit = Math.max(0.5, Math.ceil(speed * 1.25 * 10) / 10);
      candidates.push({
        id: `speed:${sphere.id}`,
        expression: `speed(${sphere.name}) <= ${formatNum(limit)}`,
        reason: `${sphere.name} already has motion, so a speed bound can stabilize or document admissible movement.`,
      });
    }
  });

  if (state.editor.regionEnabled) {
    const region = normalizedRegion();
    state.editor.spheres.forEach((sphere) => {
      const inside =
        sphere.position.x >= region.minX &&
        sphere.position.x <= region.maxX &&
        sphere.position.y >= region.minY &&
        sphere.position.y <= region.maxY;
      candidates.push({
        id: `region:${sphere.id}`,
        expression: `not inside(${sphere.name}, forbidden_zone)`,
        reason: inside
          ? `${sphere.name} is currently inside the forbidden region, so this candidate highlights an immediate contradiction to resolve.`
          : `${sphere.name} can be constrained to remain outside the forbidden region.`,
      });
    });
  }

  for (let i = 0; i < state.editor.spheres.length; i += 1) {
    for (let j = i + 1; j < state.editor.spheres.length; j += 1) {
      const left = state.editor.spheres[i];
      const right = state.editor.spheres[j];
      const approaching = isPairApproaching(left, right);
      candidates.push({
        id: `elastic:${left.id}:${right.id}`,
        expression: `elastic_collision(${left.name}, ${right.name})`,
        reason: approaching
          ? `${left.name} and ${right.name} are moving toward one another, so elastic collision is a strong interaction candidate.`
          : `${left.name} and ${right.name} are both dynamic spheres, so pairwise collision can still be proposed as an interaction law.`,
      });
    }
  }

  return candidates;
}

function isPairApproaching(left, right) {
  const dx = right.position.x - left.position.x;
  const dy = right.position.y - left.position.y;
  const dz = right.position.z - left.position.z;
  const dvx = right.velocity.x - left.velocity.x;
  const dvy = right.velocity.y - left.velocity.y;
  const dvz = right.velocity.z - left.velocity.z;
  return dx * dvx + dy * dvy + dz * dvz < 0;
}

syncViewLabels();
syncCameraLabels();
syncDraftControls();
setDraftStatus("Run Draft sends the generated scene through the local runtime.", "muted");
