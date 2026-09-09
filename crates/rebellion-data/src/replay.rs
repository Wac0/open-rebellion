//! Versioned, platform-neutral replay artifacts.
//!
//! A replay identifies the exact simulation `.DAT` inputs and tuning
//! configuration, then records state-changing commands in one total order.
//! Native and WASM runners can therefore reject incompatible inputs before
//! comparing versioned save-state fingerprints at declared checkpoints.

use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use anyhow::{bail, Context};
use rebellion_core::tick::GameSpeed;
use rebellion_core::tuning::GameConfig;
use serde::{Deserialize, Serialize};

use crate::save::StateFingerprint;

/// Current JSON replay envelope version.
pub const REPLAY_FORMAT_VERSION: u16 = 1;
/// Current canonical simulation-input manifest version.
pub const DATA_MANIFEST_VERSION: u16 = 1;
/// Current tuning-configuration fingerprint version.
pub const CONFIG_FINGERPRINT_VERSION: u16 = 1;

const FINGERPRINT_ALGORITHM: &str = "fnv1a64";
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// JSON-safe representation of a versioned 64-bit fingerprint.
///
/// The hexadecimal string avoids the precision loss that JavaScript numbers
/// can introduce above `2^53`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentFingerprint {
    pub algorithm: String,
    pub version: u16,
    pub value: String,
}

impl ContentFingerprint {
    fn fnv1a(version: u16, value: u64) -> Self {
        Self {
            algorithm: FINGERPRINT_ALGORITHM.to_string(),
            version,
            value: format!("{value:016x}"),
        }
    }

    fn validate(&self, expected_version: u16, label: &str) -> anyhow::Result<()> {
        if self.algorithm != FINGERPRINT_ALGORITHM {
            bail!(
                "unsupported {label} fingerprint algorithm {:?}",
                self.algorithm
            );
        }
        if self.version != expected_version {
            bail!(
                "unsupported {label} fingerprint version {} (expected {})",
                self.version,
                expected_version
            );
        }
        if self.value.len() != 16
            || !self
                .value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            bail!("invalid {label} fingerprint value {:?}", self.value);
        }
        Ok(())
    }
}

/// Fingerprint and size of one canonical simulation input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataInputFingerprint {
    pub name: String,
    pub byte_length: u64,
    pub fingerprint: ContentFingerprint,
}

/// Canonical identity of every `.DAT` byte consumed by the simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationDataManifest {
    pub format_version: u16,
    pub total_bytes: u64,
    pub aggregate_fingerprint: ContentFingerprint,
    pub inputs: Vec<DataInputFingerprint>,
}

impl SimulationDataManifest {
    /// Reject unsupported, malformed, incomplete, or ambiguously ordered data.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.format_version != DATA_MANIFEST_VERSION {
            bail!(
                "unsupported data-manifest version {} (expected {})",
                self.format_version,
                DATA_MANIFEST_VERSION
            );
        }
        if self.inputs.is_empty() {
            bail!("simulation data manifest contains no inputs");
        }
        self.aggregate_fingerprint
            .validate(DATA_MANIFEST_VERSION, "aggregate data")?;

        let mut previous_name: Option<&str> = None;
        let mut observed_total = 0_u64;
        for input in &self.inputs {
            validate_data_name(&input.name)?;
            if previous_name.is_some_and(|previous| previous >= input.name.as_str()) {
                bail!(
                    "simulation data inputs are not in unique canonical order at {}",
                    input.name
                );
            }
            previous_name = Some(&input.name);
            observed_total = observed_total
                .checked_add(input.byte_length)
                .context("simulation data byte count overflow")?;
            input
                .fingerprint
                .validate(DATA_MANIFEST_VERSION, &format!("data input {}", input.name))?;
        }
        if observed_total != self.total_bytes {
            bail!(
                "simulation data byte count mismatch: manifest {}, inputs {}",
                self.total_bytes,
                observed_total
            );
        }
        let observed_aggregate = aggregate_data_fingerprint(&self.inputs)?;
        if observed_aggregate != self.aggregate_fingerprint {
            bail!(
                "simulation data aggregate mismatch: manifest {}, inputs {}",
                self.aggregate_fingerprint.value,
                observed_aggregate.value
            );
        }
        Ok(())
    }
}

/// Identity responsible for a replayed command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayActor {
    Engine,
    Alliance,
    Empire,
}

/// State-changing commands supported by replay format v1.
///
/// These are the current headless/control commands. Gameplay orders will join
/// this enum when the app and playtest runner move behind one command boundary;
/// changes to existing command semantics require a replay format bump.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReplayCommand {
    AdvanceTicks { count: u64 },
    SetSpeed { speed: GameSpeed },
    ToggleDualAi,
    RevealAllSystems,
    ForceVictoryCheck,
}

impl ReplayCommand {
    fn validate(&self, actor: ReplayActor) -> anyhow::Result<()> {
        if let Self::AdvanceTicks { count: 0 } = self {
            bail!("advance_ticks command count must be greater than zero");
        }
        if matches!(
            self,
            Self::AdvanceTicks { .. }
                | Self::ToggleDualAi
                | Self::RevealAllSystems
                | Self::ForceVictoryCheck
        ) && actor != ReplayActor::Engine
        {
            bail!("engine control command cannot be issued by {actor:?}");
        }
        Ok(())
    }
}

/// One command at a precise position in the replay's total order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCommandRecord {
    /// Simulation tick immediately before the command is applied.
    pub tick: u64,
    /// Zero-based, gap-free global command sequence.
    pub sequence: u64,
    pub actor: ReplayActor,
    pub command: ReplayCommand,
}

/// Expected state after a declared prefix of the command stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCheckpoint {
    pub tick: u64,
    /// Number of commands already applied at this checkpoint.
    pub command_count: u64,
    /// Display form emitted by `StateFingerprint`, such as `v1:0123abcd...`.
    pub state_fingerprint: String,
}

/// Complete replay identity and ordered command/checkpoint stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayManifest {
    pub format: String,
    pub format_version: u16,
    pub engine_version: String,
    pub seed: u64,
    pub rng: String,
    pub rolls_per_tick: u16,
    pub data: SimulationDataManifest,
    pub config_fingerprint: ContentFingerprint,
    pub initial_state_fingerprint: String,
    pub commands: Vec<ReplayCommandRecord>,
    pub checkpoints: Vec<ReplayCheckpoint>,
}

impl ReplayManifest {
    pub fn new(
        engine_version: impl Into<String>,
        seed: u64,
        data: SimulationDataManifest,
        config: &GameConfig,
        initial_state_fingerprint: StateFingerprint,
    ) -> anyhow::Result<Self> {
        let manifest = Self {
            format: "open-rebellion-replay".to_string(),
            format_version: REPLAY_FORMAT_VERSION,
            engine_version: engine_version.into(),
            seed,
            rng: "xoshiro256++/0.6".to_string(),
            rolls_per_tick: 1024,
            data,
            config_fingerprint: compute_config_fingerprint(config)?,
            initial_state_fingerprint: initial_state_fingerprint.to_string(),
            commands: Vec::new(),
            checkpoints: Vec::new(),
        };
        manifest.validate()?;
        Ok(manifest)
    }

    /// Append a command while assigning the only valid next sequence number.
    pub fn record_command(
        &mut self,
        tick: u64,
        actor: ReplayActor,
        command: ReplayCommand,
    ) -> anyhow::Result<u64> {
        if self
            .commands
            .last()
            .is_some_and(|previous| tick < previous.tick)
        {
            bail!("command tick {tick} precedes the previous command");
        }
        command.validate(actor)?;
        let sequence = u64::try_from(self.commands.len()).context("too many replay commands")?;
        self.commands.push(ReplayCommandRecord {
            tick,
            sequence,
            actor,
            command,
        });
        Ok(sequence)
    }

    /// Append a checkpoint after `command_count` commands have been applied.
    pub fn record_checkpoint(
        &mut self,
        tick: u64,
        command_count: u64,
        state_fingerprint: StateFingerprint,
    ) -> anyhow::Result<()> {
        if command_count > self.commands.len() as u64 {
            bail!(
                "checkpoint references {} commands but stream contains {}",
                command_count,
                self.commands.len()
            );
        }
        let checkpoint = ReplayCheckpoint {
            tick,
            command_count,
            state_fingerprint: state_fingerprint.to_string(),
        };
        if self.checkpoints.last().is_some_and(|previous| {
            (checkpoint.tick, checkpoint.command_count) <= (previous.tick, previous.command_count)
        }) {
            bail!("checkpoint positions must be unique and strictly increasing");
        }
        validate_state_fingerprint(&checkpoint.state_fingerprint)?;
        self.checkpoints.push(checkpoint);
        Ok(())
    }

    /// Validate the full replay before execution or comparison.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.format != "open-rebellion-replay" {
            bail!("unsupported replay format {:?}", self.format);
        }
        if self.format_version != REPLAY_FORMAT_VERSION {
            bail!(
                "unsupported replay format version {} (expected {})",
                self.format_version,
                REPLAY_FORMAT_VERSION
            );
        }
        if self.engine_version.trim().is_empty() {
            bail!("replay engine version is empty");
        }
        if self.rng != "xoshiro256++/0.6" {
            bail!("unsupported replay RNG {:?}", self.rng);
        }
        if self.rolls_per_tick != 1024 {
            bail!(
                "unsupported replay roll budget {} (expected 1024)",
                self.rolls_per_tick
            );
        }
        self.data.validate()?;
        self.config_fingerprint
            .validate(CONFIG_FINGERPRINT_VERSION, "configuration")?;
        validate_state_fingerprint(&self.initial_state_fingerprint)?;

        let mut previous_tick = 0_u64;
        for (index, record) in self.commands.iter().enumerate() {
            let expected_sequence = index as u64;
            if record.sequence != expected_sequence {
                bail!(
                    "replay command sequence is not gap-free: expected {}, found {}",
                    expected_sequence,
                    record.sequence
                );
            }
            if index > 0 && record.tick < previous_tick {
                bail!("replay command ticks are not monotonic at sequence {index}");
            }
            record.command.validate(record.actor)?;
            previous_tick = record.tick;
        }

        let mut previous_position: Option<(u64, u64)> = None;
        for checkpoint in &self.checkpoints {
            if checkpoint.command_count > self.commands.len() as u64 {
                bail!(
                    "checkpoint references {} commands but stream contains {}",
                    checkpoint.command_count,
                    self.commands.len()
                );
            }
            let position = (checkpoint.tick, checkpoint.command_count);
            if previous_position.is_some_and(|previous| position <= previous) {
                bail!("checkpoint positions must be unique and strictly increasing");
            }
            if checkpoint.command_count > 0 {
                let command = &self.commands[checkpoint.command_count as usize - 1];
                if checkpoint.tick < command.tick {
                    bail!(
                        "checkpoint tick {} precedes applied command {} at tick {}",
                        checkpoint.tick,
                        command.sequence,
                        command.tick
                    );
                }
            }
            validate_state_fingerprint(&checkpoint.state_fingerprint)?;
            previous_position = Some(position);
        }
        Ok(())
    }

    /// Serialize a validated replay using stable struct field ordering.
    pub fn to_json_pretty(&self) -> anyhow::Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serializing replay manifest")
    }

    /// Decode and validate a replay before it reaches an executor.
    pub fn from_json(bytes: &[u8]) -> anyhow::Result<Self> {
        let manifest: Self =
            serde_json::from_slice(bytes).context("decoding replay manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }
}

/// Hash an in-memory set of `.DAT` files identically on native and WASM.
pub fn compute_simulation_data_manifest<I, K, V>(
    inputs: I,
) -> anyhow::Result<SimulationDataManifest>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<[u8]>,
{
    let mut canonical_inputs = Vec::new();
    for (name, bytes) in inputs {
        let name = canonical_data_name(name.as_ref())?;
        canonical_inputs.push((name, bytes.as_ref().to_vec()));
    }
    if canonical_inputs.is_empty() {
        bail!("cannot fingerprint an empty simulation data set");
    }
    canonical_inputs.sort_by(|left, right| left.0.cmp(&right.0));

    let mut names = HashSet::with_capacity(canonical_inputs.len());
    let mut manifest_inputs = Vec::with_capacity(canonical_inputs.len());
    let mut total_bytes = 0_u64;
    for (name, bytes) in canonical_inputs {
        if !names.insert(name.clone()) {
            bail!("duplicate canonical simulation data input {name}");
        }
        let byte_length = u64::try_from(bytes.len()).context("simulation data input too large")?;
        total_bytes = total_bytes
            .checked_add(byte_length)
            .context("simulation data byte count overflow")?;
        let value = fingerprint_bytes(
            b"OPENREB-DATA-INPUT\0",
            DATA_MANIFEST_VERSION,
            &name,
            &bytes,
        );
        manifest_inputs.push(DataInputFingerprint {
            name,
            byte_length,
            fingerprint: ContentFingerprint::fnv1a(DATA_MANIFEST_VERSION, value),
        });
    }

    let aggregate_fingerprint = aggregate_data_fingerprint(&manifest_inputs)?;

    let manifest = SimulationDataManifest {
        format_version: DATA_MANIFEST_VERSION,
        total_bytes,
        aggregate_fingerprint,
        inputs: manifest_inputs,
    };
    manifest.validate()?;
    Ok(manifest)
}

fn aggregate_data_fingerprint(
    inputs: &[DataInputFingerprint],
) -> anyhow::Result<ContentFingerprint> {
    let mut aggregate = Fnv1a64::new();
    aggregate.update(b"OPENREB-DATA-MANIFEST\0");
    aggregate.update(&DATA_MANIFEST_VERSION.to_le_bytes());
    aggregate.update(&(inputs.len() as u64).to_le_bytes());
    for input in inputs {
        aggregate.update(&(input.name.len() as u64).to_le_bytes());
        aggregate.update(input.name.as_bytes());
        aggregate.update(&input.byte_length.to_le_bytes());
        let value = u64::from_str_radix(&input.fingerprint.value, 16)
            .with_context(|| format!("decoding data input fingerprint for {}", input.name))?;
        aggregate.update(&value.to_le_bytes());
    }
    Ok(ContentFingerprint::fnv1a(
        DATA_MANIFEST_VERSION,
        aggregate.finish(),
    ))
}

/// Read and fingerprint every `.DAT` file in a native game-data directory.
#[cfg(not(target_arch = "wasm32"))]
pub fn compute_simulation_data_manifest_from_dir(
    data_dir: &Path,
) -> anyhow::Result<SimulationDataManifest> {
    let mut inputs = Vec::new();
    for entry in std::fs::read_dir(data_dir)
        .with_context(|| format!("reading simulation data directory {}", data_dir.display()))?
    {
        let entry = entry.context("reading simulation data directory entry")?;
        let path = entry.path();
        if !path.is_file()
            || path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_none_or(|extension| !extension.eq_ignore_ascii_case("dat"))
        {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("simulation data filename is not valid UTF-8")?
            .to_string();
        let bytes = std::fs::read(&path)
            .with_context(|| format!("reading simulation data input {}", path.display()))?;
        inputs.push((name, bytes));
    }
    compute_simulation_data_manifest(inputs)
}

/// Fingerprint the serialized tuning configuration embedded in a replay.
pub fn compute_config_fingerprint(config: &GameConfig) -> anyhow::Result<ContentFingerprint> {
    let bytes = serde_json::to_vec(config).context("serializing replay configuration")?;
    Ok(ContentFingerprint::fnv1a(
        CONFIG_FINGERPRINT_VERSION,
        fingerprint_bytes(
            b"OPENREB-GAME-CONFIG\0",
            CONFIG_FINGERPRINT_VERSION,
            "game-config",
            &bytes,
        ),
    ))
}

fn canonical_data_name(name: &str) -> anyhow::Result<String> {
    if name.is_empty() || !name.is_ascii() || name.contains(['/', '\\']) {
        bail!("invalid flat simulation data filename {name:?}");
    }
    let canonical = name.to_ascii_uppercase();
    validate_data_name(&canonical)?;
    Ok(canonical)
}

fn validate_data_name(name: &str) -> anyhow::Result<()> {
    if name.is_empty()
        || !name.is_ascii()
        || name != name.to_ascii_uppercase()
        || name.contains(['/', '\\'])
        || !name.ends_with(".DAT")
    {
        bail!("invalid canonical simulation data filename {name:?}");
    }
    Ok(())
}

fn validate_state_fingerprint(value: &str) -> anyhow::Result<()> {
    let Some((version, digest)) = value.split_once(':') else {
        bail!("invalid state fingerprint {value:?}");
    };
    if version.len() < 2
        || !version.starts_with('v')
        || !version[1..].bytes().all(|byte| byte.is_ascii_digit())
        || digest.len() != 16
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("invalid state fingerprint {value:?}");
    }
    Ok(())
}

fn fingerprint_bytes(domain: &[u8], version: u16, name: &str, bytes: &[u8]) -> u64 {
    let mut hash = Fnv1a64::new();
    hash.update(domain);
    hash.update(&version.to_le_bytes());
    hash.update(&(name.len() as u64).to_le_bytes());
    hash.update(name.as_bytes());
    hash.update(&(bytes.len() as u64).to_le_bytes());
    hash.update(bytes);
    hash.finish()
}

struct Fnv1a64(u64);

impl Fnv1a64 {
    fn new() -> Self {
        Self(FNV_OFFSET_BASIS)
    }

    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> SimulationDataManifest {
        compute_simulation_data_manifest([
            ("systemsD.dat", b"systems".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap()
    }

    fn sample_state_fingerprint() -> StateFingerprint {
        StateFingerprint {
            version: 1,
            value: 0x0123_4567_89ab_cdef,
        }
    }

    #[test]
    fn data_manifest_is_order_independent_and_json_safe() {
        let forward = sample_data();
        let reverse = compute_simulation_data_manifest([
            ("CAPSHPSD.DAT", b"ships".as_slice()),
            ("SYSTEMSD.DAT", b"systems".as_slice()),
        ])
        .unwrap();

        assert_eq!(forward, reverse);
        assert_eq!(forward.total_bytes, 12);
        assert_eq!(forward.inputs[0].name, "CAPSHPSD.DAT");
        assert_eq!(forward.inputs[1].name, "SYSTEMSD.DAT");
        assert_eq!(forward.aggregate_fingerprint.value.len(), 16);
        assert!(forward
            .aggregate_fingerprint
            .value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
    }

    #[test]
    fn data_manifest_detects_content_and_identity_changes() {
        let original = sample_data();
        let content_changed = compute_simulation_data_manifest([
            ("SYSTEMSD.DAT", b"systemz".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap();
        let name_changed = compute_simulation_data_manifest([
            ("SYSTEMXD.DAT", b"systems".as_slice()),
            ("CAPSHPSD.DAT", b"ships".as_slice()),
        ])
        .unwrap();

        assert_ne!(
            original.aggregate_fingerprint,
            content_changed.aggregate_fingerprint
        );
        assert_ne!(
            original.aggregate_fingerprint,
            name_changed.aggregate_fingerprint
        );

        let mut tampered = original;
        tampered.inputs[0].byte_length += 1;
        tampered.total_bytes += 1;
        assert!(tampered
            .validate()
            .unwrap_err()
            .to_string()
            .contains("aggregate mismatch"));
    }

    #[test]
    fn data_manifest_rejects_empty_ambiguous_and_non_dat_inputs() {
        let empty: Vec<(String, Vec<u8>)> = Vec::new();
        assert!(compute_simulation_data_manifest(empty).is_err());
        assert!(compute_simulation_data_manifest([
            ("systemsD.dat", b"a".as_slice()),
            ("SYSTEMSD.DAT", b"b".as_slice()),
        ])
        .unwrap_err()
        .to_string()
        .contains("duplicate canonical"));
        assert!(compute_simulation_data_manifest([("../SYSTEMSD.DAT", b"a".as_slice())]).is_err());
        assert!(compute_simulation_data_manifest([("notes.txt", b"a".as_slice())]).is_err());
    }

    #[test]
    fn replay_round_trip_preserves_total_order_and_checkpoints() {
        let mut replay = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();
        assert_eq!(
            replay
                .record_command(
                    0,
                    ReplayActor::Engine,
                    ReplayCommand::AdvanceTicks { count: 10 },
                )
                .unwrap(),
            0
        );
        assert_eq!(
            replay
                .record_command(
                    10,
                    ReplayActor::Alliance,
                    ReplayCommand::SetSpeed {
                        speed: GameSpeed::Paused,
                    },
                )
                .unwrap(),
            1
        );
        replay
            .record_checkpoint(10, 2, sample_state_fingerprint())
            .unwrap();

        let json = replay.to_json_pretty().unwrap();
        let decoded = ReplayManifest::from_json(&json).unwrap();
        assert_eq!(decoded, replay);
        assert!(std::str::from_utf8(&json)
            .unwrap()
            .contains("\"state_fingerprint\": \"v1:0123456789abcdef\""));
    }

    #[test]
    fn replay_validation_rejects_version_order_and_authority_errors() {
        let base = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();

        let mut unsupported = base.clone();
        unsupported.format_version += 1;
        assert!(unsupported.validate().is_err());

        let mut bad_sequence = base.clone();
        bad_sequence.commands.push(ReplayCommandRecord {
            tick: 0,
            sequence: 2,
            actor: ReplayActor::Engine,
            command: ReplayCommand::AdvanceTicks { count: 1 },
        });
        assert!(bad_sequence
            .validate()
            .unwrap_err()
            .to_string()
            .contains("gap-free"));

        let mut bad_actor = base;
        bad_actor.commands.push(ReplayCommandRecord {
            tick: 0,
            sequence: 0,
            actor: ReplayActor::Empire,
            command: ReplayCommand::RevealAllSystems,
        });
        assert!(bad_actor
            .validate()
            .unwrap_err()
            .to_string()
            .contains("engine control"));
    }

    #[test]
    fn replay_recording_rejects_time_travel_and_invalid_checkpoints() {
        let mut replay = ReplayManifest::new(
            "0.1.0-test",
            42,
            sample_data(),
            &GameConfig::default(),
            sample_state_fingerprint(),
        )
        .unwrap();
        replay
            .record_command(
                10,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 1 },
            )
            .unwrap();
        assert!(replay
            .record_command(
                9,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 1 },
            )
            .is_err());
        assert!(replay
            .record_command(
                10,
                ReplayActor::Engine,
                ReplayCommand::AdvanceTicks { count: 0 },
            )
            .is_err());
        assert!(replay
            .record_checkpoint(10, 2, sample_state_fingerprint())
            .is_err());
    }

    #[test]
    fn configuration_fingerprint_is_deterministic_and_sensitive() {
        let original = GameConfig::default();
        let same = GameConfig::default();
        let mut changed = GameConfig::default();
        changed.ai.tick_interval += 1;

        assert_eq!(
            compute_config_fingerprint(&original).unwrap(),
            compute_config_fingerprint(&same).unwrap()
        );
        assert_ne!(
            compute_config_fingerprint(&original).unwrap(),
            compute_config_fingerprint(&changed).unwrap()
        );
    }
}
