use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct VersionManifest {
    latest: LatestVersions,
    versions: Vec<GameVersion>,
}

#[derive(Debug, Clone, Deserialize)]
struct LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameVersion {
    id: String,
    r#type: String,
    url: String,
    time: String,
    release_time: String,
}

/// Fetches the version manifest from the Mojang api
///
/// https://minecraft.wiki/w/Version_manifest.json
pub async fn get_version_manifest() -> Result<VersionManifest> {
    // TODO: Cache this value
    let version_manifest: VersionManifest =
        reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
            .await?
            .json()
            .await?;

    Ok(version_manifest)
}

pub async fn get_release_version_names() -> Result<Vec<String>> {
    Ok(get_version_manifest()
        .await?
        .versions
        .iter()
        .filter_map(|v| {
            if v.r#type == "release" {
                Some(v.id.clone())
            } else {
                None
            }
        })
        .collect())
}

pub async fn get_version_names() -> Result<Vec<String>> {
    Ok(get_version_manifest()
        .await?
        .versions
        .iter()
        .map(|v| v.id.clone())
        .collect())
}
