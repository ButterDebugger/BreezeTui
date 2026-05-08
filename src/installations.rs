use crate::App;
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;
use serde::Deserialize;
use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::Write,
    thread,
    time::Duration,
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

impl App {
    pub async fn installations_cli(&mut self) {
        let selections = &["Fabric", "Quilt", "Neoforge"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which mod loader would you like to install?")
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            println!();

            match selection {
                0 => install_fabric().await,
                1 => install_quilt().await,
                2 => install_neoforge().await,
                _ => unreachable!(),
            }

            self.return_home();
        } else {
            self.go_back();
        }
    }
}

async fn install_fabric() {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if let Err(err) = client {
        println!("Failed to create http client: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let client = client.unwrap();

    // Fetch the list of Fabric versions
    let res = client
        .get("https://meta.fabricmc.net/v2/versions/installer")
        .send()
        .await;

    if let Err(err) = res {
        println!("Request failed: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        println!("Request failed with status: {}", res.status());

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    // Get and parse the response body
    let body = res.json::<Vec<FabricVersion>>().await;
    if let Err(err) = body {
        println!("Failed to parse JSON: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let body = body.unwrap();

    // Get the latest version entry
    let entry = body.first();
    if entry.is_none() {
        println!("Failed to get latest version");

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let latest = entry.unwrap();

    // Get the URL of the latest version
    let url = &latest.url;

    println!("Found latest fabric version {}", latest.version);

    // Download and run the installer
    download_and_execute(client, url.to_string()).await;
}

async fn install_quilt() {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if let Err(err) = client {
        println!("Failed to create http client: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let client = client.unwrap();

    // Download and run the installer
    download_and_execute(
        client,
        "https://quiltmc.org/api/v1/download-latest-installer/java-universal".to_string(),
    )
    .await;
}

async fn install_neoforge() {
    // Create an HTTP client
    let client = reqwest::Client::builder().build();

    if let Err(err) = client {
        println!("Failed to create http client: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let client = client.unwrap();

    // Fetch Neoforge release info
    let res = client
        .get("https://maven.neoforged.net/api/maven/latest/version/releases/net%2Fneoforged%2Fneoforge")
        .send()
        .await;

    if let Err(err) = res {
        println!("Request failed: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        println!("Request failed with status: {}", res.status());

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    // Get and parse the response body
    let body = res.json::<NeoforgeVersion>().await;
    if let Err(err) = body {
        println!("Failed to parse JSON: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let body = body.unwrap();

    // Create the URL for the installer
    let url = format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar", version = body.version);

    // Download and run the installer
    download_and_execute(client, url).await;
}

async fn download_and_execute(client: Client, url: String) {
    // Request the file
    let res = client.get(url).send().await;
    if let Err(err) = res {
        println!("Download failed: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let res = res.unwrap();

    // Check if the request was successful
    if !res.status().is_success() {
        println!("Download failed with status: {}", res.status());

        thread::sleep(Duration::from_millis(2500));
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

    if let Err(err) = child {
        println!("Failed to execute installer: {}", err);

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    println!("Installer started");
    child.unwrap().wait().unwrap();

    println!("Installer finished");

    thread::sleep(Duration::from_millis(2500));
}
