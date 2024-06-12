#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::Command};
use log::{info, error};
use simplelog::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;

fn main() {
    let log_file = File::create("./output.log").unwrap();
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

    let current_exe_path = env::current_exe().expect("Failed to get current exe path");
  
    match current_exe_path.to_str() {
        Some(path_str) => info!("The current executable path is: {}", path_str),
        None => error!("The current executable path contains invalid UTF-8 characters and cannot be displayed as a string."),
    }

    // let binary_path = PathBuf::from(current_exe_path.parent().expect("Executable has no parent directory"))
    // .join("../Resources/_up_/bin/azure-openai-proxy");

    let binary_path = PathBuf::from(current_exe_path.parent().expect("Executable has no parent directory"))
    .join("../../../bin/azure-openai-proxy");


   

    match Command::new(binary_path)
        .spawn(){
            Ok(child) => {
                info!("Successfully started go service with pid: {}", child.id());
            }
            Err(e) => {
                error!("Failed to start go service: {}", e);
            }
        };
       

    wait_for_go_service();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn wait_for_go_service() {
    use std::net::TcpStream;
    use std::time::{Duration, Instant};

    let timeout = Duration::from_secs(10);
    let start = Instant::now();
    let mut connected = false;

    while Instant::now().duration_since(start) < timeout && !connected {
        if TcpStream::connect("localhost:7763").is_ok() {
            connected = true;
            println!("Go service has started and is accepting requests.");
            info!("Go service has started and is accepting requests.");
        } else {
            println!("Waiting for Go service to start...");
            info!("Waiting for Go service to start...");
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    if !connected {
        panic!("Could not connect to Go service, timed out.");
    }
}
