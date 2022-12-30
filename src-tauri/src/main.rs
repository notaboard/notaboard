use std::fs::File;
use std::io::Write;
use percent_encoding::{percent_decode};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn export_board(packed_board: String, human_filename: String, ustamp_filename: String) {
    let mut path1: String = "/home/u/Box/".to_owned();
    let mut path0: String = "/home/u/Box0/".to_owned();
    path1.push_str(&human_filename);
    path0.push_str(&ustamp_filename);
    println!("{ }", ustamp_filename);
    let packed_board_decoded = percent_decode(packed_board.as_bytes()).decode_utf8().unwrap();
    let mut output1 = File::create(path1).expect("file creation failed, s.20");
    output1.write_all(packed_board_decoded.as_bytes()).expect("write of file failed, s.21");
    let mut output0 = File::create(path0).expect("file creation failed, s.22");
    output0.write_all(packed_board_decoded.as_bytes()).expect("write of file failed, s.23");
    println!("board was saved: { }", human_filename);
}


fn main() {
    let _app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![export_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
