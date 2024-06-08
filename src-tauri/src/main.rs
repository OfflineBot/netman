
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_network() -> Vec<String> {
    println!("getting network");
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID", "dev", "wifi"])
        .output()
        .expect("Couldnt get WiFi Networks!");

    if !output.status.success() {
        println!("couldnt get any network connections");
        return Vec::new();
    }

    let networks: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .split('\n')
        .map(|f| f.to_string())
        .collect();

    let mut output_vec: Vec<String> = Vec::new();

    for item in &networks {
        if item.is_empty() {
            continue;
        }
        if output_vec.contains(item) {
            continue;
        }
        output_vec.push(item.to_string());
    }

    println!("Got network");
    output_vec
}

#[tauri::command]
fn connect_network(name: &str, password: &str) -> String {
    println!("connecting to network");
    if password.trim() == "" {
        let output = Command::new("nmcli")
            .args(["device", "wifi", "connect", name])
            .output()
            .expect("Couldnt connect to wifi");

        if !output.status.success() {
            return String::from("FAILED");
        }

        return String::from("SUCCESS");
    }

    let output = Command::new("nmcli")
        .args(["device", "wifi", "connect", name, "password", password])
        .output()
        .expect("Couldnt connect to wifi");

        if !output.status.success() {
            return String::from("BASH_FAILED");
        }

        return String::from("SUCCESS");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_network,
            connect_network,
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
