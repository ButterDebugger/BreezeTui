use dioxus::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::Write,
};

#[derive(Deserialize, Debug)]
struct FabricVersion {
    url: String,
    // maven: String,
    version: String,
    // stable: bool,
}

#[derive(Deserialize, Debug)]
struct NeoforgeVersion {
    // isSnapshot: bool,
    version: String,
}

#[component]
pub fn Installations() -> Element {
    let response = use_signal(|| "".to_string());

    rsx! {
        h1 {
            style: "color: red;",

            "Which mod loader would you like to install?"
        }
        button {
            onclick: move |_| {
                spawn(async move {
                    install_fabric(response).await;
                });
            },
            "Install Fabric"
        }
        button {
            onclick: move |_| {
                spawn(async move {
                    install_quilt(response).await;
                });
            },
            "Install Quilt"
        }
        button {
            onclick: move |_| {
                spawn(async move {
                    install_neoforge(response).await;
                });
            },
            "Install Neoforge"
        }
        p {
            "{response}"
        }
    }
}

async fn install_fabric(mut response: Signal<String>) {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if client.is_err() {
        response.set(format!(
            "Failed to create http client: {}",
            client.unwrap_err()
        ));
        return;
    }

    let client = client.unwrap();

    // Fetch the list of Fabric versions
    let res = client
        .get("https://meta.fabricmc.net/v2/versions/installer")
        .send()
        .await;

    if res.is_err() {
        response.set(format!("Request failed: {}", res.unwrap_err()));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        response.set(format!("Request failed with status: {}", res.status()));
        return;
    }

    // Get and parse the response body
    let body = res.json::<Vec<FabricVersion>>().await;
    if body.is_err() {
        response.set(format!("Failed to parse JSON: {}", body.unwrap_err()));
        return;
    }

    let body = body.unwrap();

    // Get the first (latest) version entry
    let entry = body.first();
    if entry.is_none() {
        response.set("Failed to get latest version".to_string());
        return;
    }

    let latest = entry.unwrap();

    // Get the URL of the latest version
    let url = &latest.url;

    response.set(format!("Found latest fabric version {}", latest.version));

    // Download and run the installer
    download_and_execute(client, url.to_string(), response).await;
}

async fn install_quilt(mut response: Signal<String>) {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if client.is_err() {
        response.set(format!(
            "Failed to create http client: {}",
            client.unwrap_err()
        ));
        return;
    }

    let client = client.unwrap();

    // Download and run the installer
    download_and_execute(
        client,
        "https://quiltmc.org/api/v1/download-latest-installer/java-universal".to_string(),
        response,
    )
    .await;
}

async fn install_neoforge(mut response: Signal<String>) {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if client.is_err() {
        response.set(format!(
            "Failed to create http client: {}",
            client.unwrap_err()
        ));
        return;
    }

    let client = client.unwrap();

    // Fetch Neoforge release info
    let res = client
        .get("https://maven.neoforged.net/api/maven/latest/version/releases/net%2Fneoforged%2Fneoforge")
        .send()
        .await;

    if res.is_err() {
        response.set(format!("Request failed: {}", res.unwrap_err()));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        response.set(format!("Request failed with status: {}", res.status()));
        return;
    }

    // Get and parse the response body
    let body = res.json::<NeoforgeVersion>().await;
    if body.is_err() {
        response.set(format!("Failed to parse JSON: {}", body.unwrap_err()));
        return;
    }

    let body = body.unwrap();

    // Create the URL for the installer
    let url = format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar", version = body.version);

    // Download and run the installer
    download_and_execute(client, url, response).await;
}

async fn download_and_execute(client: Client, url: String, mut response: Signal<String>) {
    // FIXME: response is not properly updating

    // Request the file
    let res = client.get(url).send().await;
    if res.is_err() {
        response.set(format!("Download failed: {}", res.unwrap_err()));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        response.set(format!("Download failed with status: {}", res.status()));
        return;
    }

    // Create temp directory
    let temp_path = temp_dir().join("breeze_execution");
    create_dir_all(temp_path.clone()).expect("Failed to create temp directory");

    // Write the file
    let mut file = File::create(temp_path.join("installer.jar")).unwrap();
    let content = res.bytes().await.unwrap();
    file.write_all(&content).unwrap();

    // Execute the installer
    let child = std::process::Command::new("java")
        .current_dir(temp_dir())
        .arg("-jar")
        .arg(temp_path.join("installer.jar"))
        .spawn();

    if child.is_err() {
        response.set(format!(
            "Failed to execute installer: {}",
            child.unwrap_err()
        ));
        return;
    }

    response.set("Starting installer".to_string());
    child.unwrap().wait().unwrap();

    response.set("Installer finished".to_string());
}
