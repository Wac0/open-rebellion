//! Native fixture proof for the platform-neutral replay data manifest.

use std::path::PathBuf;

use rebellion_data::replay::compute_simulation_data_manifest_from_dir;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository root")
        .join("data/base")
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn original_simulation_data_has_canonical_identity() {
    let manifest =
        compute_simulation_data_manifest_from_dir(&data_dir()).expect("fingerprint original DATs");

    assert_eq!(manifest.inputs.len(), 51);
    assert_eq!(manifest.total_bytes, 50_597);
    assert_eq!(
        manifest.aggregate_fingerprint.value,
        "5facb1c7ba0e81ad"
    );
    assert_eq!(manifest.inputs.first().unwrap().name, "ABDCMSTB.DAT");
    assert_eq!(manifest.inputs.last().unwrap().name, "UPRIS2TB.DAT");
}
