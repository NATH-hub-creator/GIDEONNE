// main.rs — Point d'entrée Tauri Gideonne
// Empêche l'ouverture d'une console sur Windows en production
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    gideonne_lib::run();
}
