import { invoke } from '@tauri-apps/api/core';

const COMMAND = {
  CHECK_BATTERY_OPTIMIZATION_STATUS:
    'plugin:android-battery-optimization|check_battery_optimization_status',
  REQUEST_BATTERY_OPTIMIZATION_EXEMPTION:
    'plugin:android-battery-optimization|request_battery_optimization_exemption',
  OPEN_BATTERY_SETTINGS:
    'plugin:android-battery-optimization|open_battery_settings',
};

/**
 * Battery optimization status.
 */
export interface BatteryStatus {
  /**
   * Whether the app is subject to battery optimization (Doze mode restrictions).
   * `true` means the app may have delayed background execution.
   */
  isOptimized: boolean;
  /**
   * Whether the app is ignoring battery optimizations (unrestricted background usage).
   * `true` means the app has unrestricted background access.
   */
  isIgnoringOptimizations: boolean;
}

/**
 * Check the current battery optimization status.
 *
 * @returns A promise that resolves to the battery optimization status.
 *
 * @example
 * ```typescript
 * import { checkBatteryOptimizationStatus } from "tauri-plugin-android-battery-optimization-api";
 *
 * const status = await checkBatteryOptimizationStatus();
 * if (status.isOptimized) {
 *   console.log("App is subject to battery optimization");
 * }
 * ```
 */
export const checkBatteryOptimizationStatus = (): Promise<BatteryStatus> => {
  return invoke<BatteryStatus>(COMMAND.CHECK_BATTERY_OPTIMIZATION_STATUS);
};

/**
 * Request exemption from battery optimization.
 *
 * This will show a system dialog asking the user to allow the app
 * to run without battery optimization restrictions (unrestricted background usage).
 *
 * @returns A promise that resolves when the dialog is shown.
 *
 * @example
 * ```typescript
 * import { requestBatteryOptimizationExemption } from "tauri-plugin-android-battery-optimization-api";
 *
 * await requestBatteryOptimizationExemption();
 * ```
 */
export const requestBatteryOptimizationExemption = (): Promise<void> => {
  return invoke(COMMAND.REQUEST_BATTERY_OPTIMIZATION_EXEMPTION);
};

/**
 * Open the system battery optimization settings page.
 *
 * This allows users to manually configure battery optimization
 * for all installed apps.
 *
 * @returns A promise that resolves when the settings page is opened.
 *
 * @example
 * ```typescript
 * import { openBatterySettings } from "tauri-plugin-android-battery-optimization-api";
 *
 * await openBatterySettings();
 * ```
 */
export const openBatterySettings = (): Promise<void> => {
  return invoke(COMMAND.OPEN_BATTERY_SETTINGS);
};
