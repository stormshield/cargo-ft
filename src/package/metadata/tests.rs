use serde_json::json;

use super::*;

#[test]
fn missing_metadata() {
    let metadata = [
        json!(null),
        json!({}),
        json!({ "ft": null }),
        json!({ "ft": {} }),
        json!({ "ft": { "targets": null } }),
    ];

    for metadata in metadata {
        check_err(metadata.clone(), metadata.clone(), ParseMetadataError::Missing);
        check_err(json!(null), metadata.clone(), ParseMetadataError::Missing);
        check_err(metadata, json!(null), ParseMetadataError::Missing);
    }
}

#[test]
fn map_metadata() {
    check_err(json!(null), json!({ "ft": { "targets": {} } }), ParseMetadataError::Invalid);
}

#[test]
fn bool_metadata() {
    check_err(json!(null), json!({ "ft": { "targets": true } }), ParseMetadataError::Invalid);
}

#[test]
fn string_metadata() {
    check_err(json!(null), json!({ "ft": { "targets": "target" } }), ParseMetadataError::Invalid);
}

#[test]
fn missing_workspace_metadata() {
    check_err(
        json!(null),
        json!({ "ft": { "targets": { "workspace": true } } }),
        ParseMetadataError::Invalid,
    );
}

#[test]
fn package_metadata() {
    let targets = ["target1", "target2"];

    check_ok(json!(null), json!({ "ft": { "targets": targets } }), &targets);
}

#[test]
fn workspace_metadata() {
    let targets = ["target1", "target2"];

    check_ok(
        json!({ "ft": { "targets": targets } }),
        json!({ "ft": { "targets": { "workspace": true } } }),
        &targets,
    );
}

#[test]
fn package_metadata_has_priority() {
    let targets = ["target1", "target2"];

    check_ok(
        json!({ "ft": { "targets": ["unrelated_target"] } }),
        json!({ "ft": { "targets": targets } }),
        &targets,
    );
}

fn check_ok(workspace: serde_json::Value, package: serde_json::Value, expected_targets: &[&str]) {
    let targets = expected_targets.iter().copied().map(ToOwned::to_owned).collect();
    assert_eq!(metadata(workspace, package).unwrap(), FtMetadata { targets });
}

fn check_err(
    workspace: serde_json::Value,
    package: serde_json::Value,
    expected: ParseMetadataError,
) {
    assert_eq!(metadata(workspace, package).unwrap_err().current_context(), &expected);
}

fn metadata(
    workspace: serde_json::Value,
    package: serde_json::Value,
) -> MetadataResult<FtMetadata> {
    FtMetadata::parse(&FtWorkspaceMetadata::parse(workspace).unwrap(), package)
}
