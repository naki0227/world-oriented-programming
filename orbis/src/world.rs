use std::{collections::BTreeMap, fmt};

use crate::parser::{ActionCandidateDecl, ActionDirectiveDecl, Program};

const EPSILON: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Result<Self, SimulationError> {
        let length = self.magnitude();
        if length <= EPSILON {
            return Err(SimulationError::InvalidPlaneNormal);
        }
        Ok(Self::new(self.x / length, self.y / length, self.z / length))
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f64,
    pub last_update_time: f64,
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub name: String,
    pub normal: Vec3,
    pub offset: f64,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub name: String,
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Clone, Debug)]
pub enum Constraint {
    ReflectOnCollision {
        sphere_index: usize,
        plane_index: usize,
    },
    VelocityLimit {
        sphere_index: usize,
        max_speed: f64,
        policy: RepairPolicy,
    },
    NotInside {
        sphere_index: usize,
        policy: RepairPolicy,
    },
    BetweenPlanes {
        sphere_index: usize,
        lower_plane_index: usize,
        upper_plane_index: usize,
        policy: RepairPolicy,
    },
    InsidePlanes {
        sphere_index: usize,
        plane_indices: Vec<usize>,
        policy: RepairPolicy,
    },
    ThroughGate {
        sphere_index: usize,
        plane_index: usize,
        gate_region_index: usize,
        policy: RepairPolicy,
    },
    Visible {
        observer_index: usize,
        target_index: usize,
    },
    ElasticCollision { left_index: usize, right_index: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairPolicy {
    Reject,
    Clamp,
    Reflect,
}

#[derive(Clone, Debug)]
pub struct World {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub region: Option<Region>,
    pub occluder_regions: Vec<Region>,
    pub constraints: Vec<Constraint>,
    pub constraint_traces: Vec<ConstraintTrace>,
    pub candidate_resolutions: Vec<CandidateResolution>,
    pub activity_log: Vec<ActivityEntry>,
    pub action_candidates_by_entity: BTreeMap<String, Vec<ActionCandidateDecl>>,
    pub deferred_resolution_times: BTreeMap<String, f64>,
    pub deferred_preference_triggers: BTreeMap<String, DeferredPreferenceTrigger>,
    pub visibility_preference_triggers: BTreeMap<String, Vec<VisibilityPreferenceTrigger>>,
    pub deferred_score_adjustments: BTreeMap<String, Vec<DeferredScoreAdjustment>>,
    pub deferred_speed_limit_updates: BTreeMap<String, DeferredSpeedLimitUpdate>,
}

#[derive(Clone, Debug)]
pub struct SphereSnapshot {
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub time: f64,
    pub spheres: Vec<SphereSnapshot>,
}

#[derive(Clone, Debug)]
pub struct SimulationReport {
    pub analytics: LawAnalytics,
    pub constraints: Vec<ConstraintSummary>,
    pub convergence_analytics: ConvergenceAnalytics,
    pub observation_summary: ObservationSummary,
    pub observation_timeline: Vec<ObservationCheckpoint>,
    pub candidate_resolutions: Vec<CandidateResolution>,
    pub activities: Vec<ActivityEntry>,
    pub snapshots: Vec<Snapshot>,
}

#[derive(Clone, Debug)]
pub struct ObservationCheckpoint {
    pub time: f64,
    pub status: String,
    pub representative_entities: usize,
    pub ambiguous_entities: usize,
}

#[derive(Clone, Debug)]
pub struct LawInventory {
    pub analytics: LawAnalytics,
    pub constraints: Vec<ConstraintSummary>,
    pub candidate_inventory: Vec<CandidateInventorySummary>,
    pub action_directive_inventory: Vec<ActionDirectiveSummary>,
}

#[derive(Clone, Debug)]
pub struct CandidateInventorySummary {
    pub entity: String,
    pub total_candidates: usize,
    pub labels: Vec<String>,
    pub top_score: String,
    pub top_labels: Vec<String>,
    pub top_score_tied: bool,
    pub defer_on_ambiguous_top: bool,
    pub resolution_hint: String,
}

#[derive(Clone, Debug)]
pub struct ActionDirectiveSummary {
    pub entity: String,
    pub kind: String,
    pub argument: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DeferredPreferenceTrigger {
    pub label: String,
    pub time: f64,
}

#[derive(Clone, Debug)]
pub struct VisibilityPreferenceTrigger {
    pub label: String,
    pub target: String,
    pub condition: VisibilityCondition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisibilityCondition {
    Visible,
    Occluded,
}

#[derive(Clone, Debug)]
pub struct DeferredScoreAdjustment {
    pub label: String,
    pub time: f64,
    pub delta: f64,
}

#[derive(Clone, Debug)]
pub struct DeferredSpeedLimitUpdate {
    pub time: f64,
    pub limit: f64,
}

#[derive(Clone, Debug)]
pub struct CandidateResolution {
    pub entity: String,
    pub total_candidates: usize,
    pub rejected_candidates: usize,
    pub skipped_candidates: usize,
    pub convergence_mode: String,
    pub observation_mode: String,
    pub observation_labels: Vec<String>,
    pub symbolically_underdetermined: bool,
    pub observationally_underdetermined: bool,
    pub selected_candidate: Option<String>,
    pub selected_score: Option<String>,
    pub top_score: String,
    pub top_labels: Vec<String>,
    pub tie_broken: bool,
    pub equivalent_top_labels: Vec<String>,
    pub observationally_equivalent_tie: bool,
    pub repaired_after_selection: bool,
    pub observed_while_deferred: usize,
    pub deferred_past_initial_frontier: bool,
    pub resolved_from_deferred: bool,
    pub resolved_at_observation_time: Option<String>,
    pub preferred_label: Option<String>,
    pub active_score_adjustments: Vec<String>,
    pub active_law_updates: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ConvergenceAnalytics {
    pub candidate_entities: usize,
    pub direct_entities: usize,
    pub fallback_entities: usize,
    pub repaired_entities: usize,
    pub deferred_entities: usize,
    pub preference_resolved_entities: usize,
    pub rescore_resolved_entities: usize,
    pub law_updated_entities: usize,
    pub tie_broken_entities: usize,
    pub equivalent_tie_entities: usize,
    pub determinate_entities: usize,
    pub representative_entities: usize,
    pub ambiguous_entities: usize,
    pub symbolically_underdetermined_entities: usize,
    pub observationally_underdetermined_entities: usize,
    pub rejected_candidates_total: usize,
    pub skipped_candidates_total: usize,
}

#[derive(Clone, Debug)]
pub struct ObservationSummary {
    pub status: String,
    pub representative_entities: usize,
    pub ambiguous_entities: usize,
}

#[derive(Clone, Debug)]
pub struct LawAnalytics {
    pub total_constraints: usize,
    pub invariant_constraints: usize,
    pub boundary_constraints: usize,
    pub interaction_constraints: usize,
    pub idle_constraints: usize,
    pub fired_constraints: usize,
    pub repaired_constraints: usize,
    pub contradicted_constraints: usize,
}

#[derive(Clone, Debug)]
pub struct ConstraintSummary {
    pub kind: String,
    pub category: String,
    pub targets: Vec<String>,
    pub policy: String,
    pub supported_policies: Vec<String>,
    pub outcome: String,
    pub fired_count: usize,
    pub repaired_count: usize,
    pub contradicted_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ConstraintTrace {
    pub fired_count: usize,
    pub repaired_count: usize,
    pub contradicted_count: usize,
}

#[derive(Clone, Debug)]
pub struct ActivityEntry {
    pub time: f64,
    pub kind: String,
    pub targets: Vec<String>,
    pub policy: String,
    pub action: String,
}

impl SimulationReport {
    pub fn to_json(&self, source: &str) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(source)));
        json.push_str("  \"analytics\": {\n");
        json.push_str(&format!(
            "    \"total_constraints\": {},\n",
            self.analytics.total_constraints
        ));
        json.push_str(&format!(
            "    \"invariant_constraints\": {},\n",
            self.analytics.invariant_constraints
        ));
        json.push_str(&format!(
            "    \"boundary_constraints\": {},\n",
            self.analytics.boundary_constraints
        ));
        json.push_str(&format!(
            "    \"interaction_constraints\": {},\n",
            self.analytics.interaction_constraints
        ));
        json.push_str(&format!(
            "    \"idle_constraints\": {},\n",
            self.analytics.idle_constraints
        ));
        json.push_str(&format!(
            "    \"fired_constraints\": {},\n",
            self.analytics.fired_constraints
        ));
        json.push_str(&format!(
            "    \"repaired_constraints\": {},\n",
            self.analytics.repaired_constraints
        ));
        json.push_str(&format!(
            "    \"contradicted_constraints\": {}\n",
            self.analytics.contradicted_constraints
        ));
        json.push_str("  },\n");
        json.push_str("  \"constraints\": [\n");
        for (index, constraint) in self.constraints.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&constraint.kind)
            ));
            json.push_str(&format!(
                "      \"category\": \"{}\",\n",
                escape_json(&constraint.category)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in constraint.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != constraint.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&constraint.policy)
            ));
            json.push_str("      \"supported_policies\": [");
            for (policy_index, supported_policy) in
                constraint.supported_policies.iter().enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                if policy_index + 1 != constraint.supported_policies.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"outcome\": \"{}\",\n",
                escape_json(&constraint.outcome)
            ));
            json.push_str(&format!(
                "      \"fired_count\": {},\n",
                constraint.fired_count
            ));
            json.push_str(&format!(
                "      \"repaired_count\": {},\n",
                constraint.repaired_count
            ));
            json.push_str(&format!(
                "      \"contradicted_count\": {}\n",
                constraint.contradicted_count
            ));
            json.push_str("    }");
            if index + 1 != self.constraints.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"convergence_analytics\": {\n");
        json.push_str(&format!(
            "    \"candidate_entities\": {},\n",
            self.convergence_analytics.candidate_entities
        ));
        json.push_str(&format!(
            "    \"direct_entities\": {},\n",
            self.convergence_analytics.direct_entities
        ));
        json.push_str(&format!(
            "    \"fallback_entities\": {},\n",
            self.convergence_analytics.fallback_entities
        ));
        json.push_str(&format!(
            "    \"repaired_entities\": {},\n",
            self.convergence_analytics.repaired_entities
        ));
        json.push_str(&format!(
            "    \"deferred_entities\": {},\n",
            self.convergence_analytics.deferred_entities
        ));
        json.push_str(&format!(
            "    \"preference_resolved_entities\": {},\n",
            self.convergence_analytics.preference_resolved_entities
        ));
        json.push_str(&format!(
            "    \"law_updated_entities\": {},\n",
            self.convergence_analytics.law_updated_entities
        ));
        json.push_str(&format!(
            "    \"rescore_resolved_entities\": {},\n",
            self.convergence_analytics.rescore_resolved_entities
        ));
        json.push_str(&format!(
            "    \"tie_broken_entities\": {},\n",
            self.convergence_analytics.tie_broken_entities
        ));
        json.push_str(&format!(
            "    \"equivalent_tie_entities\": {},\n",
            self.convergence_analytics.equivalent_tie_entities
        ));
        json.push_str(&format!(
            "    \"determinate_entities\": {},\n",
            self.convergence_analytics.determinate_entities
        ));
        json.push_str(&format!(
            "    \"representative_entities\": {},\n",
            self.convergence_analytics.representative_entities
        ));
        json.push_str(&format!(
            "    \"ambiguous_entities\": {},\n",
            self.convergence_analytics.ambiguous_entities
        ));
        json.push_str(&format!(
            "    \"symbolically_underdetermined_entities\": {},\n",
            self.convergence_analytics.symbolically_underdetermined_entities
        ));
        json.push_str(&format!(
            "    \"observationally_underdetermined_entities\": {},\n",
            self.convergence_analytics.observationally_underdetermined_entities
        ));
        json.push_str(&format!(
            "    \"rejected_candidates_total\": {},\n",
            self.convergence_analytics.rejected_candidates_total
        ));
        json.push_str(&format!(
            "    \"skipped_candidates_total\": {}\n",
            self.convergence_analytics.skipped_candidates_total
        ));
        json.push_str("  },\n");
        json.push_str("  \"observation_summary\": {\n");
        json.push_str(&format!(
            "    \"status\": \"{}\",\n",
            escape_json(&self.observation_summary.status)
        ));
        json.push_str(&format!(
            "    \"representative_entities\": {},\n",
            self.observation_summary.representative_entities
        ));
        json.push_str(&format!(
            "    \"ambiguous_entities\": {}\n",
            self.observation_summary.ambiguous_entities
        ));
        json.push_str("  },\n");
        json.push_str("  \"observation_timeline\": [\n");
        for (index, checkpoint) in self.observation_timeline.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"time\": {:.6},\n", checkpoint.time));
            json.push_str(&format!(
                "      \"status\": \"{}\",\n",
                escape_json(&checkpoint.status)
            ));
            json.push_str(&format!(
                "      \"representative_entities\": {},\n",
                checkpoint.representative_entities
            ));
            json.push_str(&format!(
                "      \"ambiguous_entities\": {}\n",
                checkpoint.ambiguous_entities
            ));
            json.push_str("    }");
            if index + 1 != self.observation_timeline.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"candidate_resolutions\": [\n");
        for (index, candidate_resolution) in self.candidate_resolutions.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&candidate_resolution.entity)
            ));
            json.push_str(&format!(
                "      \"total_candidates\": {},\n",
                candidate_resolution.total_candidates
            ));
                json.push_str(&format!(
                    "      \"rejected_candidates\": {},\n",
                    candidate_resolution.rejected_candidates
                ));
            json.push_str(&format!(
                "      \"skipped_candidates\": {},\n",
                candidate_resolution.skipped_candidates
            ));
            json.push_str(&format!(
                "      \"convergence_mode\": \"{}\",\n",
                escape_json(&candidate_resolution.convergence_mode)
            ));
            json.push_str(&format!(
                "      \"observation_mode\": \"{}\",\n",
                escape_json(&candidate_resolution.observation_mode)
            ));
            json.push_str("      \"observation_labels\": [");
            for (label_index, label) in candidate_resolution.observation_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.observation_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"symbolically_underdetermined\": {},\n",
                candidate_resolution.symbolically_underdetermined
            ));
            json.push_str(&format!(
                "      \"observationally_underdetermined\": {},\n",
                candidate_resolution.observationally_underdetermined
            ));
                match &candidate_resolution.selected_candidate {
                    Some(selected_candidate) => json.push_str(&format!(
                        "      \"selected_candidate\": \"{}\",\n",
                    escape_json(selected_candidate)
                )),
                None => json.push_str("      \"selected_candidate\": null,\n"),
            }
                match &candidate_resolution.selected_score {
                    Some(selected_score) => json.push_str(&format!(
                        "      \"selected_score\": \"{}\",\n",
                        escape_json(selected_score)
                    )),
                    None => json.push_str("      \"selected_score\": null,\n"),
                }
                json.push_str(&format!(
                    "      \"top_score\": \"{}\",\n",
                    escape_json(&candidate_resolution.top_score)
                ));
            json.push_str("      \"top_labels\": [");
            for (label_index, label) in candidate_resolution.top_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"tie_broken\": {},\n",
                candidate_resolution.tie_broken
            ));
            json.push_str("      \"equivalent_top_labels\": [");
            for (label_index, label) in candidate_resolution
                .equivalent_top_labels
                .iter()
                .enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.equivalent_top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"observationally_equivalent_tie\": {},\n",
                candidate_resolution.observationally_equivalent_tie
            ));
            json.push_str(&format!(
                "      \"repaired_after_selection\": {},\n",
                candidate_resolution.repaired_after_selection
            ));
            json.push_str(&format!(
                "      \"observed_while_deferred\": {},\n",
                candidate_resolution.observed_while_deferred
            ));
            json.push_str(&format!(
                "      \"deferred_past_initial_frontier\": {},\n",
                candidate_resolution.deferred_past_initial_frontier
            ));
            json.push_str(&format!(
                "      \"resolved_from_deferred\": {},\n",
                candidate_resolution.resolved_from_deferred
            ));
            match &candidate_resolution.preferred_label {
                Some(label) => json.push_str(&format!(
                    "      \"preferred_label\": \"{}\",\n",
                    escape_json(label)
                )),
                None => json.push_str("      \"preferred_label\": null,\n"),
            }
            json.push_str("      \"active_score_adjustments\": [");
            for (adjustment_index, adjustment) in candidate_resolution
                .active_score_adjustments
                .iter()
                .enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(adjustment)));
                if adjustment_index + 1 != candidate_resolution.active_score_adjustments.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str("      \"active_law_updates\": [");
            for (update_index, update) in candidate_resolution.active_law_updates.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(update)));
                if update_index + 1 != candidate_resolution.active_law_updates.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            match &candidate_resolution.resolved_at_observation_time {
                Some(time) => json.push_str(&format!(
                    "      \"resolved_at_observation_time\": \"{}\"\n",
                    escape_json(time)
                )),
                None => json.push_str("      \"resolved_at_observation_time\": null\n"),
            }
            json.push_str("    }");
            if index + 1 != self.candidate_resolutions.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"activities\": [\n");
        for (index, activity) in self.activities.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"time\": {:.6},\n", activity.time));
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&activity.kind)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in activity.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != activity.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&activity.policy)
            ));
            json.push_str(&format!(
                "      \"action\": \"{}\"\n",
                escape_json(&activity.action)
            ));
            json.push_str("    }");
            if index + 1 != self.activities.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"snapshots\": [\n");

        for (snapshot_index, snapshot) in self.snapshots.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"time\": {:.6},\n", snapshot.time));
            json.push_str("      \"spheres\": [\n");

            for (sphere_index, sphere) in snapshot.spheres.iter().enumerate() {
                json.push_str("        {\n");
                json.push_str(&format!(
                    "          \"name\": \"{}\",\n",
                    escape_json(&sphere.name)
                ));
                json.push_str(&format!(
                    "          \"position\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }},\n",
                    sphere.position.x, sphere.position.y, sphere.position.z
                ));
                json.push_str(&format!(
                    "          \"velocity\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }}\n",
                    sphere.velocity.x, sphere.velocity.y, sphere.velocity.z
                ));
                json.push_str("        }");
                if sphere_index + 1 != snapshot.spheres.len() {
                    json.push(',');
                }
                json.push('\n');
            }

            json.push_str("      ]\n");
            json.push_str("    }");
            if snapshot_index + 1 != self.snapshots.len() {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n");
        json.push('}');
        json
    }
}

impl LawInventory {
    pub fn to_json(&self, source: &str) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(source)));
        json.push_str("  \"analytics\": {\n");
        json.push_str(&format!(
            "    \"total_constraints\": {},\n",
            self.analytics.total_constraints
        ));
        json.push_str(&format!(
            "    \"invariant_constraints\": {},\n",
            self.analytics.invariant_constraints
        ));
        json.push_str(&format!(
            "    \"boundary_constraints\": {},\n",
            self.analytics.boundary_constraints
        ));
        json.push_str(&format!(
            "    \"interaction_constraints\": {},\n",
            self.analytics.interaction_constraints
        ));
        json.push_str(&format!(
            "    \"idle_constraints\": {},\n",
            self.analytics.idle_constraints
        ));
        json.push_str(&format!(
            "    \"fired_constraints\": {},\n",
            self.analytics.fired_constraints
        ));
        json.push_str(&format!(
            "    \"repaired_constraints\": {},\n",
            self.analytics.repaired_constraints
        ));
        json.push_str(&format!(
            "    \"contradicted_constraints\": {}\n",
            self.analytics.contradicted_constraints
        ));
        json.push_str("  },\n");
        json.push_str("  \"constraints\": [\n");
        for (index, constraint) in self.constraints.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&constraint.kind)
            ));
            json.push_str(&format!(
                "      \"category\": \"{}\",\n",
                escape_json(&constraint.category)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in constraint.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != constraint.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&constraint.policy)
            ));
            json.push_str("      \"supported_policies\": [");
            for (policy_index, supported_policy) in constraint.supported_policies.iter().enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                if policy_index + 1 != constraint.supported_policies.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"outcome\": \"{}\",\n",
                escape_json(&constraint.outcome)
            ));
            json.push_str(&format!(
                "      \"fired_count\": {},\n",
                constraint.fired_count
            ));
            json.push_str(&format!(
                "      \"repaired_count\": {},\n",
                constraint.repaired_count
            ));
            json.push_str(&format!(
                "      \"contradicted_count\": {}\n",
                constraint.contradicted_count
            ));
            json.push_str("    }");
            if index + 1 != self.constraints.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"candidate_inventory\": [\n");
        for (index, candidate_inventory) in self.candidate_inventory.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&candidate_inventory.entity)
            ));
            json.push_str(&format!(
                "      \"total_candidates\": {},\n",
                candidate_inventory.total_candidates
            ));
            json.push_str("      \"labels\": [");
            for (label_index, label) in candidate_inventory.labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_inventory.labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"top_score\": \"{}\",\n",
                escape_json(&candidate_inventory.top_score)
            ));
            json.push_str("      \"top_labels\": [");
            for (label_index, label) in candidate_inventory.top_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_inventory.top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"top_score_tied\": {},\n",
                candidate_inventory.top_score_tied
            ));
            json.push_str(&format!(
                "      \"defer_on_ambiguous_top\": {},\n",
                candidate_inventory.defer_on_ambiguous_top
            ));
            json.push_str(&format!(
                "      \"resolution_hint\": \"{}\"\n",
                escape_json(&candidate_inventory.resolution_hint)
            ));
            json.push_str("    }");
            if index + 1 != self.candidate_inventory.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"action_directive_inventory\": [\n");
        for (index, directive) in self.action_directive_inventory.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&directive.entity)
            ));
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&directive.kind)
            ));
            match &directive.argument {
                Some(argument) => json.push_str(&format!(
                    "      \"argument\": \"{}\"\n",
                    escape_json(argument)
                )),
                None => json.push_str("      \"argument\": null\n"),
            }
            json.push_str("    }");
            if index + 1 != self.action_directive_inventory.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ]\n");
        json.push('}');
        json
    }
}

#[derive(Clone, Debug)]
pub struct SimulationEnvelope {
    pub source: String,
    pub status: String,
    pub report: Option<SimulationReport>,
    pub error: Option<String>,
}

impl SimulationEnvelope {
    pub fn success(source: &str, report: SimulationReport) -> Self {
        Self {
            source: source.to_string(),
            status: "ok".to_string(),
            report: Some(report),
            error: None,
        }
    }

    pub fn failure(source: &str, error: impl Into<String>) -> Self {
        Self {
            source: source.to_string(),
            status: "error".to_string(),
            report: None,
            error: Some(error.into()),
        }
    }

    pub fn failure_with_report(
        source: &str,
        error: impl Into<String>,
        report: SimulationReport,
    ) -> Self {
        Self {
            source: source.to_string(),
            status: "error".to_string(),
            report: Some(report),
            error: Some(error.into()),
        }
    }

    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(&self.source)));
        json.push_str(&format!("  \"status\": \"{}\",\n", escape_json(&self.status)));
        match &self.report {
            Some(report) => {
                match &self.error {
                    Some(error) => json.push_str(&format!(
                        "  \"error\": \"{}\",\n",
                        escape_json(error)
                    )),
                    None => json.push_str("  \"error\": null,\n"),
                }
                json.push_str("  \"analytics\": {\n");
                json.push_str(&format!(
                    "    \"total_constraints\": {},\n",
                    report.analytics.total_constraints
                ));
                json.push_str(&format!(
                    "    \"invariant_constraints\": {},\n",
                    report.analytics.invariant_constraints
                ));
                json.push_str(&format!(
                    "    \"boundary_constraints\": {},\n",
                    report.analytics.boundary_constraints
                ));
                json.push_str(&format!(
                    "    \"interaction_constraints\": {},\n",
                    report.analytics.interaction_constraints
                ));
                json.push_str(&format!(
                    "    \"idle_constraints\": {},\n",
                    report.analytics.idle_constraints
                ));
                json.push_str(&format!(
                    "    \"fired_constraints\": {},\n",
                    report.analytics.fired_constraints
                ));
                json.push_str(&format!(
                    "    \"repaired_constraints\": {},\n",
                    report.analytics.repaired_constraints
                ));
                json.push_str(&format!(
                    "    \"contradicted_constraints\": {}\n",
                    report.analytics.contradicted_constraints
                ));
                json.push_str("  },\n");
                json.push_str("  \"constraints\": [\n");
                for (index, constraint) in report.constraints.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!(
                        "      \"kind\": \"{}\",\n",
                        escape_json(&constraint.kind)
                    ));
                    json.push_str(&format!(
                        "      \"category\": \"{}\",\n",
                        escape_json(&constraint.category)
                    ));
                    json.push_str("      \"targets\": [");
                    for (target_index, target) in constraint.targets.iter().enumerate() {
                        json.push_str(&format!("\"{}\"", escape_json(target)));
                        if target_index + 1 != constraint.targets.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"policy\": \"{}\",\n",
                        escape_json(&constraint.policy)
                    ));
                    json.push_str("      \"supported_policies\": [");
                    for (policy_index, supported_policy) in
                        constraint.supported_policies.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                        if policy_index + 1 != constraint.supported_policies.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"outcome\": \"{}\",\n",
                        escape_json(&constraint.outcome)
                    ));
                    json.push_str(&format!(
                        "      \"fired_count\": {},\n",
                        constraint.fired_count
                    ));
                    json.push_str(&format!(
                        "      \"repaired_count\": {},\n",
                        constraint.repaired_count
                    ));
                    json.push_str(&format!(
                        "      \"contradicted_count\": {}\n",
                        constraint.contradicted_count
                    ));
                    json.push_str("    }");
                    if index + 1 != report.constraints.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"convergence_analytics\": {\n");
                json.push_str(&format!(
                    "    \"candidate_entities\": {},\n",
                    report.convergence_analytics.candidate_entities
                ));
                json.push_str(&format!(
                    "    \"direct_entities\": {},\n",
                    report.convergence_analytics.direct_entities
                ));
                json.push_str(&format!(
                    "    \"fallback_entities\": {},\n",
                    report.convergence_analytics.fallback_entities
                ));
                json.push_str(&format!(
                    "    \"repaired_entities\": {},\n",
                    report.convergence_analytics.repaired_entities
                ));
                json.push_str(&format!(
                    "    \"deferred_entities\": {},\n",
                    report.convergence_analytics.deferred_entities
                ));
                json.push_str(&format!(
                    "    \"preference_resolved_entities\": {},\n",
                    report.convergence_analytics.preference_resolved_entities
                ));
                json.push_str(&format!(
                    "    \"law_updated_entities\": {},\n",
                    report.convergence_analytics.law_updated_entities
                ));
                json.push_str(&format!(
                    "    \"rescore_resolved_entities\": {},\n",
                    report.convergence_analytics.rescore_resolved_entities
                ));
                json.push_str(&format!(
                    "    \"tie_broken_entities\": {},\n",
                    report.convergence_analytics.tie_broken_entities
                ));
                json.push_str(&format!(
                    "    \"equivalent_tie_entities\": {},\n",
                    report.convergence_analytics.equivalent_tie_entities
                ));
                json.push_str(&format!(
                    "    \"determinate_entities\": {},\n",
                    report.convergence_analytics.determinate_entities
                ));
                json.push_str(&format!(
                    "    \"representative_entities\": {},\n",
                    report.convergence_analytics.representative_entities
                ));
                json.push_str(&format!(
                    "    \"ambiguous_entities\": {},\n",
                    report.convergence_analytics.ambiguous_entities
                ));
                json.push_str(&format!(
                    "    \"symbolically_underdetermined_entities\": {},\n",
                    report.convergence_analytics.symbolically_underdetermined_entities
                ));
                json.push_str(&format!(
                    "    \"observationally_underdetermined_entities\": {},\n",
                    report.convergence_analytics.observationally_underdetermined_entities
                ));
                json.push_str(&format!(
                    "    \"rejected_candidates_total\": {},\n",
                    report.convergence_analytics.rejected_candidates_total
                ));
                json.push_str(&format!(
                    "    \"skipped_candidates_total\": {}\n",
                    report.convergence_analytics.skipped_candidates_total
                ));
                json.push_str("  },\n");
                json.push_str("  \"observation_summary\": {\n");
                json.push_str(&format!(
                    "    \"status\": \"{}\",\n",
                    escape_json(&report.observation_summary.status)
                ));
                json.push_str(&format!(
                    "    \"representative_entities\": {},\n",
                    report.observation_summary.representative_entities
                ));
                json.push_str(&format!(
                    "    \"ambiguous_entities\": {}\n",
                    report.observation_summary.ambiguous_entities
                ));
                json.push_str("  },\n");
                json.push_str("  \"observation_timeline\": [\n");
                for (index, checkpoint) in report.observation_timeline.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!("      \"time\": {:.6},\n", checkpoint.time));
                    json.push_str(&format!(
                        "      \"status\": \"{}\",\n",
                        escape_json(&checkpoint.status)
                    ));
                    json.push_str(&format!(
                        "      \"representative_entities\": {},\n",
                        checkpoint.representative_entities
                    ));
                    json.push_str(&format!(
                        "      \"ambiguous_entities\": {}\n",
                        checkpoint.ambiguous_entities
                    ));
                    json.push_str("    }");
                    if index + 1 != report.observation_timeline.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"candidate_resolutions\": [\n");
                for (index, candidate_resolution) in report.candidate_resolutions.iter().enumerate()
                {
                    json.push_str("    {\n");
                    json.push_str(&format!(
                        "      \"entity\": \"{}\",\n",
                        escape_json(&candidate_resolution.entity)
                    ));
                    json.push_str(&format!(
                        "      \"total_candidates\": {},\n",
                        candidate_resolution.total_candidates
                    ));
                    json.push_str(&format!(
                        "      \"rejected_candidates\": {},\n",
                        candidate_resolution.rejected_candidates
                    ));
                    json.push_str(&format!(
                        "      \"skipped_candidates\": {},\n",
                        candidate_resolution.skipped_candidates
                    ));
                    json.push_str(&format!(
                        "      \"convergence_mode\": \"{}\",\n",
                        escape_json(&candidate_resolution.convergence_mode)
                    ));
                    json.push_str(&format!(
                        "      \"observation_mode\": \"{}\",\n",
                        escape_json(&candidate_resolution.observation_mode)
                    ));
                    json.push_str("      \"observation_labels\": [");
                    for (label_index, label) in
                        candidate_resolution.observation_labels.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.observation_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"symbolically_underdetermined\": {},\n",
                        candidate_resolution.symbolically_underdetermined
                    ));
                    json.push_str(&format!(
                        "      \"observationally_underdetermined\": {},\n",
                        candidate_resolution.observationally_underdetermined
                    ));
                    match &candidate_resolution.selected_candidate {
                        Some(selected_candidate) => json.push_str(&format!(
                            "      \"selected_candidate\": \"{}\",\n",
                            escape_json(selected_candidate)
                        )),
                        None => json.push_str("      \"selected_candidate\": null,\n"),
                    }
                    match &candidate_resolution.selected_score {
                        Some(selected_score) => json.push_str(&format!(
                            "      \"selected_score\": \"{}\",\n",
                            escape_json(selected_score)
                        )),
                        None => json.push_str("      \"selected_score\": null,\n"),
                    }
                    json.push_str(&format!(
                        "      \"top_score\": \"{}\",\n",
                        escape_json(&candidate_resolution.top_score)
                    ));
                    json.push_str("      \"top_labels\": [");
                    for (label_index, label) in candidate_resolution.top_labels.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.top_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"tie_broken\": {},\n",
                        candidate_resolution.tie_broken
                    ));
                    json.push_str("      \"equivalent_top_labels\": [");
                    for (label_index, label) in candidate_resolution
                        .equivalent_top_labels
                        .iter()
                        .enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.equivalent_top_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"observationally_equivalent_tie\": {},\n",
                        candidate_resolution.observationally_equivalent_tie
                    ));
                    json.push_str(&format!(
                        "      \"repaired_after_selection\": {},\n",
                        candidate_resolution.repaired_after_selection
                    ));
                    json.push_str(&format!(
                        "      \"observed_while_deferred\": {},\n",
                        candidate_resolution.observed_while_deferred
                    ));
                    json.push_str(&format!(
                        "      \"deferred_past_initial_frontier\": {},\n",
                        candidate_resolution.deferred_past_initial_frontier
                    ));
                    json.push_str(&format!(
                        "      \"resolved_from_deferred\": {},\n",
                        candidate_resolution.resolved_from_deferred
                    ));
                    match &candidate_resolution.preferred_label {
                        Some(label) => json.push_str(&format!(
                            "      \"preferred_label\": \"{}\",\n",
                            escape_json(label)
                        )),
                        None => json.push_str("      \"preferred_label\": null,\n"),
                    }
                    json.push_str("      \"active_score_adjustments\": [");
                    for (adjustment_index, adjustment) in candidate_resolution
                        .active_score_adjustments
                        .iter()
                        .enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(adjustment)));
                        if adjustment_index + 1 != candidate_resolution.active_score_adjustments.len()
                        {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str("      \"active_law_updates\": [");
                    for (update_index, update) in candidate_resolution.active_law_updates.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(update)));
                        if update_index + 1 != candidate_resolution.active_law_updates.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    match &candidate_resolution.resolved_at_observation_time {
                        Some(time) => json.push_str(&format!(
                            "      \"resolved_at_observation_time\": \"{}\"\n",
                            escape_json(time)
                        )),
                        None => json.push_str("      \"resolved_at_observation_time\": null\n"),
                    }
                    json.push_str("    }");
                    if index + 1 != report.candidate_resolutions.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"activities\": [\n");
                for (index, activity) in report.activities.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!("      \"time\": {:.6},\n", activity.time));
                    json.push_str(&format!(
                        "      \"kind\": \"{}\",\n",
                        escape_json(&activity.kind)
                    ));
                    json.push_str("      \"targets\": [");
                    for (target_index, target) in activity.targets.iter().enumerate() {
                        json.push_str(&format!("\"{}\"", escape_json(target)));
                        if target_index + 1 != activity.targets.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"policy\": \"{}\",\n",
                        escape_json(&activity.policy)
                    ));
                    json.push_str(&format!(
                        "      \"action\": \"{}\"\n",
                        escape_json(&activity.action)
                    ));
                    json.push_str("    }");
                    if index + 1 != report.activities.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"snapshots\": [\n");
                for (snapshot_index, snapshot) in report.snapshots.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!("      \"time\": {:.6},\n", snapshot.time));
                    json.push_str("      \"spheres\": [\n");
                    for (sphere_index, sphere) in snapshot.spheres.iter().enumerate() {
                        json.push_str("        {\n");
                        json.push_str(&format!(
                            "          \"name\": \"{}\",\n",
                            escape_json(&sphere.name)
                        ));
                        json.push_str(&format!(
                            "          \"position\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }},\n",
                            sphere.position.x, sphere.position.y, sphere.position.z
                        ));
                        json.push_str(&format!(
                            "          \"velocity\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }}\n",
                            sphere.velocity.x, sphere.velocity.y, sphere.velocity.z
                        ));
                        json.push_str("        }");
                        if sphere_index + 1 != snapshot.spheres.len() {
                            json.push(',');
                        }
                        json.push('\n');
                    }
                    json.push_str("      ]\n");
                    json.push_str("    }");
                    if snapshot_index + 1 != report.snapshots.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ]\n");
            }
            None => {
                json.push_str(&format!(
                    "  \"error\": \"{}\",\n",
                    escape_json(self.error.as_deref().unwrap_or("unknown error"))
                ));
                json.push_str("  \"analytics\": {\n");
                json.push_str("    \"total_constraints\": 0,\n");
                json.push_str("    \"invariant_constraints\": 0,\n");
                json.push_str("    \"boundary_constraints\": 0,\n");
                json.push_str("    \"interaction_constraints\": 0,\n");
                json.push_str("    \"idle_constraints\": 0,\n");
                json.push_str("    \"fired_constraints\": 0,\n");
                json.push_str("    \"repaired_constraints\": 0,\n");
                json.push_str("    \"contradicted_constraints\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"constraints\": [],\n");
                json.push_str("  \"convergence_analytics\": {\n");
                json.push_str("    \"candidate_entities\": 0,\n");
                json.push_str("    \"direct_entities\": 0,\n");
                json.push_str("    \"fallback_entities\": 0,\n");
                json.push_str("    \"repaired_entities\": 0,\n");
                json.push_str("    \"deferred_entities\": 0,\n");
                json.push_str("    \"preference_resolved_entities\": 0,\n");
                json.push_str("    \"law_updated_entities\": 0,\n");
                json.push_str("    \"rescore_resolved_entities\": 0,\n");
                json.push_str("    \"tie_broken_entities\": 0,\n");
                json.push_str("    \"equivalent_tie_entities\": 0,\n");
                json.push_str("    \"determinate_entities\": 0,\n");
                json.push_str("    \"representative_entities\": 0,\n");
                json.push_str("    \"ambiguous_entities\": 0,\n");
                json.push_str("    \"symbolically_underdetermined_entities\": 0,\n");
                json.push_str("    \"observationally_underdetermined_entities\": 0,\n");
                json.push_str("    \"rejected_candidates_total\": 0,\n");
                json.push_str("    \"skipped_candidates_total\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"observation_summary\": {\n");
                json.push_str("    \"status\": \"determinate\",\n");
                json.push_str("    \"representative_entities\": 0,\n");
                json.push_str("    \"ambiguous_entities\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"candidate_resolutions\": [],\n");
                json.push_str("  \"activities\": [],\n");
                json.push_str("  \"snapshots\": []\n");
            }
        }
        json.push('}');
        json
    }
}

#[derive(Debug)]
pub enum SimulationError {
    MissingSphere,
    MissingPlane,
    MissingPosition(String),
    MissingVelocity(String),
    InvalidRadius(String),
    InvalidPlaneNormal,
    InvalidConstraint(String),
    MissingRegionBounds(String),
    VelocityLimitExceeded { sphere: String, speed: f64, limit: f64 },
    EnteredForbiddenRegion { sphere: String, region: String, time: f64 },
    LeftPlaneChannel {
        sphere: String,
        lower_plane: String,
        upper_plane: String,
        time: f64,
    },
    LeftBoundedPlaneSet {
        sphere: String,
        planes: Vec<String>,
        time: f64,
    },
    MissedGateCrossing {
        sphere: String,
        plane: String,
        gate: String,
        time: f64,
    },
    VisibilityOccluded {
        observer: String,
        target: String,
        region: String,
        time: f64,
    },
    SphereNotFound(String),
    InvalidActionCandidate(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSphere => write!(f, "program requires at least one sphere"),
            Self::MissingPlane => write!(f, "program requires at least one plane"),
            Self::MissingPosition(name) => write!(f, "missing position for entity `{name}`"),
            Self::MissingVelocity(name) => write!(f, "missing velocity for entity `{name}`"),
            Self::InvalidRadius(name) => write!(f, "missing or invalid radius for sphere `{name}`"),
            Self::InvalidPlaneNormal => write!(f, "plane normal must be non-zero"),
            Self::InvalidConstraint(message) => write!(f, "{message}"),
            Self::MissingRegionBounds(name) => {
                write!(f, "region `{name}` requires both min and max vectors")
            }
            Self::VelocityLimitExceeded {
                sphere,
                speed,
                limit,
            } => write!(
                f,
                "sphere `{sphere}` exceeded velocity limit: speed {speed:.3} > limit {limit:.3}"
            ),
            Self::EnteredForbiddenRegion { sphere, region, time } => write!(
                f,
                "sphere `{sphere}` entered forbidden region `{region}` at t={time:.3}"
            ),
            Self::LeftPlaneChannel {
                sphere,
                lower_plane,
                upper_plane,
                time,
            } => write!(
                f,
                "sphere `{sphere}` left plane-bounded space between `{lower_plane}` and `{upper_plane}` at t={time:.3}"
            ),
            Self::LeftBoundedPlaneSet { sphere, planes, time } => write!(
                f,
                "sphere `{sphere}` left plane-bounded space bounded by `{}` at t={time:.3}",
                planes.join(", ")
            ),
            Self::MissedGateCrossing {
                sphere,
                plane,
                gate,
                time,
            } => write!(
                f,
                "sphere `{sphere}` crossed plane `{plane}` outside gate `{gate}` at t={time:.3}"
            ),
            Self::VisibilityOccluded {
                observer,
                target,
                region,
                time,
            } => write!(
                f,
                "`{observer}` cannot see `{target}` through region `{region}` at t={time:.3}"
            ),
            Self::SphereNotFound(name) => write!(f, "unknown sphere `{name}`"),
            Self::InvalidActionCandidate(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for SimulationError {}

pub fn simulate_program(program: &Program) -> Result<SimulationReport, SimulationError> {
    let mut world = World::from_program(program)?;
    let mut snapshots = Vec::new();
    let mut observation_timeline = Vec::new();
    let mut observation_times = program.observe_times.clone();
    observation_times.sort_by(|a, b| a.total_cmp(b));

    for time in observation_times {
        world.advance_to(time)?;
        world.resolve_deferred_candidates_at(time)?;
        let candidate_resolutions =
            candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len() + 1);
        observation_timeline.push(observation_checkpoint(
            time,
            &ObservationSummary::from_candidate_resolutions(&candidate_resolutions),
        ));
        let mut spheres = world
            .spheres
            .iter()
            .map(|sphere| SphereSnapshot {
                name: sphere.name.clone(),
                position: sphere.position,
                velocity: sphere.velocity,
            })
            .collect::<Vec<_>>();
        spheres.sort_by(|a, b| a.name.cmp(&b.name));
        snapshots.push(Snapshot { time, spheres });
    }

    let constraints = world.constraint_summaries();
    let candidate_resolutions =
        candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len());

    Ok(SimulationReport {
        analytics: LawAnalytics::from_constraints(&constraints),
        constraints,
        convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
            &candidate_resolutions,
        ),
        observation_summary: ObservationSummary::from_candidate_resolutions(
            &candidate_resolutions,
        ),
        observation_timeline,
        candidate_resolutions,
        activities: world.activity_log.clone(),
        snapshots,
    })
}

pub fn analyze_program(program: &Program) -> Result<LawInventory, SimulationError> {
    let world = World::from_program(program)?;
    let constraints = world.constraint_summaries();
    Ok(LawInventory {
        analytics: LawAnalytics::from_constraints(&constraints),
        constraints,
        candidate_inventory: candidate_inventory_from_program(program),
        action_directive_inventory: action_directive_inventory_from_program(program),
    })
}

pub fn simulate_program_envelope(program: &Program, source: &str) -> SimulationEnvelope {
    let mut world = match World::from_program(program) {
        Ok(world) => world,
        Err(error) => return SimulationEnvelope::failure(source, error.to_string()),
    };
    let mut snapshots = Vec::new();
    let mut observation_timeline = Vec::new();
    let mut observation_times = program.observe_times.clone();
    observation_times.sort_by(|a, b| a.total_cmp(b));

    for time in observation_times {
        if let Err(error) = world.advance_to(time) {
            let constraints = world.constraint_summaries();
            let candidate_resolutions =
                candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len());
            return SimulationEnvelope::failure_with_report(
                source,
                error.to_string(),
                SimulationReport {
                    analytics: LawAnalytics::from_constraints(&constraints),
                    constraints,
                    convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
                        &candidate_resolutions,
                    ),
                    observation_summary: ObservationSummary::from_candidate_resolutions(
                        &candidate_resolutions,
                    ),
                    observation_timeline,
                    candidate_resolutions,
                    activities: world.activity_log.clone(),
                    snapshots,
                },
            );
        }
        if let Err(error) = world.resolve_deferred_candidates_at(time) {
            let constraints = world.constraint_summaries();
            let candidate_resolutions =
                candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len());
            return SimulationEnvelope::failure_with_report(
                source,
                error.to_string(),
                SimulationReport {
                    analytics: LawAnalytics::from_constraints(&constraints),
                    constraints,
                    convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
                        &candidate_resolutions,
                    ),
                    observation_summary: ObservationSummary::from_candidate_resolutions(
                        &candidate_resolutions,
                    ),
                    observation_timeline,
                    candidate_resolutions,
                    activities: world.activity_log.clone(),
                    snapshots,
                },
            );
        }
        let candidate_resolutions =
            candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len() + 1);
        observation_timeline.push(observation_checkpoint(
            time,
            &ObservationSummary::from_candidate_resolutions(&candidate_resolutions),
        ));
        let mut spheres = world
            .spheres
            .iter()
            .map(|sphere| SphereSnapshot {
                name: sphere.name.clone(),
                position: sphere.position,
                velocity: sphere.velocity,
            })
            .collect::<Vec<_>>();
        spheres.sort_by(|a, b| a.name.cmp(&b.name));
        snapshots.push(Snapshot { time, spheres });
    }

    let constraints = world.constraint_summaries();
    let candidate_resolutions =
        candidate_resolutions_for_report(&world.candidate_resolutions, snapshots.len());

    SimulationEnvelope::success(
        source,
        SimulationReport {
            analytics: LawAnalytics::from_constraints(&constraints),
            constraints,
            convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
                &candidate_resolutions,
            ),
            observation_summary: ObservationSummary::from_candidate_resolutions(
                &candidate_resolutions,
            ),
            observation_timeline,
            candidate_resolutions,
            activities: world.activity_log.clone(),
            snapshots,
        },
    )
}

impl World {
    pub fn from_program(program: &Program) -> Result<Self, SimulationError> {
        let sphere_decls = program
            .entities
            .iter()
            .filter(|entity| entity.kind == "sphere")
            .collect::<Vec<_>>();
        if sphere_decls.is_empty() {
            return Err(SimulationError::MissingSphere);
        }

        let plane_decls = program
            .entities
            .iter()
            .filter(|entity| entity.kind == "plane")
            .collect::<Vec<_>>();
        if plane_decls.is_empty() {
            return Err(SimulationError::MissingPlane);
        }
        let region_decls = program
            .entities
            .iter()
            .filter(|entity| entity.kind == "region")
            .collect::<Vec<_>>();

        let mut spheres = Vec::new();
        for sphere_decl in sphere_decls {
            let sphere_name = sphere_decl.name.clone();
            spheres.push(Sphere {
                name: sphere_name.clone(),
                position: program
                    .vec3_property("position", &sphere_name)
                    .ok_or_else(|| SimulationError::MissingPosition(sphere_name.clone()))?,
                velocity: program
                    .vec3_property("velocity", &sphere_name)
                    .ok_or_else(|| SimulationError::MissingVelocity(sphere_name.clone()))?,
                radius: program
                    .number_property("radius", &sphere_name)
                    .ok_or_else(|| SimulationError::InvalidRadius(sphere_name.clone()))?,
                last_update_time: 0.0,
            });
        }

        let mut planes = Vec::new();
        for plane_decl in plane_decls {
            let plane_name = plane_decl.name.clone();
            planes.push(Plane {
                name: plane_name.clone(),
                normal: program
                    .vec3_property("normal", &plane_name)
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
                    .normalized()?,
                offset: program.number_property("offset", &plane_name).unwrap_or(0.0),
            });
        }

        let mut occluder_regions = Vec::new();
        for region_decl in &region_decls {
            let region_name = region_decl.name.clone();
            occluder_regions.push(Region {
                name: region_name.clone(),
                min: program
                    .vec3_property("min", &region_name)
                    .ok_or_else(|| SimulationError::MissingRegionBounds(region_name.clone()))?,
                max: program
                    .vec3_property("max", &region_name)
                    .ok_or_else(|| SimulationError::MissingRegionBounds(region_name.clone()))?,
            });
        }
        let region = occluder_regions.first().cloned();

        let build_context = ConstraintBuildContext {
            spheres: &spheres,
            planes: &planes,
            regions: &occluder_regions,
            region_name: region.as_ref().map(|decl| decl.name.as_str()),
        };
        let mut constraints = Vec::new();
        for constraint in &program.constraints {
            constraints.push(Constraint::from_parts(constraint, &build_context)?);
        }

        let mut world = Self {
            spheres,
            planes,
            region,
            occluder_regions,
            constraints,
            constraint_traces: vec![ConstraintTrace::default(); program.constraints.len()],
            candidate_resolutions: Vec::new(),
            activity_log: Vec::new(),
            action_candidates_by_entity: action_candidates_by_entity(program),
            deferred_resolution_times: deferred_resolution_times_from_action_directives(
                &program.action_directives,
            ),
            deferred_preference_triggers: deferred_preference_triggers_from_action_directives(
                &program.action_directives,
            ),
            visibility_preference_triggers: visibility_preference_triggers_from_action_directives(
                &program.action_directives,
            ),
            deferred_score_adjustments: deferred_score_adjustments_from_action_directives(
                &program.action_directives,
            ),
            deferred_speed_limit_updates: deferred_speed_limit_updates_from_action_directives(
                &program.action_directives,
            ),
        };

        world.resolve_initial_action_candidates(
            &program.action_candidates,
            &program.action_directives,
        )?;
        world.validated()
    }

    pub fn advance_to(&mut self, target_time: f64) -> Result<(), SimulationError> {
        loop {
            let current_time = self
                .spheres
                .iter()
                .map(|sphere| sphere.last_update_time)
                .fold(0.0, f64::max);
            if target_time <= current_time + EPSILON {
                break;
            }

            let max_dt = target_time - current_time;
            let next_event = self.next_event(max_dt);
            match next_event {
                Some(event) => {
                    self.advance_all_by(event.dt);
                    self.handle_event(event)?;
                }
                None => {
                    self.advance_all_by(max_dt);
                    self.enforce_all_constraints()?;
                    break;
                }
            }
        }

        Ok(())
    }

    fn validated(mut self) -> Result<Self, SimulationError> {
        self.enforce_all_constraints()?;
        Ok(self)
    }

    fn enforce_all_constraints(&mut self) -> Result<(), SimulationError> {
        let constraints = self.constraints.clone();
        for (index, constraint) in constraints.iter().enumerate() {
            match constraint.enforce(self) {
                Ok(repaired) => {
                    if repaired {
                        self.constraint_traces[index].repaired_count += 1;
                        self.activity_log.push(constraint.activity_entry(self, "repaired"));
                    }
                }
                Err(error) => {
                    self.constraint_traces[index].contradicted_count += 1;
                    self.activity_log
                        .push(constraint.activity_entry(self, "contradicted"));
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    fn next_event(&self, max_dt: f64) -> Option<Event> {
        let mut best: Option<Event> = None;
        for (constraint_index, constraint) in self.constraints.iter().enumerate() {
            if let Some(candidate) = constraint.candidate_event(self, constraint_index) {
                best = choose_earlier(best, candidate);
            }
        }

        match best {
            Some(event) if event.dt <= max_dt + EPSILON => Some(event),
            _ => None,
        }
    }

    fn advance_all_by(&mut self, dt: f64) {
        if dt <= EPSILON {
            return;
        }

        for sphere in &mut self.spheres {
            sphere.position = sphere.position + sphere.velocity * dt;
            sphere.last_update_time += dt;
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<(), SimulationError> {
        self.constraint_traces[event.constraint_index].fired_count += 1;
        self.activity_log.push(
            self.constraints[event.constraint_index].activity_entry(self, "fired"),
        );
        event.kind.apply(self)?;
        self.enforce_all_constraints()?;
        Ok(())
    }

    fn apply_elastic_sphere_collision(
        &mut self,
        left_index: usize,
        right_index: usize,
    ) -> Result<(), SimulationError> {
        let left = self.spheres[left_index].clone();
        let right = self.spheres[right_index].clone();
        let collision_normal = (left.position - right.position).normalized()?;
        let relative = left.velocity - right.velocity;
        let impulse = relative.dot(collision_normal);
        if impulse >= 0.0 {
            return Ok(());
        }

        self.spheres[left_index].velocity = left.velocity - collision_normal * impulse;
        self.spheres[right_index].velocity = right.velocity + collision_normal * impulse;
        Ok(())
    }

    fn constraint_summaries(&self) -> Vec<ConstraintSummary> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(index, constraint)| constraint.summary(self, &self.constraint_traces[index]))
            .collect()
    }

    fn current_time(&self) -> f64 {
        self.spheres
            .iter()
            .map(|sphere| sphere.last_update_time)
            .fold(0.0, f64::max)
    }

    fn resolve_initial_action_candidates(
        &mut self,
        action_candidates: &[ActionCandidateDecl],
        action_directives: &[ActionDirectiveDecl],
    ) -> Result<(), SimulationError> {
        if action_candidates.is_empty() {
            return Ok(());
        }

        let deferred_entities = action_directives
            .iter()
            .filter(|directive| directive.kind == "defer_on_ambiguous_top")
            .map(|directive| directive.entity.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let mut grouped = BTreeMap::<String, Vec<ActionCandidateDecl>>::new();
        for candidate in action_candidates {
            grouped
                .entry(candidate.entity.clone())
                .or_default()
                .push(candidate.clone());
        }

        for (sphere_name, candidates) in grouped {
            let resolution = self.resolve_candidates_for_entity(
                &sphere_name,
                &candidates,
                deferred_entities.contains(&sphere_name),
                false,
                None,
                self.visibility_conditioned_preference(&sphere_name).as_deref(),
                &[],
                &[],
            )?;
            self.candidate_resolutions.push(resolution);
        }

        Ok(())
    }

    fn resolve_deferred_candidates_at(
        &mut self,
        observation_time: f64,
    ) -> Result<(), SimulationError> {
        let mut entities = self
            .candidate_resolutions
            .iter()
            .filter(|resolution| {
                resolution.convergence_mode == "deferred"
                    && resolution.selected_candidate.is_none()
                    && self
                        .deferred_resolution_times
                        .get(&resolution.entity)
                        .is_some_and(|time| observation_time + EPSILON >= *time)
            })
            .map(|resolution| resolution.entity.clone())
            .collect::<Vec<_>>();
        entities.sort();

        for entity in entities {
            let candidates = self
                .action_candidates_by_entity
                .get(&entity)
                .cloned()
                .ok_or_else(|| {
                    SimulationError::InvalidActionCandidate(format!(
                        "missing deferred candidate inventory for sphere `{}`",
                        entity
                    ))
                })?;
            let preferred_label = self
                .deferred_preference_triggers
                .get(&entity)
                .filter(|trigger| observation_time + EPSILON >= trigger.time)
                .map(|trigger| trigger.label.clone())
                .or_else(|| self.visibility_conditioned_preference(&entity));
            let score_adjustments = self
                .deferred_score_adjustments
                .get(&entity)
                .map(|adjustments| {
                    adjustments
                        .iter()
                        .filter(|adjustment| observation_time + EPSILON >= adjustment.time)
                        .map(|adjustment| (adjustment.label.clone(), adjustment.delta))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let active_law_updates = self.apply_deferred_law_updates_at(&entity, observation_time);
            let resolution = self.resolve_candidates_for_entity(
                &entity,
                &candidates,
                false,
                true,
                Some(observation_time),
                preferred_label.as_deref(),
                &score_adjustments,
                &active_law_updates,
            )?;
            if let Some(existing) = self
                .candidate_resolutions
                .iter_mut()
                .find(|existing| existing.entity == entity)
            {
                *existing = resolution;
            }
        }

        Ok(())
    }

    fn visibility_conditioned_preference(&self, sphere_name: &str) -> Option<String> {
        self.visibility_preference_triggers
            .get(sphere_name)
            .and_then(|triggers| {
                triggers.iter().find_map(|trigger| {
                    let visible = self.entity_can_see(sphere_name, &trigger.target);
                    let matches = match trigger.condition {
                        VisibilityCondition::Visible => visible,
                        VisibilityCondition::Occluded => !visible,
                    };
                    matches.then(|| trigger.label.clone())
                })
            })
    }

    fn apply_deferred_law_updates_at(
        &mut self,
        entity: &str,
        observation_time: f64,
    ) -> Vec<String> {
        let Some(update) = self.deferred_speed_limit_updates.get(entity).cloned() else {
            return Vec::new();
        };
        if observation_time + EPSILON < update.time {
            return Vec::new();
        }
        let mut activated = Vec::new();
        for constraint in &mut self.constraints {
            if let Constraint::VelocityLimit {
                sphere_index,
                max_speed,
                ..
            } = constraint
            {
                if self.spheres[*sphere_index].name == entity && (*max_speed - update.limit).abs() > EPSILON {
                    *max_speed = update.limit;
                    activated.push(format!("speed_limit:{:.3}", update.limit));
                }
            }
        }
        if !activated.is_empty() {
            self.activity_log.push(ActivityEntry {
                time: observation_time,
                kind: "velocity_limit".to_string(),
                targets: vec![entity.to_string()],
                policy: format!("limit={:.3}", update.limit),
                action: "updated_for_deferred_resolution".to_string(),
            });
        }
        activated
    }

    fn entity_can_see(&self, observer_name: &str, target_name: &str) -> bool {
        if self.occluder_regions.is_empty() {
            return true;
        }
        let Ok(observer_index) = ensure_sphere_exists(&self.spheres, observer_name) else {
            return false;
        };
        let Ok(target_index) = ensure_sphere_exists(&self.spheres, target_name) else {
            return false;
        };

        self.occluding_regions_between(observer_index, target_index).is_empty()
    }

    fn occluding_regions_between(&self, observer_index: usize, target_index: usize) -> Vec<String> {
        let observer = &self.spheres[observer_index];
        let target = &self.spheres[target_index];
        self.occluder_regions
            .iter()
            .filter(|region| {
                line_segment_intersects_box(observer.position, target.position, region.min, region.max)
            })
            .map(|region| region.name.clone())
            .collect()
    }

    fn resolve_candidates_for_entity(
        &mut self,
        sphere_name: &str,
        candidates: &[ActionCandidateDecl],
        allow_defer: bool,
        resolved_from_deferred: bool,
        resolved_at_observation_time: Option<f64>,
        preferred_label: Option<&str>,
        score_adjustments: &[(String, f64)],
        active_law_updates: &[String],
    ) -> Result<CandidateResolution, SimulationError> {
        let sphere_index = ensure_sphere_exists(&self.spheres, sphere_name)?;
        let mut candidates = candidates.to_vec();
        for candidate in &mut candidates {
            for (label, delta) in score_adjustments {
                if candidate.label == *label {
                    candidate.score += *delta;
                }
            }
        }
        candidates.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| {
                    match (
                        preferred_label.is_some_and(|label| left.label == label),
                        preferred_label.is_some_and(|label| right.label == label),
                    ) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                })
                .then_with(|| left.label.cmp(&right.label))
        });
        let total_candidates = candidates.len();
        let top_score = candidates.first().map(|candidate| candidate.score).unwrap_or(0.0);
        let top_labels = candidates
            .iter()
            .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
            .map(|candidate| candidate.label.clone())
            .collect::<Vec<_>>();
        let top_candidate_specs = candidates
            .iter()
            .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
            .cloned()
            .collect::<Vec<_>>();

        let mut selected = false;
        let mut rejected_candidates = 0usize;
        let mut selected_candidate = None;
        let mut selected_score = None;
        let mut selected_score_value = None;
        let mut repaired_after_selection = false;
        let mut selected_signature = None;
        let mut selected_spheres = None;
        let mut selected_activity_log = Vec::new();
        for candidate in candidates {
            let mut probe = self.clone();
            probe.activity_log.clear();
            probe.spheres[sphere_index].velocity = candidate.velocity;

            match probe.enforce_all_constraints() {
                Ok(_) => {
                    let probe_signature = world_signature(&probe);
                    let probe_repaired = probe
                        .activity_log
                        .iter()
                        .any(|activity| activity.action == "repaired");
                    selected_spheres = Some(probe.spheres);
                    selected_activity_log = probe.activity_log;
                    selected_candidate = Some(candidate.label.clone());
                    selected_score = Some(format!("score={:.3}", candidate.score));
                    selected_score_value = Some(candidate.score);
                    selected_signature = Some(probe_signature);
                    repaired_after_selection = probe_repaired;
                    selected = true;
                    break;
                }
                Err(_) => {
                    rejected_candidates += 1;
                    self.activity_log.push(candidate_activity_entry(
                        self.current_time(),
                        sphere_name,
                        &candidate.label,
                        candidate.score,
                        "rejected_by_hard_law",
                    ));
                }
            }
        }

        if !selected {
            return Err(SimulationError::InvalidActionCandidate(format!(
                "all candidate actions for sphere `{}` were rejected by hard laws",
                sphere_name
            )));
        }

        let mut equivalent_top_labels = Vec::new();
        if let Some(selected_signature) = &selected_signature {
            for candidate in &top_candidate_specs {
                let mut probe = self.clone();
                probe.activity_log.clear();
                probe.spheres[sphere_index].velocity = candidate.velocity;
                if probe.enforce_all_constraints().is_ok()
                    && &world_signature(&probe) == selected_signature
                {
                    equivalent_top_labels.push(candidate.label.clone());
                }
            }
            equivalent_top_labels.sort();
        }

        let tie_broken = top_labels.len() > 1;
        let observationally_equivalent_tie = equivalent_top_labels.len() > 1;
        let deferred = allow_defer && tie_broken && !observationally_equivalent_tie;
        let preferred_label = preferred_label.map(ToString::to_string);
        let active_score_adjustments = score_adjustments
            .iter()
            .map(|(label, delta)| format!("{label}:{delta:+.3}"))
            .collect::<Vec<_>>();
        if deferred {
            self.activity_log.push(candidate_activity_entry(
                self.current_time(),
                sphere_name,
                top_labels.first().map(String::as_str).unwrap_or(""),
                top_score,
                "deferred_due_to_tie",
            ));
            selected_candidate = None;
            selected_score = None;
            repaired_after_selection = false;
        } else {
            if let Some(spheres) = selected_spheres {
                self.spheres = spheres;
            }
            if let Some(label) = &selected_candidate {
                self.activity_log.push(candidate_activity_entry(
                    self.current_time(),
                    sphere_name,
                    label,
                    selected_score_value.unwrap_or(top_score),
                    if resolved_from_deferred && !active_law_updates.is_empty() {
                        "selected_after_law_update"
                    } else if resolved_from_deferred && !active_score_adjustments.is_empty() {
                        "selected_after_rescore"
                    } else if resolved_from_deferred && preferred_label.is_some() {
                        "selected_after_preference"
                    } else if resolved_from_deferred {
                        "selected_after_defer"
                    } else {
                        "selected"
                    },
                ));
            }
            self.activity_log.extend(selected_activity_log);
        }

        let convergence_mode = if deferred {
            "deferred"
        } else if resolved_from_deferred && !active_law_updates.is_empty() {
            "resolved_after_law_update"
        } else if resolved_from_deferred && !active_score_adjustments.is_empty() {
            "resolved_after_rescore"
        } else if resolved_from_deferred && preferred_label.is_some() {
            "resolved_after_preference"
        } else if resolved_from_deferred {
            "resolved_after_defer"
        } else if observationally_equivalent_tie {
            "equivalent_tie"
        } else if tie_broken {
            "tie_broken"
        } else if repaired_after_selection {
            "repaired"
        } else if rejected_candidates > 0 {
            "fallback"
        } else {
            "direct"
        };
        let observation_mode = if deferred {
            "ambiguous"
        } else if resolved_from_deferred {
            "determinate"
        } else if observationally_equivalent_tie {
            "representative"
        } else if tie_broken {
            "ambiguous"
        } else {
            "determinate"
        };
        let observation_labels = if resolved_from_deferred {
            selected_candidate
                .as_ref()
                .map(|label| vec![label.clone()])
                .unwrap_or_default()
        } else if observationally_equivalent_tie {
            equivalent_top_labels.clone()
        } else if tie_broken {
            top_labels.clone()
        } else {
            selected_candidate
                .as_ref()
                .map(|label| vec![label.clone()])
                .unwrap_or_default()
        };

        Ok(CandidateResolution {
            entity: sphere_name.to_string(),
            total_candidates,
            rejected_candidates,
            skipped_candidates: if deferred {
                total_candidates.saturating_sub(rejected_candidates)
            } else {
                total_candidates - rejected_candidates - 1
            },
            convergence_mode: convergence_mode.to_string(),
            observation_mode: observation_mode.to_string(),
            observation_labels,
            symbolically_underdetermined: tie_broken && !resolved_from_deferred,
            observationally_underdetermined: tie_broken
                && !observationally_equivalent_tie
                && !resolved_from_deferred,
            selected_candidate,
            selected_score,
            top_score: format!("{top_score:.3}"),
            tie_broken,
            top_labels,
            observationally_equivalent_tie,
            equivalent_top_labels,
            repaired_after_selection,
            observed_while_deferred: 0,
            deferred_past_initial_frontier: false,
            resolved_from_deferred,
            resolved_at_observation_time: resolved_at_observation_time.map(|time| format!("{time:.3}")),
            preferred_label,
            active_score_adjustments,
            active_law_updates: active_law_updates.to_vec(),
        })
    }

}

#[derive(Clone, Copy, Debug)]
struct Event {
    constraint_index: usize,
    dt: f64,
    kind: EventKind,
}

impl Event {
    fn plane(constraint_index: usize, sphere_index: usize, plane_index: usize, dt: f64) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::PlaneCollision {
                sphere_index,
                plane_index,
            },
        }
    }

    fn region(constraint_index: usize, sphere_index: usize, dt: f64) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::ForbiddenRegionEntry { sphere_index },
        }
    }

    fn sphere_pair(
        constraint_index: usize,
        left_index: usize,
        right_index: usize,
        dt: f64,
    ) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::SphereCollision {
                left_index,
                right_index,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum EventKind {
    PlaneCollision {
        sphere_index: usize,
        plane_index: usize,
    },
    ForbiddenRegionEntry { sphere_index: usize },
    SphereCollision { left_index: usize, right_index: usize },
}

impl EventKind {
    fn apply(self, world: &mut World) -> Result<(), SimulationError> {
        match self {
            Self::PlaneCollision {
                sphere_index,
                plane_index,
            } => {
                let normal = world.planes[plane_index].normal;
                let sphere = &mut world.spheres[sphere_index];
                let reflected = sphere.velocity - normal * (2.0 * sphere.velocity.dot(normal));
                sphere.velocity = reflected;
                Ok(())
            }
            Self::ForbiddenRegionEntry { sphere_index } => {
                let _ = sphere_index;
                Ok(())
            }
            Self::SphereCollision {
                left_index,
                right_index,
            } => world.apply_elastic_sphere_collision(left_index, right_index),
        }
    }
}

struct ConstraintBuildContext<'a> {
    spheres: &'a [Sphere],
    planes: &'a [Plane],
    regions: &'a [Region],
    region_name: Option<&'a str>,
}

#[derive(Clone, Copy, Debug)]
enum ConstraintCategory {
    Invariant,
    Boundary,
    Interaction,
}

impl Constraint {
    fn activity_entry(&self, world: &World, action: &str) -> ActivityEntry {
        let summary = self.summary(world, &ConstraintTrace::default());
        ActivityEntry {
            time: world.current_time(),
            kind: summary.kind,
            targets: summary.targets,
            policy: summary.policy,
            action: action.to_string(),
        }
    }

    fn summary(&self, world: &World, trace: &ConstraintTrace) -> ConstraintSummary {
        match self {
            Self::ReflectOnCollision {
                sphere_index,
                plane_index,
            } => ConstraintSummary {
                kind: "reflect_on_collision".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world.planes[*plane_index].name.clone(),
                ],
                policy: "implicit".to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::VelocityLimit {
                sphere_index,
                policy,
                ..
            } => ConstraintSummary {
                kind: "velocity_limit".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![world.spheres[*sphere_index].name.clone()],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::NotInside {
                sphere_index,
                policy,
            } => ConstraintSummary {
                kind: "not_inside".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world
                        .region
                        .as_ref()
                        .map(|region| region.name.clone())
                        .unwrap_or_else(|| "region".to_string()),
                ],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::BetweenPlanes {
                sphere_index,
                lower_plane_index,
                upper_plane_index,
                policy,
            } => ConstraintSummary {
                kind: "between_planes".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world.planes[*lower_plane_index].name.clone(),
                    world.planes[*upper_plane_index].name.clone(),
                ],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::InsidePlanes {
                sphere_index,
                plane_indices,
                policy,
            } => ConstraintSummary {
                kind: "inside_planes".to_string(),
                category: self.category().as_str().to_string(),
                targets: std::iter::once(world.spheres[*sphere_index].name.clone())
                    .chain(
                        plane_indices
                            .iter()
                            .map(|plane_index| world.planes[*plane_index].name.clone()),
                    )
                    .collect(),
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::ThroughGate {
                sphere_index,
                plane_index,
                gate_region_index,
                policy,
            } => ConstraintSummary {
                kind: "through_gate".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world.planes[*plane_index].name.clone(),
                    world.occluder_regions[*gate_region_index].name.clone(),
                ],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::Visible {
                observer_index,
                target_index,
            } => ConstraintSummary {
                kind: "visible".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*observer_index].name.clone(),
                    world.spheres[*target_index].name.clone(),
                ],
                policy: "implicit".to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::ElasticCollision {
                left_index,
                right_index,
            } => ConstraintSummary {
                kind: "elastic_collision".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*left_index].name.clone(),
                    world.spheres[*right_index].name.clone(),
                ],
                policy: "implicit".to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
        }
    }

    fn category(&self) -> ConstraintCategory {
        match self {
            Self::VelocityLimit { .. } => ConstraintCategory::Invariant,
            Self::ReflectOnCollision { .. }
            | Self::NotInside { .. }
            | Self::BetweenPlanes { .. }
            | Self::InsidePlanes { .. }
            | Self::ThroughGate { .. } => ConstraintCategory::Boundary,
            Self::Visible { .. } | Self::ElasticCollision { .. } => ConstraintCategory::Interaction,
        }
    }

    fn supported_policies(&self) -> Vec<RepairPolicy> {
        match self {
            Self::VelocityLimit { .. } => vec![RepairPolicy::Reject, RepairPolicy::Clamp],
            Self::NotInside { .. } => {
                vec![RepairPolicy::Reject, RepairPolicy::Clamp, RepairPolicy::Reflect]
            }
            Self::BetweenPlanes { .. } => vec![RepairPolicy::Reject, RepairPolicy::Clamp],
            Self::InsidePlanes { .. } => vec![RepairPolicy::Reject, RepairPolicy::Clamp],
            Self::ThroughGate { .. } => vec![RepairPolicy::Reject, RepairPolicy::Clamp],
            Self::ReflectOnCollision { .. }
            | Self::Visible { .. }
            | Self::ElasticCollision { .. } => Vec::new(),
        }
    }

    fn from_parts(
        parts: &[String],
        context: &ConstraintBuildContext<'_>,
    ) -> Result<Self, SimulationError> {
        match parts {
            [name, sphere_ref, plane_ref] if name == "reflect_on_collision" => {
                let plane_index = context
                    .planes
                    .iter()
                    .position(|plane| plane.name == *plane_ref)
                    .ok_or_else(|| {
                        SimulationError::InvalidConstraint(format!(
                            "unknown plane in reflect_on_collision: {plane_ref}"
                        ))
                    })?;
                Ok(Self::ReflectOnCollision {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    plane_index,
                })
            }
            [name, sphere_ref, limit] if name == "velocity_limit" => {
                let max_speed = limit.parse::<f64>().map_err(|_| {
                    SimulationError::InvalidConstraint(format!(
                        "invalid velocity limit value: {limit}"
                    ))
                })?;
                Ok(Self::VelocityLimit {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    max_speed,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, limit, policy] if name == "velocity_limit" => {
                let max_speed = limit.parse::<f64>().map_err(|_| {
                    SimulationError::InvalidConstraint(format!(
                        "invalid velocity limit value: {limit}"
                    ))
                })?;
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "velocity_limit",
                    policy,
                    &[RepairPolicy::Reject, RepairPolicy::Clamp],
                )?;
                Ok(Self::VelocityLimit {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    max_speed,
                    policy,
                })
            }
            [name, sphere_ref, region_ref] if name == "not_inside" => {
                let Some(region_name) = context.region_name else {
                    return Err(SimulationError::InvalidConstraint(
                        "not_inside requires a declared region".to_string(),
                    ));
                };
                if region_ref != region_name {
                    return Err(SimulationError::InvalidConstraint(format!(
                        "unknown region in not_inside: {region_ref}"
                    )));
                }
                Ok(Self::NotInside {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, region_ref, policy] if name == "not_inside" => {
                let Some(region_name) = context.region_name else {
                    return Err(SimulationError::InvalidConstraint(
                        "not_inside requires a declared region".to_string(),
                    ));
                };
                if region_ref != region_name {
                    return Err(SimulationError::InvalidConstraint(format!(
                        "unknown region in not_inside: {region_ref}"
                    )));
                }
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "not_inside",
                    policy,
                    &[
                        RepairPolicy::Reject,
                        RepairPolicy::Clamp,
                        RepairPolicy::Reflect,
                    ],
                )?;
                Ok(Self::NotInside {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    policy,
                })
            }
            [name, observer, target] if name == "visible" => Ok(Self::Visible {
                observer_index: ensure_sphere_exists(context.spheres, observer)?,
                target_index: ensure_sphere_exists(context.spheres, target)?,
            }),
            [name, sphere_ref, lower_plane_ref, upper_plane_ref] if name == "between_planes" => {
                let lower_plane_index = ensure_plane_exists(context.planes, lower_plane_ref)?;
                let upper_plane_index = ensure_plane_exists(context.planes, upper_plane_ref)?;
                Ok(Self::BetweenPlanes {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    lower_plane_index,
                    upper_plane_index,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, lower_plane_ref, upper_plane_ref, policy]
                if name == "between_planes" =>
            {
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "between_planes",
                    policy,
                    &[RepairPolicy::Reject, RepairPolicy::Clamp],
                )?;
                Ok(Self::BetweenPlanes {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    lower_plane_index: ensure_plane_exists(context.planes, lower_plane_ref)?,
                    upper_plane_index: ensure_plane_exists(context.planes, upper_plane_ref)?,
                    policy,
                })
            }
            parts if !parts.is_empty() && parts[0] == "inside_planes" => {
                let sphere_ref = parts.get(1).ok_or_else(|| {
                    SimulationError::InvalidConstraint(
                        "inside_planes requires a sphere and at least two planes".to_string(),
                    )
                })?;
                let (plane_refs, policy) = if let Some(last) = parts.last() {
                    if let Ok(policy) = parse_repair_policy(last) {
                        (&parts[2..parts.len() - 1], policy)
                    } else {
                        (&parts[2..], RepairPolicy::Reject)
                    }
                } else {
                    (&parts[2..], RepairPolicy::Reject)
                };
                if plane_refs.len() < 2 {
                    return Err(SimulationError::InvalidConstraint(
                        "inside_planes requires at least two planes".to_string(),
                    ));
                }
                ensure_policy_supported(
                    "inside_planes",
                    policy,
                    &[RepairPolicy::Reject, RepairPolicy::Clamp],
                )?;
                Ok(Self::InsidePlanes {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    plane_indices: plane_refs
                        .iter()
                        .map(|plane_ref| ensure_plane_exists(context.planes, plane_ref))
                        .collect::<Result<Vec<_>, _>>()?,
                    policy,
                })
            }
            [name, sphere_ref, plane_ref, gate_ref] if name == "through_gate" => {
                Ok(Self::ThroughGate {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    plane_index: ensure_plane_exists(context.planes, plane_ref)?,
                    gate_region_index: ensure_region_exists(context.regions, gate_ref)?,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, plane_ref, gate_ref, policy] if name == "through_gate" => {
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "through_gate",
                    policy,
                    &[RepairPolicy::Reject, RepairPolicy::Clamp],
                )?;
                Ok(Self::ThroughGate {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    plane_index: ensure_plane_exists(context.planes, plane_ref)?,
                    gate_region_index: ensure_region_exists(context.regions, gate_ref)?,
                    policy,
                })
            }
            [name, left, right] if name == "elastic_collision" => Ok(Self::ElasticCollision {
                left_index: ensure_sphere_exists(context.spheres, left)?,
                right_index: ensure_sphere_exists(context.spheres, right)?,
            }),
            _ => Err(SimulationError::InvalidConstraint(format!(
                "unsupported constraint: {:?}",
                parts
            ))),
        }
    }

    fn enforce(&self, world: &mut World) -> Result<bool, SimulationError> {
        match self {
            Self::ReflectOnCollision { .. } | Self::ElasticCollision { .. } => Ok(false),
            Self::VelocityLimit {
                sphere_index,
                max_speed,
                policy,
            } => {
                let sphere = &mut world.spheres[*sphere_index];
                let speed = sphere.velocity.magnitude();
                if speed > *max_speed + EPSILON {
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::VelocityLimitExceeded {
                                sphere: sphere.name.clone(),
                                speed,
                                limit: *max_speed,
                            });
                        }
                        RepairPolicy::Clamp => {
                            let direction = sphere.velocity * (1.0 / speed);
                            sphere.velocity = direction * *max_speed;
                            return Ok(true);
                        }
                        RepairPolicy::Reflect => {
                            return Err(SimulationError::InvalidConstraint(
                                "velocity_limit does not support reflect policy".to_string(),
                            ));
                        }
                    }
                }
                Ok(false)
            }
            Self::NotInside {
                sphere_index,
                policy,
            } => {
                let Some(region) = world.region.as_ref() else {
                    return Ok(false);
                };
                let region_min = region.min;
                let region_max = region.max;
                let region_name = region.name.clone();
                let sphere = &mut world.spheres[*sphere_index];
                if point_inside_box(sphere.position, region_min, region_max) {
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::EnteredForbiddenRegion {
                                sphere: sphere.name.clone(),
                                region: region_name,
                                time: sphere.last_update_time,
                            });
                        }
                        RepairPolicy::Clamp => {
                            clamp_sphere_outside_box(sphere, region_min, region_max);
                            return Ok(true);
                        }
                        RepairPolicy::Reflect => {
                            reflect_sphere_outside_box(sphere, region_min, region_max);
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            Self::BetweenPlanes {
                sphere_index,
                lower_plane_index,
                upper_plane_index,
                policy,
            } => {
                let sphere = &mut world.spheres[*sphere_index];
                let mut repaired = false;
                loop {
                    let lower_plane = world.planes[*lower_plane_index].clone();
                    let upper_plane = world.planes[*upper_plane_index].clone();
                    let lower_margin =
                        lower_plane.normal.dot(sphere.position) - lower_plane.offset - sphere.radius;
                    let upper_margin =
                        upper_plane.normal.dot(sphere.position) - upper_plane.offset - sphere.radius;
                    let violating_plane =
                        if lower_margin < -EPSILON && lower_margin <= upper_margin {
                            Some((lower_margin, lower_plane))
                        } else if upper_margin < -EPSILON {
                            Some((upper_margin, upper_plane))
                        } else {
                            None
                        };

                    let Some((margin, plane)) = violating_plane else {
                        break;
                    };
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::LeftPlaneChannel {
                                sphere: sphere.name.clone(),
                                lower_plane: world.planes[*lower_plane_index].name.clone(),
                                upper_plane: world.planes[*upper_plane_index].name.clone(),
                                time: sphere.last_update_time,
                            });
                        }
                        RepairPolicy::Clamp => {
                            clamp_sphere_inside_halfspace(sphere, &plane, margin);
                            repaired = true;
                        }
                        RepairPolicy::Reflect => {
                            return Err(SimulationError::InvalidConstraint(
                                "between_planes does not support reflect policy".to_string(),
                            ));
                        }
                    }
                }
                Ok(repaired)
            }
            Self::InsidePlanes {
                sphere_index,
                plane_indices,
                policy,
            } => {
                let sphere = &mut world.spheres[*sphere_index];
                let mut repaired = false;
                loop {
                    let violating_plane = plane_indices
                        .iter()
                        .map(|plane_index| {
                            let plane = world.planes[*plane_index].clone();
                            let margin =
                                plane.normal.dot(sphere.position) - plane.offset - sphere.radius;
                            (margin, plane)
                        })
                        .filter(|(margin, _)| *margin < -EPSILON)
                        .min_by(|(left_margin, _), (right_margin, _)| {
                            left_margin.total_cmp(right_margin)
                        });

                    let Some((margin, plane)) = violating_plane else {
                        break;
                    };
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::LeftBoundedPlaneSet {
                                sphere: sphere.name.clone(),
                                planes: plane_indices
                                    .iter()
                                    .map(|plane_index| world.planes[*plane_index].name.clone())
                                    .collect(),
                                time: sphere.last_update_time,
                            });
                        }
                        RepairPolicy::Clamp => {
                            clamp_sphere_inside_halfspace(sphere, &plane, margin);
                            repaired = true;
                        }
                        RepairPolicy::Reflect => {
                            return Err(SimulationError::InvalidConstraint(
                                "inside_planes does not support reflect policy".to_string(),
                            ));
                        }
                    }
                }
                Ok(repaired)
            }
            Self::ThroughGate {
                sphere_index,
                plane_index,
                gate_region_index,
                policy,
            } => {
                let plane = world.planes[*plane_index].clone();
                let gate = world.occluder_regions[*gate_region_index].clone();
                let sphere = &mut world.spheres[*sphere_index];
                let margin = plane.normal.dot(sphere.position) - plane.offset - sphere.radius;
                if margin < -EPSILON
                    && !point_inside_gate_aperture(
                        sphere.position,
                        plane.normal,
                        gate.min,
                        gate.max,
                    )
                {
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::MissedGateCrossing {
                                sphere: sphere.name.clone(),
                                plane: plane.name,
                                gate: gate.name,
                                time: sphere.last_update_time,
                            });
                        }
                        RepairPolicy::Clamp => {
                            clamp_sphere_inside_halfspace(sphere, &plane, margin);
                            return Ok(true);
                        }
                        RepairPolicy::Reflect => {
                            return Err(SimulationError::InvalidConstraint(
                                "through_gate does not support reflect policy".to_string(),
                            ));
                        }
                    }
                }
                Ok(false)
            }
            Self::Visible {
                observer_index,
                target_index,
            } => {
                if world.occluder_regions.is_empty() {
                    return Ok(false);
                }
                let blocking_regions = world.occluding_regions_between(*observer_index, *target_index);
                if !blocking_regions.is_empty() {
                    let observer = &world.spheres[*observer_index];
                    let target = &world.spheres[*target_index];
                    return Err(SimulationError::VisibilityOccluded {
                        observer: observer.name.clone(),
                        target: target.name.clone(),
                        region: blocking_regions.join(", "),
                        time: world.current_time(),
                    });
                }
                Ok(false)
            }
        }
    }

    fn candidate_event(&self, world: &World, constraint_index: usize) -> Option<Event> {
        match self {
            Self::ReflectOnCollision {
                sphere_index,
                plane_index,
            } => {
                let sphere = &world.spheres[*sphere_index];
                time_to_plane_collision(sphere, &world.planes[*plane_index]).map(|dt| {
                    Event::plane(constraint_index, *sphere_index, *plane_index, dt)
                })
            }
            Self::VelocityLimit { .. }
            | Self::BetweenPlanes { .. }
            | Self::InsidePlanes { .. }
            | Self::ThroughGate { .. }
            | Self::Visible { .. } => {
                None
            }
            Self::NotInside { sphere_index, .. } => {
                let region = world.region.as_ref()?;
                let sphere = &world.spheres[*sphere_index];
                time_to_box_entry(sphere.position, sphere.velocity, region.min, region.max)
                    .map(|dt| Event::region(constraint_index, *sphere_index, dt))
            }
            Self::ElasticCollision {
                left_index,
                right_index,
            } => time_to_sphere_collision(&world.spheres[*left_index], &world.spheres[*right_index])
                .map(|dt| Event::sphere_pair(constraint_index, *left_index, *right_index, dt)),
        }
    }
}

impl ConstraintCategory {
    fn as_str(self) -> &'static str {
        match self {
            Self::Invariant => "invariant",
            Self::Boundary => "boundary",
            Self::Interaction => "interaction",
        }
    }
}

impl ConstraintTrace {
    fn outcome(&self) -> &'static str {
        if self.contradicted_count > 0 {
            "contradicted"
        } else if self.repaired_count > 0 {
            "repaired"
        } else if self.fired_count > 0 {
            "fired"
        } else {
            "idle"
        }
    }
}

impl LawAnalytics {
    fn from_constraints(constraints: &[ConstraintSummary]) -> Self {
        let mut analytics = Self {
            total_constraints: constraints.len(),
            invariant_constraints: 0,
            boundary_constraints: 0,
            interaction_constraints: 0,
            idle_constraints: 0,
            fired_constraints: 0,
            repaired_constraints: 0,
            contradicted_constraints: 0,
        };

        for constraint in constraints {
            match constraint.category.as_str() {
                "invariant" => analytics.invariant_constraints += 1,
                "boundary" => analytics.boundary_constraints += 1,
                "interaction" => analytics.interaction_constraints += 1,
                _ => {}
            }

            match constraint.outcome.as_str() {
                "idle" => analytics.idle_constraints += 1,
                "fired" => analytics.fired_constraints += 1,
                "repaired" => analytics.repaired_constraints += 1,
                "contradicted" => analytics.contradicted_constraints += 1,
                _ => {}
            }
        }

        analytics
    }
}

impl ConvergenceAnalytics {
    pub fn from_candidate_resolutions(candidate_resolutions: &[CandidateResolution]) -> Self {
        let mut analytics = Self {
            candidate_entities: candidate_resolutions.len(),
            direct_entities: 0,
            fallback_entities: 0,
            repaired_entities: 0,
            deferred_entities: 0,
            preference_resolved_entities: 0,
            rescore_resolved_entities: 0,
            law_updated_entities: 0,
            tie_broken_entities: 0,
            equivalent_tie_entities: 0,
            determinate_entities: 0,
            representative_entities: 0,
            ambiguous_entities: 0,
            symbolically_underdetermined_entities: 0,
            observationally_underdetermined_entities: 0,
            rejected_candidates_total: 0,
            skipped_candidates_total: 0,
        };

        for resolution in candidate_resolutions {
            analytics.rejected_candidates_total += resolution.rejected_candidates;
            analytics.skipped_candidates_total += resolution.skipped_candidates;
            match resolution.convergence_mode.as_str() {
                "direct" => analytics.direct_entities += 1,
                "fallback" => analytics.fallback_entities += 1,
                "repaired" => analytics.repaired_entities += 1,
                "deferred" => analytics.deferred_entities += 1,
                "resolved_after_preference" => analytics.preference_resolved_entities += 1,
                "resolved_after_rescore" => analytics.rescore_resolved_entities += 1,
                "resolved_after_law_update" => analytics.law_updated_entities += 1,
                "tie_broken" => analytics.tie_broken_entities += 1,
                "equivalent_tie" => analytics.equivalent_tie_entities += 1,
                _ => {}
            }
            if resolution.symbolically_underdetermined {
                analytics.symbolically_underdetermined_entities += 1;
            }
            if resolution.observationally_underdetermined {
                analytics.observationally_underdetermined_entities += 1;
            }
            match resolution.observation_mode.as_str() {
                "determinate" => analytics.determinate_entities += 1,
                "representative" => analytics.representative_entities += 1,
                "ambiguous" => analytics.ambiguous_entities += 1,
                _ => {}
            }
        }

        analytics
    }
}

impl ObservationSummary {
    pub fn from_candidate_resolutions(candidate_resolutions: &[CandidateResolution]) -> Self {
        let representative_entities = candidate_resolutions
            .iter()
            .filter(|resolution| resolution.observation_mode == "representative")
            .count();
        let ambiguous_entities = candidate_resolutions
            .iter()
            .filter(|resolution| resolution.observation_mode == "ambiguous")
            .count();

        let status = if ambiguous_entities > 0 {
            "unresolved"
        } else if representative_entities > 0 {
            "representative"
        } else {
            "determinate"
        };

        Self {
            status: status.to_string(),
            representative_entities,
            ambiguous_entities,
        }
    }
}

impl RepairPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Reject => "reject",
            Self::Clamp => "clamp",
            Self::Reflect => "reflect",
        }
    }
}

fn ensure_sphere_exists(spheres: &[Sphere], sphere_name: &str) -> Result<usize, SimulationError> {
    spheres
        .iter()
        .position(|sphere| sphere.name == sphere_name)
        .ok_or_else(|| SimulationError::SphereNotFound(sphere_name.to_string()))
}

fn ensure_plane_exists(planes: &[Plane], plane_name: &str) -> Result<usize, SimulationError> {
    planes
        .iter()
        .position(|plane| plane.name == plane_name)
        .ok_or_else(|| {
            SimulationError::InvalidConstraint(format!(
                "unknown plane in constraint: {plane_name}"
            ))
        })
}

fn ensure_region_exists(regions: &[Region], region_name: &str) -> Result<usize, SimulationError> {
    regions
        .iter()
        .position(|region| region.name == region_name)
        .ok_or_else(|| {
            SimulationError::InvalidConstraint(format!(
                "unknown region in constraint: {region_name}"
            ))
        })
}

fn parse_repair_policy(policy: &str) -> Result<RepairPolicy, SimulationError> {
    match policy {
        "reject" => Ok(RepairPolicy::Reject),
        "clamp" => Ok(RepairPolicy::Clamp),
        "reflect" => Ok(RepairPolicy::Reflect),
        _ => Err(SimulationError::InvalidConstraint(format!(
            "unknown repair policy: {policy}"
        ))),
    }
}

fn ensure_policy_supported(
    constraint_name: &str,
    selected_policy: RepairPolicy,
    supported_policies: &[RepairPolicy],
) -> Result<(), SimulationError> {
    if supported_policies.contains(&selected_policy) {
        return Ok(());
    }

    let supported = supported_policies
        .iter()
        .map(|policy| policy.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Err(SimulationError::InvalidConstraint(format!(
        "{} does not support {}; supported policies: {}",
        constraint_name,
        selected_policy.as_str(),
        supported
    )))
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn choose_earlier(current: Option<Event>, candidate: Event) -> Option<Event> {
    if candidate.dt < 0.0 {
        return current;
    }

    match current {
        Some(existing) if existing.dt <= candidate.dt => Some(existing),
        _ => Some(candidate),
    }
}

fn candidate_activity_entry(
    time: f64,
    sphere_name: &str,
    label: &str,
    score: f64,
    action: &str,
) -> ActivityEntry {
    ActivityEntry {
        time,
        kind: "candidate_velocity".to_string(),
        targets: vec![sphere_name.to_string(), label.to_string()],
        policy: format!("score={score:.3}"),
        action: action.to_string(),
    }
}

fn world_signature(world: &World) -> Vec<String> {
    let mut entries = world
        .spheres
        .iter()
        .map(|sphere| {
            format!(
                "{}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}",
                sphere.name,
                sphere.position.x,
                sphere.position.y,
                sphere.position.z,
                sphere.velocity.x,
                sphere.velocity.y,
                sphere.velocity.z
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

fn candidate_inventory_from_program(program: &Program) -> Vec<CandidateInventorySummary> {
    let deferred_entities = program
        .action_directives
        .iter()
        .filter(|directive| directive.kind == "defer_on_ambiguous_top")
        .map(|directive| directive.entity.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let visibility_preferred_entities = program
        .action_directives
        .iter()
        .filter(|directive| {
            directive.kind == "prefer_candidate_if_visible"
                || directive.kind == "prefer_candidate_if_occluded"
        })
        .filter_map(|directive| {
            directive
                .label_argument
                .as_ref()
                .zip(directive.target_argument.as_ref())
                .map(|(label, target)| {
                    let hint = if directive.kind == "prefer_candidate_if_visible" {
                        format!("prefer_{label}_if_visible_to_{target}")
                    } else {
                        format!("prefer_{label}_if_occluded_to_{target}")
                    };
                    (directive.entity.clone(), hint)
                })
        })
        .fold(BTreeMap::<String, Vec<String>>::new(), |mut acc, (entity, hint)| {
            acc.entry(entity).or_default().push(hint);
            acc
        });
    let preferred_entities = preferred_candidate_labels_from_action_directives(&program.action_directives);
    let rescored_entities = rescore_candidate_labels_from_action_directives(&program.action_directives);
    let law_updated_entities = law_update_labels_from_action_directives(&program.action_directives);
    let mut grouped = BTreeMap::<String, Vec<ActionCandidateDecl>>::new();
    for candidate in &program.action_candidates {
        grouped
            .entry(candidate.entity.clone())
            .or_default()
            .push(candidate.clone());
    }

    grouped
        .into_iter()
        .map(|(entity, mut candidates)| {
            candidates.sort_by(|left, right| left.label.cmp(&right.label));
            let top_score = candidates
                .iter()
                .map(|candidate| candidate.score)
                .max_by(|left, right| left.total_cmp(right))
                .unwrap_or(0.0);
            let mut labels = candidates
                .iter()
                .map(|candidate| candidate.label.clone())
                .collect::<Vec<_>>();
            labels.sort();
            let mut top_labels = candidates
                .iter()
                .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
                .map(|candidate| candidate.label.clone())
                .collect::<Vec<_>>();
            top_labels.sort();
            let top_score_tied = top_labels.len() > 1;
            let defer_on_ambiguous_top = deferred_entities.contains(&entity);
            let resolution_hint = if top_score_tied && defer_on_ambiguous_top {
                if let Some((limit, update_time)) = law_updated_entities.get(&entity) {
                    format!("defer_then_update_speed_limit_to_{limit:.3}_at_{update_time:.3}")
                } else if let Some((rescore_label, delta, rescore_time)) = rescored_entities.get(&entity)
                {
                    format!(
                        "defer_then_rescore_{}_by_{delta:+.3}_at_{rescore_time:.3}",
                        rescore_label
                    )
                } else if let Some((label, time)) = preferred_entities.get(&entity) {
                    format!("defer_then_prefer_{}_at_{time:.3}", label)
                } else {
                    "deferred_on_ambiguous_top".to_string()
                }
            } else if let Some(hints) = visibility_preferred_entities.get(&entity) {
                hints.join("_else_")
            } else if top_score_tied {
                "deterministic_tie_break".to_string()
            } else {
                "single_top_candidate".to_string()
            };

            CandidateInventorySummary {
                entity,
                total_candidates: candidates.len(),
                labels,
                top_score: format!("{top_score:.3}"),
                top_labels,
                top_score_tied,
                defer_on_ambiguous_top,
                resolution_hint,
            }
        })
        .collect()
}

fn candidate_resolutions_for_report(
    candidate_resolutions: &[CandidateResolution],
    observation_count: usize,
) -> Vec<CandidateResolution> {
    candidate_resolutions
        .iter()
        .cloned()
        .map(|mut resolution| {
            if resolution.convergence_mode == "deferred" {
                resolution.observed_while_deferred = observation_count;
                resolution.deferred_past_initial_frontier = observation_count > 1;
            } else if resolution.resolved_from_deferred {
                resolution.observed_while_deferred = observation_count.saturating_sub(1);
                resolution.deferred_past_initial_frontier = false;
            }
            resolution
        })
        .collect()
}

fn observation_checkpoint(time: f64, summary: &ObservationSummary) -> ObservationCheckpoint {
    ObservationCheckpoint {
        time,
        status: summary.status.clone(),
        representative_entities: summary.representative_entities,
        ambiguous_entities: summary.ambiguous_entities,
    }
}

fn action_candidates_by_entity(program: &Program) -> BTreeMap<String, Vec<ActionCandidateDecl>> {
    let mut grouped = BTreeMap::<String, Vec<ActionCandidateDecl>>::new();
    for candidate in &program.action_candidates {
        grouped
            .entry(candidate.entity.clone())
            .or_default()
            .push(candidate.clone());
    }
    grouped
}

fn deferred_resolution_times_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, f64> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "resolve_deferred_at" {
            if let Some(time) = directive.time_argument {
                result.insert(directive.entity.clone(), time);
            }
        }
    }
    result
}

fn deferred_preference_triggers_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, DeferredPreferenceTrigger> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "prefer_candidate_at" {
            if let (Some(label), Some(time)) = (&directive.label_argument, directive.time_argument) {
                result.insert(
                    directive.entity.clone(),
                    DeferredPreferenceTrigger {
                        label: label.clone(),
                        time,
                    },
                );
            }
        }
    }
    result
}

fn visibility_preference_triggers_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, Vec<VisibilityPreferenceTrigger>> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "prefer_candidate_if_visible"
            || directive.kind == "prefer_candidate_if_occluded"
        {
            if let (Some(label), Some(target)) = (&directive.label_argument, &directive.target_argument)
            {
                result
                    .entry(directive.entity.clone())
                    .or_insert_with(Vec::new)
                    .push(VisibilityPreferenceTrigger {
                        label: label.clone(),
                        target: target.clone(),
                        condition: if directive.kind == "prefer_candidate_if_visible" {
                            VisibilityCondition::Visible
                        } else {
                            VisibilityCondition::Occluded
                        },
                    });
            }
        }
    }
    result
}

fn deferred_score_adjustments_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, Vec<DeferredScoreAdjustment>> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "rescore_candidate_at" {
            if let (Some(label), Some(time), Some(delta)) = (
                &directive.label_argument,
                directive.time_argument,
                directive.score_argument,
            ) {
                result
                    .entry(directive.entity.clone())
                    .or_insert_with(Vec::new)
                    .push(DeferredScoreAdjustment {
                        label: label.clone(),
                        time,
                        delta,
                    });
            }
        }
    }
    result
}

fn deferred_speed_limit_updates_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, DeferredSpeedLimitUpdate> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "update_speed_limit_at" {
            if let (Some(time), Some(limit)) = (directive.time_argument, directive.value_argument) {
                result.insert(
                    directive.entity.clone(),
                    DeferredSpeedLimitUpdate { time, limit },
                );
            }
        }
    }
    result
}

fn preferred_candidate_labels_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, (String, f64)> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "prefer_candidate_at" {
            if let (Some(label), Some(time)) = (&directive.label_argument, directive.time_argument) {
                result.insert(directive.entity.clone(), (label.clone(), time));
            }
        } else if directive.kind == "rescore_candidate_at" {
            if let (Some(label), Some(time), Some(delta)) = (
                &directive.label_argument,
                directive.time_argument,
                directive.score_argument,
            ) {
                result.insert(
                    directive.entity.clone(),
                    (format!("{label}+{delta:.3}"), time),
                );
            }
        }
    }
    result
}

fn law_update_labels_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, (f64, f64)> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "update_speed_limit_at" {
            if let (Some(time), Some(limit)) = (directive.time_argument, directive.value_argument) {
                result.insert(directive.entity.clone(), (limit, time));
            }
        }
    }
    result
}

fn rescore_candidate_labels_from_action_directives(
    action_directives: &[ActionDirectiveDecl],
) -> BTreeMap<String, (String, f64, f64)> {
    let mut result = BTreeMap::new();
    for directive in action_directives {
        if directive.kind == "rescore_candidate_at" {
            if let (Some(label), Some(time), Some(delta)) = (
                &directive.label_argument,
                directive.time_argument,
                directive.score_argument,
            ) {
                result.insert(directive.entity.clone(), (label.clone(), delta, time));
            }
        }
    }
    result
}

fn action_directive_inventory_from_program(program: &Program) -> Vec<ActionDirectiveSummary> {
    let mut directives = program
        .action_directives
        .iter()
        .map(|directive| ActionDirectiveSummary {
            entity: directive.entity.clone(),
            kind: directive.kind.clone(),
            argument: if directive.kind == "prefer_candidate_if_visible"
                || directive.kind == "prefer_candidate_if_occluded"
            {
                match (&directive.label_argument, &directive.target_argument) {
                    (Some(label), Some(target)) => Some(format!("{label}, {target}")),
                    _ => None,
                }
            } else {
                match (&directive.label_argument, directive.time_argument) {
                (Some(label), Some(time)) => {
                    if let Some(delta) = directive.score_argument {
                        Some(format!("{label}, {time:.3}, {delta:+.3}"))
                    } else if let Some(limit) = directive.value_argument {
                        Some(format!("{time:.3}, {limit:.3}"))
                    } else {
                        Some(format!("{label}, {time:.3}"))
                    }
                }
                (None, Some(time)) if directive.value_argument.is_some() => {
                    Some(format!("{time:.3}, {:.3}", directive.value_argument.unwrap_or_default()))
                }
                (Some(label), None) => Some(label.clone()),
                (None, Some(time)) => Some(format!("{time:.3}")),
                (None, None) => None,
            }
            },
        })
        .collect::<Vec<_>>();
    directives.sort_by(|left, right| {
        left.entity
            .cmp(&right.entity)
            .then_with(|| left.kind.cmp(&right.kind))
    });
    directives
}

fn time_to_plane_collision(sphere: &Sphere, plane: &Plane) -> Option<f64> {
    let signed_distance = plane.normal.dot(sphere.position) - plane.offset - sphere.radius;
    let approach_speed = plane.normal.dot(sphere.velocity);

    if signed_distance <= EPSILON || approach_speed >= -EPSILON {
        return None;
    }

    let hit_dt = -signed_distance / approach_speed;
    if hit_dt >= 0.0 { Some(hit_dt) } else { None }
}

fn clamp_sphere_inside_halfspace(sphere: &mut Sphere, plane: &Plane, margin: f64) {
    let correction = -margin + EPSILON;
    sphere.position = sphere.position + plane.normal * correction;
    let normal_speed = sphere.velocity.dot(plane.normal);
    if normal_speed < 0.0 {
        sphere.velocity = sphere.velocity - plane.normal * normal_speed;
    }
}

fn time_to_sphere_collision(left: &Sphere, right: &Sphere) -> Option<f64> {
    let delta_position = left.position - right.position;
    let delta_velocity = left.velocity - right.velocity;
    let combined_radius = left.radius + right.radius;

    let a = delta_velocity.dot(delta_velocity);
    if a <= EPSILON {
        return None;
    }

    let b = 2.0 * delta_position.dot(delta_velocity);
    let c = delta_position.dot(delta_position) - combined_radius * combined_radius;

    if c <= EPSILON {
        return None;
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    [t1, t2]
        .into_iter()
        .filter(|dt| *dt >= 0.0)
        .min_by(|a, b| a.total_cmp(b))
}

fn point_inside_box(point: Vec3, min: Vec3, max: Vec3) -> bool {
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

fn point_inside_gate_aperture(point: Vec3, normal: Vec3, min: Vec3, max: Vec3) -> bool {
    let normal = Vec3::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
    if normal.x >= normal.y && normal.x >= normal.z {
        point.y >= min.y && point.y <= max.y && point.z >= min.z && point.z <= max.z
    } else if normal.y >= normal.z {
        point.x >= min.x && point.x <= max.x && point.z >= min.z && point.z <= max.z
    } else {
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

fn time_to_box_entry(position: Vec3, velocity: Vec3, min: Vec3, max: Vec3) -> Option<f64> {
    if point_inside_box(position, min, max) {
        return Some(0.0);
    }

    let mut t_min: f64 = 0.0;
    let mut t_max = f64::INFINITY;

    for (pos, vel, axis_min, axis_max) in [
        (position.x, velocity.x, min.x, max.x),
        (position.y, velocity.y, min.y, max.y),
        (position.z, velocity.z, min.z, max.z),
    ] {
        if vel.abs() <= EPSILON {
            if pos < axis_min || pos > axis_max {
                return None;
            }
            continue;
        }

        let t1 = (axis_min - pos) / vel;
        let t2 = (axis_max - pos) / vel;
        let axis_entry = t1.min(t2);
        let axis_exit = t1.max(t2);
        t_min = t_min.max(axis_entry);
        t_max = t_max.min(axis_exit);
        if t_min > t_max {
            return None;
        }
    }

    if t_max < 0.0 {
        None
    } else if t_min <= 0.0 {
        Some(0.0)
    } else {
        Some(t_min)
    }
}

fn line_segment_intersects_box(start: Vec3, end: Vec3, min: Vec3, max: Vec3) -> bool {
    let delta = end - start;
    let mut t_min: f64 = 0.0;
    let mut t_max: f64 = 1.0;

    for (origin, direction, axis_min, axis_max) in [
        (start.x, delta.x, min.x, max.x),
        (start.y, delta.y, min.y, max.y),
        (start.z, delta.z, min.z, max.z),
    ] {
        if direction.abs() <= EPSILON {
            if origin < axis_min || origin > axis_max {
                return false;
            }
            continue;
        }

        let t1 = (axis_min - origin) / direction;
        let t2 = (axis_max - origin) / direction;
        let axis_entry = t1.min(t2);
        let axis_exit = t1.max(t2);
        t_min = t_min.max(axis_entry);
        t_max = t_max.min(axis_exit);
        if t_min > t_max {
            return false;
        }
    }

    t_max >= 0.0 && t_min <= 1.0
}

fn clamp_sphere_outside_box(sphere: &mut Sphere, min: Vec3, max: Vec3) {
    let distances = [
        (sphere.position.x - min.x, 0usize, min.x - EPSILON),
        (max.x - sphere.position.x, 0usize, max.x + EPSILON),
        (sphere.position.y - min.y, 1usize, min.y - EPSILON),
        (max.y - sphere.position.y, 1usize, max.y + EPSILON),
        (sphere.position.z - min.z, 2usize, min.z - EPSILON),
        (max.z - sphere.position.z, 2usize, max.z + EPSILON),
    ];

    let (_, axis, target) = distances
        .into_iter()
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .expect("box face distances are non-empty");

    match axis {
        0 => {
            sphere.position.x = target;
            sphere.velocity.x = 0.0;
        }
        1 => {
            sphere.position.y = target;
            sphere.velocity.y = 0.0;
        }
        2 => {
            sphere.position.z = target;
            sphere.velocity.z = 0.0;
        }
        _ => unreachable!("axis index is bounded above"),
    }
}

fn reflect_sphere_outside_box(sphere: &mut Sphere, min: Vec3, max: Vec3) {
    let distances = [
        (sphere.position.x - min.x, 0usize, min.x - EPSILON, -1.0),
        (max.x - sphere.position.x, 0usize, max.x + EPSILON, 1.0),
        (sphere.position.y - min.y, 1usize, min.y - EPSILON, -1.0),
        (max.y - sphere.position.y, 1usize, max.y + EPSILON, 1.0),
        (sphere.position.z - min.z, 2usize, min.z - EPSILON, -1.0),
        (max.z - sphere.position.z, 2usize, max.z + EPSILON, 1.0),
    ];

    let (_, axis, target, direction) = distances
        .into_iter()
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .expect("box face distances are non-empty");

    match axis {
        0 => {
            sphere.position.x = target;
            sphere.velocity.x = sphere.velocity.x.abs() * direction;
        }
        1 => {
            sphere.position.y = target;
            sphere.velocity.y = sphere.velocity.y.abs() * direction;
        }
        2 => {
            sphere.position.z = target;
            sphere.velocity.z = sphere.velocity.z.abs() * direction;
        }
        _ => unreachable!("axis index is bounded above"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        SimulationEnvelope, Vec3, analyze_program, parse_program, simulate_program,
        simulate_program_envelope,
    };

    #[test]
    fn bounce_reflects_on_floor() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (1, -3, 0)
radius(A) = 1
constraint:
    reflect_on_collision(A, floor)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots[0].spheres[0].position.y, 1.0);
        assert_eq!(report.snapshots[0].spheres[0].velocity.y, 3.0);
    }

    #[test]
    fn bounce_can_reflect_between_floor_and_ceiling() {
        let source = r#"
sphere A
plane floor
plane ceiling
position(A) = (0, 2, 0)
velocity(A) = (1, 2, 0)
radius(A) = 1
normal(floor) = (0, 1, 0)
offset(floor) = 0
normal(ceiling) = (0, -1, 0)
offset(ceiling) = -4
constraint:
    reflect_on_collision(A, floor)
    reflect_on_collision(A, ceiling)
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots.len(), 4);
        assert_eq!(report.snapshots[1].spheres[0].position.y, 2.0);
        assert_eq!(report.snapshots[1].spheres[0].velocity.y, -2.0);
        assert_eq!(report.snapshots[2].spheres[0].position.y, 2.0);
        assert_eq!(report.snapshots[2].spheres[0].velocity.y, 2.0);
        assert_eq!(report.snapshots[3].spheres[0].position.y, 2.0);
        assert_eq!(report.snapshots[3].spheres[0].velocity.y, -2.0);
        let reflect_targets = report
            .constraints
            .iter()
            .filter(|constraint| constraint.kind == "reflect_on_collision")
            .map(|constraint| constraint.targets[1].clone())
            .collect::<Vec<_>>();
        assert_eq!(reflect_targets, vec!["floor".to_string(), "ceiling".to_string()]);
    }

    #[test]
    fn plane_bounded_space_can_clamp_inside_a_surface_wedge() {
        let source = r#"
sphere A
plane floor
plane ceiling
position(A) = (0, 2, 0)
velocity(A) = (2, 1, 0)
radius(A) = 1
normal(floor) = (0, 1, 0)
offset(floor) = 0
normal(ceiling) = (-0.5, -1, 0)
offset(ceiling) = -4
constraint:
    between_planes(A, floor, ceiling, clamp)
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.constraints.len(), 1);
        assert_eq!(report.constraints[0].kind, "between_planes");
        assert_eq!(report.constraints[0].targets, vec!["A", "floor", "ceiling"]);
        assert_eq!(report.constraints[0].policy, "clamp");
        assert_eq!(report.constraints[0].outcome, "repaired");
        let final_sphere = &report.snapshots[2].spheres[0];
        assert!(final_sphere.position.y < 2.1);
        assert!(final_sphere.velocity.y <= 0.2);
    }

    #[test]
    fn bounded_plane_set_can_clamp_inside_a_surface_room() {
        let source = r#"
sphere A
plane floor
plane ceiling
plane left_wall
plane right_wall
position(A) = (1.2, 2.6, 0)
velocity(A) = (-1.2, 1.0, 0)
radius(A) = 0.8
normal(floor) = (0, 1, 0)
offset(floor) = 0
normal(ceiling) = (-0.5, -1, 0)
offset(ceiling) = -4
normal(left_wall) = (1, 0, 0)
offset(left_wall) = 0
normal(right_wall) = (-1, 0, 0)
offset(right_wall) = -5
constraint:
    inside_planes(A, floor, ceiling, left_wall, right_wall, clamp)
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.constraints.len(), 1);
        assert_eq!(report.constraints[0].kind, "inside_planes");
        assert_eq!(
            report.constraints[0].targets,
            vec!["A", "floor", "ceiling", "left_wall", "right_wall"]
        );
        assert_eq!(report.constraints[0].policy, "clamp");
        assert_eq!(report.constraints[0].outcome, "repaired");
        let final_sphere = &report.snapshots[2].spheres[0];
        assert!(final_sphere.position.x >= 0.79);
        assert!(final_sphere.position.x <= 4.21);
        assert!(final_sphere.position.y >= 0.79);
        let ceiling_margin = Vec3::new(-0.5, -1.0, 0.0)
            .normalized()
            .expect("plane normal should normalize")
            .dot(final_sphere.position)
            - (-4.0)
            - 0.8;
        assert!(ceiling_margin >= -0.001);
    }

    #[test]
    fn bounce_can_reflect_inside_a_surface_room() {
        let source = r#"
sphere A
plane floor
plane ceiling
plane left_wall
plane right_wall
position(A) = (2.0, 2.0, 0)
velocity(A) = (1.5, 1.0, 0)
radius(A) = 0.5
normal(floor) = (0, 1, 0)
offset(floor) = 0
normal(ceiling) = (0, -1, 0)
offset(ceiling) = -4
normal(left_wall) = (1, 0, 0)
offset(left_wall) = 0
normal(right_wall) = (-1, 0, 0)
offset(right_wall) = -5
constraint:
    reflect_on_collision(A, floor)
    reflect_on_collision(A, ceiling)
    reflect_on_collision(A, left_wall)
    reflect_on_collision(A, right_wall)
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots.len(), 4);
        let final_sphere = &report.snapshots[3].spheres[0];
        assert!(final_sphere.position.x >= 0.5);
        assert!(final_sphere.position.x <= 4.5);
        assert!(final_sphere.position.y >= 0.5);
        assert!(final_sphere.position.y <= 3.5);
        let reflect_targets = report
            .constraints
            .iter()
            .filter(|constraint| constraint.kind == "reflect_on_collision")
            .map(|constraint| constraint.targets[1].clone())
            .collect::<Vec<_>>();
        assert_eq!(
            reflect_targets,
            vec![
                "floor".to_string(),
                "ceiling".to_string(),
                "left_wall".to_string(),
                "right_wall".to_string()
            ]
        );
    }

    #[test]
    fn gate_constraint_allows_crossing_through_a_door_aperture() {
        let source = r#"
sphere A
plane wall
region door
position(A) = (2, 2, 0)
velocity(A) = (-2, 0, 0)
radius(A) = 0.5
normal(wall) = (1, 0, 0)
offset(wall) = 0
min(door) = (-0.5, 1, -1)
max(door) = (0.5, 3, 1)
constraint:
    through_gate(A, wall, door)
observe:
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.constraints.len(), 1);
        assert_eq!(report.constraints[0].kind, "through_gate");
        assert_eq!(report.constraints[0].targets, vec!["A", "wall", "door"]);
        assert_eq!(report.constraints[0].policy, "reject");
        assert_eq!(report.constraints[0].outcome, "idle");
        assert!(report.snapshots[0].spheres[0].position.x <= 0.0);
    }

    #[test]
    fn gate_constraint_rejects_crossing_outside_the_door_aperture() {
        let source = r#"
sphere A
plane wall
region door
position(A) = (2, 4, 0)
velocity(A) = (-2, 0, 0)
radius(A) = 0.5
normal(wall) = (1, 0, 0)
offset(wall) = 0
min(door) = (-0.5, 1, -1)
max(door) = (0.5, 3, 1)
constraint:
    through_gate(A, wall, door)
observe:
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("outside gate `door`"));
    }

    #[test]
    fn gate_constraint_can_clamp_back_to_the_allowed_side() {
        let source = r#"
sphere A
plane wall
region door
position(A) = (2, 4, 0)
velocity(A) = (-2, 0, 0)
radius(A) = 0.5
normal(wall) = (1, 0, 0)
offset(wall) = 0
min(door) = (-0.5, 1, -1)
max(door) = (0.5, 3, 1)
constraint:
    through_gate(A, wall, door, clamp)
observe:
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.constraints[0].policy, "clamp");
        assert_eq!(report.constraints[0].outcome, "repaired");
        assert!(report.snapshots[0].spheres[0].position.x >= 0.5 - 0.001);
    }

    #[test]
    fn elastic_collision_swaps_velocities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
position(B) = (4, 2, 0)
velocity(B) = (-1, 0, 0)
radius(B) = 1
constraint:
    elastic_collision(A, B)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let snapshot = &report.snapshots[0];
        let a = snapshot
            .spheres
            .iter()
            .find(|sphere| sphere.name == "A")
            .expect("sphere A exists");
        let b = snapshot
            .spheres
            .iter()
            .find(|sphere| sphere.name == "B")
            .expect("sphere B exists");
        assert_eq!(a.velocity.x, -1.0);
        assert_eq!(b.velocity.x, 1.0);
    }

    #[test]
    fn velocity_limit_rejects_fast_sphere() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (3, 4, 0)
radius(A) = 1
constraint:
    velocity_limit(A, 4)
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("velocity limit"));
    }

    #[test]
    fn velocity_limit_can_clamp_speed() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (6, 8, 0)
radius(A) = 1
constraint:
    clamp speed(A) <= 5
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!((sphere.velocity.magnitude() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn velocity_limit_rejects_unsupported_reflect_policy_at_build_time() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (6, 8, 0)
radius(A) = 1
constraint:
    reflect speed(A) <= 5
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error
            .to_string()
            .contains("velocity_limit does not support reflect"));
    }

    #[test]
    fn forbidden_region_stops_world() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("forbidden region"));
    }

    #[test]
    fn forbidden_region_can_clamp_to_boundary() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    clamp not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!(sphere.position.x < 2.0 || sphere.position.x > 4.0);
        assert_eq!(sphere.velocity.x, 0.0);
    }

    #[test]
    fn forbidden_region_can_reflect_from_boundary() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    reflect not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!(sphere.position.x < 2.0 || sphere.position.x > 4.0);
        assert_eq!(sphere.velocity.x, -1.0);
    }

    #[test]
    fn visibility_law_allows_clear_line_of_sight() {
        let source = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (0, 4, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (2, 1, -1)
max(wall) = (3, 3, 1)
constraint:
    visible(A, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let summary = report
            .constraints
            .iter()
            .find(|constraint| constraint.kind == "visible")
            .expect("visible summary should exist");
        assert_eq!(summary.category, "interaction");
        assert_eq!(summary.outcome, "idle");
    }

    #[test]
    fn visibility_law_reports_occlusion_as_contradiction() {
        let source = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (1, -1, -1)
max(wall) = (3, 1, 1)
constraint:
    visible(A, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("cannot see"));
    }

    #[test]
    fn visibility_law_allows_multiple_clear_occluders() {
        let source = r#"
sphere A
sphere B
plane floor
region wall_left
region wall_right
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (0, 4, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall_left) = (2, 0, -1)
max(wall_left) = (3, 1, 1)
min(wall_right) = (-3, 2.5, -1)
max(wall_right) = (-2, 3.5, 1)
constraint:
    visible(A, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let summary = report
            .constraints
            .iter()
            .find(|constraint| constraint.kind == "visible")
            .expect("visible summary should exist");
        assert_eq!(summary.outcome, "idle");
    }

    #[test]
    fn visibility_law_reports_first_multi_occluder_hit_set() {
        let source = r#"
sphere A
sphere B
plane floor
region wall_left
region wall_right
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall_left) = (0.5, -1, -1)
max(wall_left) = (1.5, 1, 1)
min(wall_right) = (2.5, -1, -1)
max(wall_right) = (3.5, 1, 1)
constraint:
    visible(A, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        let message = error.to_string();
        assert!(message.contains("wall_left"));
        assert!(message.contains("wall_right"));
    }

    #[test]
    fn report_serializes_to_json() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 1, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
constraint:
    speed(A) <= 1
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let json = report.to_json("example.sk");
        assert!(json.contains("\"source\": \"example.sk\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"total_constraints\": 1"));
        assert!(json.contains("\"constraints\""));
        assert!(json.contains("\"velocity_limit\""));
        assert!(json.contains("\"category\": \"invariant\""));
        assert!(json.contains("\"supported_policies\": [\"reject\", \"clamp\"]"));
        assert!(json.contains("\"outcome\": \"idle\""));
        assert!(json.contains("\"fired_count\""));
        assert!(json.contains("\"repaired_count\""));
        assert!(json.contains("\"convergence_analytics\""));
        assert!(json.contains("\"determinate_entities\""));
        assert!(json.contains("\"observation_summary\""));
        assert!(json.contains("\"candidate_resolutions\": ["));
        assert!(json.contains("\"activities\""));
        assert!(json.contains("\"snapshots\""));
        assert!(json.contains("\"name\": \"A\""));
    }

    #[test]
    fn analyze_program_summarizes_declared_laws_without_simulating() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    reflect not inside(A, zone)
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("analyze.sk");
        assert!(json.contains("\"source\": \"analyze.sk\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"total_constraints\": 1"));
        assert!(json.contains("\"policy\": \"reflect\""));
        assert!(json.contains("\"outcome\": \"idle\""));
        assert!(json.contains("\"candidate_inventory\": ["));
        assert!(!json.contains("\"entity\":"));
        assert!(!json.contains("\"snapshots\""));
    }

    #[test]
    fn analyze_program_reports_candidate_inventory() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("candidate.sk");
        assert!(json.contains("\"candidate_inventory\""));
        assert!(json.contains("\"entity\": \"A\""));
        assert!(json.contains("\"total_candidates\": 2"));
        assert!(json.contains("\"labels\": [\"fast\", \"safe\"]"));
        assert!(json.contains("\"top_score\": \"5.000\""));
        assert!(json.contains("\"top_labels\": [\"fast\"]"));
        assert!(json.contains("\"top_score_tied\": false"));
        assert!(json.contains("\"defer_on_ambiguous_top\": false"));
        assert!(json.contains("\"resolution_hint\": \"single_top_candidate\""));
    }

    #[test]
    fn analyze_program_reports_action_directives() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_at(A, beta, 1)
constraint:
    speed(A) <= 4
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("deferred.sk");
        assert!(json.contains("\"action_directive_inventory\""));
        assert!(json.contains("\"entity\": \"A\""));
        assert!(json.contains("\"kind\": \"defer_on_ambiguous_top\""));
        assert!(json.contains("\"kind\": \"resolve_deferred_at\""));
        assert!(json.contains("\"kind\": \"prefer_candidate_at\""));
        assert!(json.contains("\"argument\": \"1.000\""));
        assert!(json.contains("\"argument\": \"beta, 1.000\""));
        assert!(json.contains("\"top_score_tied\": true"));
        assert!(json.contains("\"defer_on_ambiguous_top\": true"));
        assert!(json.contains("\"resolution_hint\": \"defer_then_prefer_beta_at_1.000\""));
    }

    #[test]
    fn envelope_serializes_error_json() {
        let envelope = SimulationEnvelope::failure("bad.sk", "world contradiction");
        let json = envelope.to_json();
        assert!(json.contains("\"status\": \"error\""));
        assert!(json.contains("\"error\": \"world contradiction\""));
        assert!(json.contains("\"analytics\": {"));
        assert!(json.contains("\"constraints\": []"));
        assert!(json.contains("\"convergence_analytics\": {"));
        assert!(json.contains("\"observation_summary\": {"));
        assert!(json.contains("\"activities\": []"));
        assert!(json.contains("\"snapshots\": []"));
    }

    #[test]
    fn failure_envelope_can_keep_partial_report() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    not inside(A, zone)
observe:
    snapshot at 1
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let envelope = simulate_program_envelope(&program, "forbidden_region.sk");
        let json = envelope.to_json();
        assert!(json.contains("\"status\": \"error\""));
        assert!(json.contains("\"constraints\""));
        assert!(json.contains("\"not_inside\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"boundary_constraints\": 1"));
        assert!(json.contains("\"outcome\": \"contradicted\""));
        assert!(json.contains("\"contradicted_count\": 1"));
        assert!(json.contains("\"action\": \"contradicted\""));
        assert!(json.contains("\"activities\""));
        assert!(json.contains("\"time\": 1.000000"));
    }

    #[test]
    fn candidate_velocity_selects_best_admissible_option() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 3.0);
        assert!(report
            .activities
            .iter()
            .any(|entry| entry.kind == "candidate_velocity" && entry.action == "selected"));
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "rejected_by_hard_law"
                && entry.targets.iter().any(|target| target == "fast")
        }));
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.entity, "A");
        assert_eq!(candidate_resolution.total_candidates, 2);
        assert_eq!(candidate_resolution.rejected_candidates, 1);
        assert_eq!(candidate_resolution.skipped_candidates, 0);
        assert_eq!(candidate_resolution.convergence_mode, "fallback");
        assert_eq!(candidate_resolution.observation_mode, "determinate");
        assert_eq!(candidate_resolution.observation_labels, vec!["safe".to_string()]);
        assert!(!candidate_resolution.symbolically_underdetermined);
        assert!(!candidate_resolution.observationally_underdetermined);
        assert_eq!(
            candidate_resolution.selected_candidate.as_deref(),
            Some("safe")
        );
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(candidate_resolution.top_labels, vec!["fast".to_string()]);
        assert!(!candidate_resolution.tie_broken);
        assert!(candidate_resolution.equivalent_top_labels.is_empty());
        assert!(!candidate_resolution.observationally_equivalent_tie);
    }

    #[test]
    fn candidate_velocity_can_select_repaired_option() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    clamp speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 4.0);
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "selected"
                && entry.targets.iter().any(|target| target == "fast")
        }));
        assert!(report
            .activities
            .iter()
            .any(|entry| entry.kind == "velocity_limit" && entry.action == "repaired"));
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(
            candidate_resolution.selected_candidate.as_deref(),
            Some("fast")
        );
        assert_eq!(candidate_resolution.convergence_mode, "repaired");
        assert_eq!(candidate_resolution.observation_mode, "determinate");
        assert_eq!(candidate_resolution.observation_labels, vec!["fast".to_string()]);
        assert_eq!(candidate_resolution.skipped_candidates, 1);
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(candidate_resolution.top_labels, vec!["fast".to_string()]);
        assert!(!candidate_resolution.tie_broken);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["fast".to_string()]
        );
        assert!(!candidate_resolution.observationally_equivalent_tie);
        assert!(candidate_resolution.repaired_after_selection);
    }

    #[test]
    fn candidate_velocity_can_resolve_multiple_entities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (5, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
    candidate_velocity(B, sprint) = (5, 0, 0) score 4
    candidate_velocity(B, coast) = (2, 0, 0) score 1
constraint:
    speed(A) <= 4
    speed(B) <= 3
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .snapshots[0]
            .spheres
            .iter()
            .find(|sphere| sphere.name == "A")
            .expect("sphere A exists");
        let b = report
            .snapshots[0]
            .spheres
            .iter()
            .find(|sphere| sphere.name == "B")
            .expect("sphere B exists");
        assert_eq!(a.velocity.x, 3.0);
        assert_eq!(b.velocity.x, 2.0);
        assert_eq!(report.candidate_resolutions.len(), 2);
        assert!(report
            .candidate_resolutions
            .iter()
            .any(|resolution| resolution.entity == "A"));
        assert!(report
            .candidate_resolutions
            .iter()
            .any(|resolution| resolution.entity == "B"));
    }

    #[test]
    fn candidate_velocity_reports_top_score_ties() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.selected_candidate.as_deref(), Some("alpha"));
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(
            candidate_resolution.top_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert_eq!(candidate_resolution.skipped_candidates, 1);
        assert_eq!(candidate_resolution.convergence_mode, "tie_broken");
        assert_eq!(candidate_resolution.observation_mode, "ambiguous");
        assert_eq!(
            candidate_resolution.observation_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert!(candidate_resolution.symbolically_underdetermined);
        assert!(candidate_resolution.observationally_underdetermined);
        assert!(candidate_resolution.tie_broken);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["alpha".to_string()]
        );
        assert!(!candidate_resolution.observationally_equivalent_tie);
    }

    #[test]
    fn candidate_velocity_can_report_observationally_equivalent_ties() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (3, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert!(candidate_resolution.tie_broken);
        assert_eq!(candidate_resolution.convergence_mode, "equivalent_tie");
        assert_eq!(candidate_resolution.observation_mode, "representative");
        assert_eq!(
            candidate_resolution.observation_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert!(candidate_resolution.symbolically_underdetermined);
        assert!(!candidate_resolution.observationally_underdetermined);
        assert!(candidate_resolution.observationally_equivalent_tie);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
    }

    #[test]
    fn candidate_velocity_reports_convergence_analytics() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (3, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.convergence_analytics.candidate_entities, 1);
        assert_eq!(report.convergence_analytics.equivalent_tie_entities, 1);
        assert_eq!(report.convergence_analytics.direct_entities, 0);
        assert_eq!(report.convergence_analytics.tie_broken_entities, 0);
        assert_eq!(report.convergence_analytics.determinate_entities, 0);
        assert_eq!(report.convergence_analytics.representative_entities, 1);
        assert_eq!(report.convergence_analytics.ambiguous_entities, 0);
        assert_eq!(report.observation_summary.status, "representative");
        assert_eq!(report.observation_summary.representative_entities, 1);
        assert_eq!(report.observation_summary.ambiguous_entities, 0);
        assert_eq!(
            report.convergence_analytics.symbolically_underdetermined_entities,
            1
        );
        assert_eq!(
            report
                .convergence_analytics
                .observationally_underdetermined_entities,
            0
        );
    }

    #[test]
    fn candidate_velocity_can_surface_unresolved_observation() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.observation_summary.status, "unresolved");
        assert_eq!(report.observation_summary.representative_entities, 0);
        assert_eq!(report.observation_summary.ambiguous_entities, 1);
    }

    #[test]
    fn candidate_velocity_can_defer_ambiguous_top_choice() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 0.0);
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.convergence_mode, "deferred");
        assert_eq!(candidate_resolution.observation_mode, "ambiguous");
        assert_eq!(candidate_resolution.selected_candidate, None);
        assert_eq!(candidate_resolution.skipped_candidates, 2);
        assert_eq!(report.convergence_analytics.deferred_entities, 1);
        assert_eq!(report.observation_summary.status, "unresolved");
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity" && entry.action == "deferred_due_to_tie"
        }));
    }

    #[test]
    fn candidate_velocity_can_mix_deferred_and_repaired_entities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    candidate_velocity(B, sprint) = (6, 0, 0) score 6
    candidate_velocity(B, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
    clamp speed(B) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A should have candidate resolution");
        let b = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "B")
            .expect("B should have candidate resolution");
        assert_eq!(a.convergence_mode, "deferred");
        assert_eq!(a.observation_mode, "ambiguous");
        assert_eq!(a.selected_candidate, None);
        assert_eq!(b.convergence_mode, "repaired");
        assert_eq!(b.observation_mode, "determinate");
        assert_eq!(b.selected_candidate.as_deref(), Some("sprint"));
        assert!(b.repaired_after_selection);
        assert_eq!(report.convergence_analytics.candidate_entities, 2);
        assert_eq!(report.convergence_analytics.deferred_entities, 1);
        assert_eq!(report.convergence_analytics.repaired_entities, 1);
        assert_eq!(report.observation_summary.status, "unresolved");
        assert_eq!(report.observation_summary.ambiguous_entities, 1);
        assert_eq!(report.observation_summary.representative_entities, 0);
    }

    #[test]
    fn candidate_velocity_can_keep_deferred_entity_across_observations() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    candidate_velocity(B, sprint) = (6, 0, 0) score 6
    candidate_velocity(B, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
    clamp speed(B) <= 4
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots.len(), 2);
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A should have candidate resolution");
        let b = report.snapshots[1]
            .spheres
            .iter()
            .find(|sphere| sphere.name == "B")
            .expect("B should exist");
        assert_eq!(a.convergence_mode, "deferred");
        assert_eq!(a.observed_while_deferred, 2);
        assert!(a.deferred_past_initial_frontier);
        assert_eq!(b.position.x, 8.0);
        assert_eq!(report.observation_summary.status, "unresolved");
    }

    #[test]
    fn candidate_velocity_can_resolve_after_defer_at_later_frontier() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A should have candidate resolution");
        assert_eq!(report.snapshots.len(), 2);
        assert_eq!(resolution.convergence_mode, "resolved_after_defer");
        assert_eq!(resolution.observation_mode, "determinate");
        assert_eq!(resolution.observation_labels, vec!["alpha".to_string()]);
        assert_eq!(resolution.selected_candidate.as_deref(), Some("alpha"));
        assert_eq!(resolution.observed_while_deferred, 1);
        assert!(!resolution.deferred_past_initial_frontier);
        assert!(resolution.resolved_from_deferred);
        assert_eq!(
            resolution.resolved_at_observation_time.as_deref(),
            Some("1.000")
        );
        assert_eq!(report.observation_summary.status, "determinate");
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity" && entry.action == "selected_after_defer"
        }));
    }

    #[test]
    fn candidate_velocity_can_resolve_after_defer_with_later_preference() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (2, 0, 0) score 5
    candidate_velocity(A, beta) = (-2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_at(A, beta, 1)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_preference");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("beta"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("beta"));
        assert_eq!(resolution.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "selected_after_preference"
                && entry.targets.iter().any(|target| target == "beta")
        }));
    }

    #[test]
    fn candidate_velocity_can_resolve_after_defer_with_later_rescore() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (2, 0, 0) score 5
    candidate_velocity(A, beta) = (-2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    rescore_candidate_at(A, beta, 1, 1)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_rescore");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("beta"));
        assert_eq!(resolution.selected_score.as_deref(), Some("score=6.000"));
        assert_eq!(
            resolution.active_score_adjustments,
            vec!["beta:+1.000".to_string()]
        );
        assert_eq!(report.convergence_analytics.rescore_resolved_entities, 1);
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "selected_after_rescore"
                && entry.targets.iter().any(|target| target == "beta")
        }));
    }

    #[test]
    fn candidate_velocity_can_resolve_after_defer_with_later_law_update() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (5, 0, 0) score 5
    candidate_velocity(A, beta) = (3, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    update_speed_limit_at(A, 1, 6)
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_law_update");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("alpha"));
        assert_eq!(resolution.active_law_updates, vec!["speed_limit:6.000".to_string()]);
        assert_eq!(report.convergence_analytics.law_updated_entities, 1);
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "velocity_limit"
                && entry.action == "updated_for_deferred_resolution"
        }));
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "selected_after_law_update"
                && entry.targets.iter().any(|target| target == "alpha")
        }));
    }

    #[test]
    fn candidate_velocity_can_stagger_resolution_across_entities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    candidate_velocity(B, gamma) = (1, 0, 0) score 5
    candidate_velocity(B, delta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(B)
    resolve_deferred_at(B, 2)
constraint:
    speed(A) <= 4
    speed(B) <= 4
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots.len(), 3);
        assert_eq!(report.observation_timeline.len(), 3);
        assert_eq!(report.observation_timeline[0].status, "unresolved");
        assert_eq!(report.observation_timeline[1].status, "unresolved");
        assert_eq!(report.observation_timeline[2].status, "determinate");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A should have candidate resolution");
        let b = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "B")
            .expect("B should have candidate resolution");
        assert_eq!(a.convergence_mode, "resolved_after_defer");
        assert_eq!(a.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(b.convergence_mode, "resolved_after_defer");
        assert_eq!(b.resolved_at_observation_time.as_deref(), Some("2.000"));
        assert_eq!(report.observation_summary.status, "determinate");
    }

    #[test]
    fn visibility_condition_can_prefer_a_pursuit_candidate() {
        let source = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (0, 4, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (2, 1, -1)
max(wall) = (3, 3, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (0, 1, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("pursue"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("pursue"));
    }

    #[test]
    fn visibility_condition_does_not_prefer_when_occluded() {
        let source = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (1, -1, -1)
max(wall) = (3, 1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (1, 0, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("hold"));
        assert_eq!(resolution.preferred_label, None);
    }

    #[test]
    fn visibility_world_can_switch_between_pursue_and_search() {
        let clear = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (0, 4, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (2, 1, -1)
max(wall) = (3, 3, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (0, 1, 0) score 5
    candidate_velocity(A, search) = (1, 0, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
    prefer_candidate_if_occluded(A, search, B)
observe:
    snapshot at 0
"#;
        let clear_program = parse_program(clear).expect("clear program should parse");
        let clear_report = simulate_program(&clear_program).expect("clear simulation should succeed");
        let clear_resolution = clear_report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("clear candidate resolution should be present");
        assert_eq!(clear_resolution.selected_candidate.as_deref(), Some("pursue"));
        assert_eq!(clear_resolution.preferred_label.as_deref(), Some("pursue"));

        let occluded = r#"
sphere A
sphere B
plane floor
region wall
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (4, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall) = (1, -1, -1)
max(wall) = (3, 1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (1, 0, 0) score 5
    candidate_velocity(A, search) = (0, 1, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
    prefer_candidate_if_occluded(A, search, B)
observe:
    snapshot at 0
"#;
        let occluded_program = parse_program(occluded).expect("occluded program should parse");
        let occluded_report =
            simulate_program(&occluded_program).expect("occluded simulation should succeed");
        let occluded_resolution = occluded_report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("occluded candidate resolution should be present");
        assert_eq!(occluded_resolution.selected_candidate.as_deref(), Some("search"));
        assert_eq!(occluded_resolution.preferred_label.as_deref(), Some("search"));
    }

    #[test]
    fn visibility_corridor_world_can_switch_between_pursue_and_search() {
        let clear = r#"
sphere A
sphere B
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (1, 0, 0) score 5
    candidate_velocity(A, search) = (0, 1, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
    prefer_candidate_if_occluded(A, search, B)
observe:
    snapshot at 0
"#;
        let clear_program = parse_program(clear).expect("clear corridor program should parse");
        let clear_report =
            simulate_program(&clear_program).expect("clear corridor simulation should succeed");
        let clear_resolution = clear_report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("clear corridor candidate resolution should be present");
        assert_eq!(clear_resolution.selected_candidate.as_deref(), Some("pursue"));

        let occluded = r#"
sphere A
sphere B
plane floor
region wall_top
region wall_bottom
region blocker
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 0, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
min(blocker) = (2.5, -0.5, -1)
max(blocker) = (3.5, 0.5, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (1, 0, 0) score 5
    candidate_velocity(A, search) = (0, 1, 0) score 5
    prefer_candidate_if_visible(A, pursue, B)
    prefer_candidate_if_occluded(A, search, B)
observe:
    snapshot at 0
"#;
        let occluded_program =
            parse_program(occluded).expect("occluded corridor program should parse");
        let occluded_report = simulate_program(&occluded_program)
            .expect("occluded corridor simulation should succeed");
        let occluded_resolution = occluded_report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("occluded corridor candidate resolution should be present");
        assert_eq!(occluded_resolution.selected_candidate.as_deref(), Some("search"));
        assert_eq!(occluded_resolution.preferred_label.as_deref(), Some("search"));
    }

    #[test]
    fn visibility_corridor_can_resolve_after_becoming_visible() {
        let source = r#"
sphere A
sphere B
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 2, 0)
velocity(B) = (0, -2, 0)
radius(B) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue) = (1, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue, B)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_preference");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("pursue"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("pursue"));
        assert_eq!(resolution.resolved_at_observation_time.as_deref(), Some("1.000"));
    }

    #[test]
    fn visibility_corridor_can_resolve_after_becoming_occluded() {
        let source = r#"
sphere A
sphere B
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 0, 0)
velocity(B) = (0, 2, 0)
radius(B) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, search) = (0, 1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_occluded(A, search, B)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_preference");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("search"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("search"));
        assert_eq!(resolution.resolved_at_observation_time.as_deref(), Some("1.000"));
    }

    #[test]
    fn visibility_handoff_can_resolve_toward_b() {
        let source = r#"
sphere A
sphere B
sphere C
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 2, 0)
velocity(B) = (0, -2, 0)
radius(B) = 1
position(C) = (6, -2, 0)
velocity(C) = (0, 0, 0)
radius(C) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_b) = (1, 1, 0) score 5
    candidate_velocity(A, pursue_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_b, B)
    prefer_candidate_if_visible(A, pursue_c, C)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_preference");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("pursue_b"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("pursue_b"));
        assert_eq!(resolution.resolved_at_observation_time.as_deref(), Some("1.000"));
    }

    #[test]
    fn visibility_handoff_can_resolve_toward_c() {
        let source = r#"
sphere A
sphere B
sphere C
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (6, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
position(C) = (6, -2, 0)
velocity(C) = (0, 2, 0)
radius(C) = 1
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_b) = (1, 1, 0) score 5
    candidate_velocity(A, pursue_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_b, B)
    prefer_candidate_if_visible(A, pursue_c, C)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be present");
        assert_eq!(resolution.convergence_mode, "resolved_after_preference");
        assert_eq!(resolution.selected_candidate.as_deref(), Some("pursue_c"));
        assert_eq!(resolution.preferred_label.as_deref(), Some("pursue_c"));
        assert_eq!(resolution.resolved_at_observation_time.as_deref(), Some("1.000"));
    }

    #[test]
    fn visibility_coordination_can_resolve_multiple_entities_when_target_becomes_visible() {
        let source = r#"
sphere A
sphere D
sphere B
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0.0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 0.5
position(D) = (0, -0.5, 0)
velocity(D) = (0, 0, 0)
radius(D) = 0.5
position(B) = (6, 2, 0)
velocity(B) = (0, -2, 0)
radius(B) = 0.5
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold_a) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_a) = (1, 0, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_a, B)
    candidate_velocity(D, hold_d) = (0, 0, 0) score 5
    candidate_velocity(D, support_d) = (1, 0, 0) score 5
    defer_on_ambiguous_top(D)
    resolve_deferred_at(D, 1)
    prefer_candidate_if_visible(D, support_d, B)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A candidate resolution should be present");
        let d = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "D")
            .expect("D candidate resolution should be present");
        assert_eq!(a.convergence_mode, "resolved_after_preference");
        assert_eq!(d.convergence_mode, "resolved_after_preference");
        assert_eq!(a.selected_candidate.as_deref(), Some("pursue_a"));
        assert_eq!(d.selected_candidate.as_deref(), Some("support_d"));
        assert_eq!(a.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(d.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(report.observation_summary.status, "determinate");
    }

    #[test]
    fn visibility_coordination_can_resolve_multiple_entities_when_target_becomes_occluded() {
        let source = r#"
sphere A
sphere D
sphere B
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0.0, 0)
velocity(A) = (0, 0, 0)
radius(A) = 0.5
position(D) = (0, -0.5, 0)
velocity(D) = (0, 0, 0)
radius(D) = 0.5
position(B) = (6, 0, 0)
velocity(B) = (0, 2, 0)
radius(B) = 0.5
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold_a) = (0, 0, 0) score 5
    candidate_velocity(A, search_a) = (0, 1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_occluded(A, search_a, B)
    candidate_velocity(D, hold_d) = (0, 0, 0) score 5
    candidate_velocity(D, cover_d) = (0, -1, 0) score 5
    defer_on_ambiguous_top(D)
    resolve_deferred_at(D, 1)
    prefer_candidate_if_occluded(D, cover_d, B)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A candidate resolution should be present");
        let d = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "D")
            .expect("D candidate resolution should be present");
        assert_eq!(a.convergence_mode, "resolved_after_preference");
        assert_eq!(d.convergence_mode, "resolved_after_preference");
        assert_eq!(a.selected_candidate.as_deref(), Some("search_a"));
        assert_eq!(d.selected_candidate.as_deref(), Some("cover_d"));
        assert_eq!(a.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(d.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(report.observation_summary.status, "determinate");
    }

    #[test]
    fn visibility_network_can_assign_roles_toward_b() {
        let source = r#"
sphere A
sphere D
sphere B
sphere C
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0.5, 0)
velocity(A) = (0, 0, 0)
radius(A) = 0.5
position(D) = (0, -0.5, 0)
velocity(D) = (0, 0, 0)
radius(D) = 0.5
position(B) = (6, 2, 0)
velocity(B) = (0, -2, 0)
radius(B) = 0.5
position(C) = (6, -2, 0)
velocity(C) = (0, 0, 0)
radius(C) = 0.5
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold_a) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_b) = (1, 1, 0) score 5
    candidate_velocity(A, pursue_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_b, B)
    prefer_candidate_if_visible(A, pursue_c, C)
    candidate_velocity(D, hold_d) = (0, 0, 0) score 5
    candidate_velocity(D, support_b) = (1, 1, 0) score 5
    candidate_velocity(D, support_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(D)
    resolve_deferred_at(D, 1)
    prefer_candidate_if_visible(D, support_b, B)
    prefer_candidate_if_visible(D, support_c, C)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A candidate resolution should be present");
        let d = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "D")
            .expect("D candidate resolution should be present");
        assert_eq!(a.selected_candidate.as_deref(), Some("pursue_b"));
        assert_eq!(d.selected_candidate.as_deref(), Some("support_b"));
        assert_eq!(a.preferred_label.as_deref(), Some("pursue_b"));
        assert_eq!(d.preferred_label.as_deref(), Some("support_b"));
    }

    #[test]
    fn visibility_network_can_assign_roles_toward_c() {
        let source = r#"
sphere A
sphere D
sphere B
sphere C
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0.5, 0)
velocity(A) = (0, 0, 0)
radius(A) = 0.5
position(D) = (0, -0.5, 0)
velocity(D) = (0, 0, 0)
radius(D) = 0.5
position(B) = (6, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 0.5
position(C) = (6, -2, 0)
velocity(C) = (0, 2, 0)
radius(C) = 0.5
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold_a) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_b) = (1, 1, 0) score 5
    candidate_velocity(A, pursue_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_b, B)
    prefer_candidate_if_visible(A, pursue_c, C)
    candidate_velocity(D, hold_d) = (0, 0, 0) score 5
    candidate_velocity(D, support_b) = (1, 1, 0) score 5
    candidate_velocity(D, support_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(D)
    resolve_deferred_at(D, 1)
    prefer_candidate_if_visible(D, support_b, B)
    prefer_candidate_if_visible(D, support_c, C)
observe:
    snapshot at 0
    snapshot at 1
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A candidate resolution should be present");
        let d = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "D")
            .expect("D candidate resolution should be present");
        assert_eq!(a.selected_candidate.as_deref(), Some("pursue_c"));
        assert_eq!(d.selected_candidate.as_deref(), Some("support_c"));
        assert_eq!(a.preferred_label.as_deref(), Some("pursue_c"));
        assert_eq!(d.preferred_label.as_deref(), Some("support_c"));
    }

    #[test]
    fn visibility_network_can_assign_roles_across_staggered_frontiers() {
        let source = r#"
sphere A
sphere D
sphere B
sphere C
plane floor
region wall_top
region wall_bottom
position(A) = (0, 0.5, 0)
velocity(A) = (0, 0, 0)
radius(A) = 0.5
position(D) = (0, -0.5, 0)
velocity(D) = (0, 0, 0)
radius(D) = 0.5
position(B) = (6, 2, 0)
velocity(B) = (0, -2, 0)
radius(B) = 0.5
position(C) = (6, -4, 0)
velocity(C) = (0, 2, 0)
radius(C) = 0.5
min(wall_top) = (1, 1, -1)
max(wall_top) = (5, 3, 1)
min(wall_bottom) = (1, -3, -1)
max(wall_bottom) = (5, -1, 1)
action:
    candidate_velocity(A, hold_a) = (0, 0, 0) score 5
    candidate_velocity(A, pursue_b) = (1, 1, 0) score 5
    candidate_velocity(A, pursue_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(A)
    resolve_deferred_at(A, 1)
    prefer_candidate_if_visible(A, pursue_b, B)
    prefer_candidate_if_visible(A, pursue_c, C)
    candidate_velocity(D, hold_d) = (0, 0, 0) score 5
    candidate_velocity(D, support_b) = (1, 1, 0) score 5
    candidate_velocity(D, support_c) = (1, -1, 0) score 5
    defer_on_ambiguous_top(D)
    resolve_deferred_at(D, 2)
    prefer_candidate_if_visible(D, support_b, B)
    prefer_candidate_if_visible(D, support_c, C)
observe:
    snapshot at 0
    snapshot at 1
    snapshot at 2
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("A candidate resolution should be present");
        let d = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "D")
            .expect("D candidate resolution should be present");
        assert_eq!(report.observation_timeline.len(), 3);
        assert_eq!(report.observation_timeline[0].status, "unresolved");
        assert_eq!(report.observation_timeline[1].status, "unresolved");
        assert_eq!(report.observation_timeline[2].status, "determinate");
        assert_eq!(a.selected_candidate.as_deref(), Some("pursue_b"));
        assert_eq!(d.selected_candidate.as_deref(), Some("support_c"));
        assert_eq!(a.resolved_at_observation_time.as_deref(), Some("1.000"));
        assert_eq!(d.resolved_at_observation_time.as_deref(), Some("2.000"));
        assert_eq!(a.preferred_label.as_deref(), Some("pursue_b"));
        assert_eq!(d.preferred_label.as_deref(), Some("support_c"));
    }
}
