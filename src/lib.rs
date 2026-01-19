use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Runtime,
};

mod commands;
mod error;

pub use commands::*;
pub use error::Error;

/// Battery optimization status returned by the plugin.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryStatus {
    /// Whether the app is subject to battery optimization (Doze mode restrictions).
    /// `true` means the app may have delayed background execution.
    pub is_optimized: bool,
    /// Whether the app is ignoring battery optimizations (unrestricted background usage).
    /// `true` means the app has unrestricted background access.
    pub is_ignoring_optimizations: bool,
}

pub struct AndroidBatteryOptimization<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidBatteryOptimization<R> {
    /// Check the current battery optimization status.
    #[cfg(target_os = "android")]
    pub fn check_status(&self) -> Result<BatteryStatus, Error> {
        self.0
            .run_mobile_plugin("checkBatteryOptimizationStatus", ())
            .map_err(|e| Error::PluginInvoke(e.to_string()))
    }

    /// Request exemption from battery optimization (shows system dialog).
    #[cfg(target_os = "android")]
    pub fn request_exemption(&self) -> Result<(), Error> {
        self.0
            .run_mobile_plugin("requestBatteryOptimizationExemption", ())
            .map_err(|e| Error::PluginInvoke(e.to_string()))
    }

    /// Open the battery optimization settings page.
    #[cfg(target_os = "android")]
    pub fn open_settings(&self) -> Result<(), Error> {
        self.0
            .run_mobile_plugin("openBatterySettings", ())
            .map_err(|e| Error::PluginInvoke(e.to_string()))
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-battery-optimization")
        .invoke_handler(tauri::generate_handler![
            commands::check_battery_optimization_status,
            commands::request_battery_optimization_exemption,
            commands::open_battery_settings
        ])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            {
                let handle = api.register_android_plugin(
                    "com.plugin.android_battery_optimization",
                    "BatteryOptimizationPlugin",
                )?;
                app.manage(AndroidBatteryOptimization(handle));
            }
            let _ = (app, api);
            Ok(())
        })
        .build()
}
