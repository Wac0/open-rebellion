//! Native fixture proof for the platform-neutral replay data manifest.

use std::path::PathBuf;

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::ai::{AIState, AiFaction};
use rebellion_core::betrayal::BetrayalState;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::dat::Faction;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::economy::EconomyState;
use rebellion_core::events::EventState;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::jedi::JediState;
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::MissionState;
use rebellion_core::movement::MovementState;
use rebellion_core::repair::RepairState;
use rebellion_core::research::ResearchState;
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::tuning::GameConfig;
use rebellion_core::uprising::UprisingState;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::{CampaignConfig, SeedOptions, VictoryConditions};
use rebellion_data::replay::{
    compute_simulation_data_manifest_from_dir, execute_replay, record_replay, ReplayActor,
    ReplayCommand, ReplayEnvironment,
};
use rebellion_data::save::{compute_state_fingerprint, load_slot, save_slot, SaveState};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository root")
        .join("data/base")
}

fn original_save_state(seed: u64) -> SaveState {
    let options = SeedOptions {
        rng_seed: Some(seed),
        ..SeedOptions::default()
    };
    let world = rebellion_data::load_game_data_with_options(&data_dir(), &options)
        .expect("load original campaign data");
    let alliance_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Alliance)
        })
        .map(|(key, _)| key)
        .expect("alliance headquarters");
    let empire_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Empire)
        })
        .map(|(key, _)| key)
        .expect("empire headquarters");
    let mut fog_alliance = FogState::new(Faction::Alliance);
    let mut fog_empire = FogState::new(Faction::Empire);
    FogSystem::seed(&mut fog_alliance, &world);
    FogSystem::seed(&mut fog_empire, &world);
    let mut events = EventState::new();
    rebellion_core::story_events::define_story_events(&mut events, &world);

    SaveState {
        world,
        clock: GameClock::new(),
        manufacturing: ManufacturingState::new(),
        missions: MissionState::new(),
        events,
        ai: AIState::new(AiFaction::Empire),
        movement: MovementState::new(),
        fog_alliance,
        fog_empire,
        player_is_alliance: true,
        blockade: BlockadeState::new(),
        uprising: UprisingState::new(),
        death_star: DeathStarState::default(),
        research: ResearchState::new(),
        jedi: JediState::new(),
        victory: VictoryState::new(alliance_hq, empire_hq),
        betrayal: BetrayalState::new(),
        economy: EconomyState::default(),
        sim_rng: Xoshiro256PlusPlus::seed_from_u64(seed),
        ai2: None,
        repair: RepairState,
        combat_cooldowns: std::collections::HashMap::new(),
        game_config: GameConfig::default(),
        campaign_config: CampaignConfig::from_seed_options(options, VictoryConditions::Standard),
    }
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn original_simulation_data_has_canonical_identity() {
    let manifest =
        compute_simulation_data_manifest_from_dir(&data_dir()).expect("fingerprint original DATs");

    assert_eq!(manifest.inputs.len(), 51);
    assert_eq!(manifest.total_bytes, 50_597);
    assert_eq!(manifest.aggregate_fingerprint.value, "5facb1c7ba0e81ad");
    assert_eq!(manifest.inputs.first().unwrap().name, "ABDCMSTB.DAT");
    assert_eq!(manifest.inputs.last().unwrap().name, "UPRIS2TB.DAT");
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn original_campaign_replay_matches_after_save_v11_reload() {
    let seed = 42;
    let data =
        compute_simulation_data_manifest_from_dir(&data_dir()).expect("fingerprint original DATs");
    let environment = ReplayEnvironment {
        engine_version: "0.1.0-test",
        seed,
        data: &data,
    };
    let initial = original_save_state(seed);
    let initial_fingerprint = compute_state_fingerprint(&initial).unwrap();
    let mut commands = vec![
        (
            ReplayActor::Alliance,
            ReplayCommand::SetSpeed {
                speed: GameSpeed::Faster,
            },
        ),
        (ReplayActor::Engine, ReplayCommand::ToggleDualAi),
    ];
    commands.extend((0..5).map(|_| {
        (
            ReplayActor::Engine,
            ReplayCommand::AdvanceTicks { count: 5 },
        )
    }));
    commands.extend([
        (ReplayActor::Engine, ReplayCommand::RevealAllSystems),
        (ReplayActor::Engine, ReplayCommand::ForceVictoryCheck),
    ]);
    let recording = record_replay(environment, initial.clone(), commands).expect("record replay");

    let saves = tempfile::tempdir().expect("temporary save directory");
    save_slot(saves.path(), 0, "Replay Start", &initial, &[]).expect("save initial state");
    let (_, restored) = load_slot(saves.path(), 0).expect("reload initial state");
    let executed =
        execute_replay(environment, &recording.manifest, restored).expect("execute replay");
    let executed_fingerprint = compute_state_fingerprint(&executed.final_state).unwrap();
    let recorded_fingerprint = compute_state_fingerprint(&recording.execution.final_state).unwrap();

    assert_eq!(
        executed.observed_checkpoints,
        recording.manifest.checkpoints
    );
    assert_eq!(executed_fingerprint, recorded_fingerprint);
    assert_eq!(executed.final_state.clock.tick, 25);
    assert_eq!(initial_fingerprint.to_string(), "v1:765c9318acb5cb50");
    let observed_fingerprints: Vec<_> = recording
        .manifest
        .checkpoints
        .iter()
        .map(|checkpoint| {
            (
                checkpoint.command_count,
                checkpoint.tick,
                checkpoint.state_fingerprint.to_string(),
            )
        })
        .collect();
    assert_eq!(
        observed_fingerprints,
        vec![
            (1, 0, "v1:48f9a97ca6dec269".into()),
            (2, 0, "v1:357708f65600307b".into()),
            (3, 5, "v1:c3782c7f2ba38f9d".into()),
            (4, 10, "v1:772525cf399636af".into()),
            (5, 15, "v1:f0131ad2a2d7a2c1".into()),
            (6, 20, "v1:54316ab6da548898".into()),
            (7, 25, "v1:bc68cc7ac27bd974".into()),
            (8, 25, "v1:f512773b607069ee".into()),
            (9, 25, "v1:f512773b607069ee".into()),
        ]
    );
    assert_eq!(executed_fingerprint.to_string(), "v1:f512773b607069ee");
}
