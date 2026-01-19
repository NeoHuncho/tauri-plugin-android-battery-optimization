use tauri::Runtime;

use crate::{BatteryStatus, Error};

/// Check the current battery optimization status.
///
/// Returns a `BatteryStatus` object with:
/// - `isOptimized`: `true` if the app is subject to battery optimization
/// - `isIgnoringOptimizations`: `true` if the app has unrestricted background access
#[tauri::command]
pub fn check_battery_optimization_status<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<BatteryStatus, Error> {
    #[cfg(target_os = "android")]
    {
        app.state::<AndroidBatteryOptimization<R>>().check_status()
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(BatteryStatus {
            is_optimized: false,
            is_ignoring_optimizations: true,
        })
    }
}

/// Request exemption from battery optimization.
///
/// This will show a system dialog asking the user to allow the app
/// to run without battery optimization restrictions.
#[tauri::command]
pub fn request_battery_optimization_exemption<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        app.state::<AndroidBatteryOptimization<R>>()
            .request_exemption()
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(())
    }
}

/// Open the system battery optimization settings page.
///
/// This allows users to manually configure battery optimization
/// for all installed apps.
#[tauri::command]
pub fn open_battery_settings<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        app.state::<AndroidBatteryOptimization<R>>().open_settings()
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(())
    }
}
