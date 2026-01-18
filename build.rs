const COMMANDS: &[&str] = &[
    "check_battery_optimization_status",
    "request_battery_optimization_exemption",
    "open_battery_settings",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
