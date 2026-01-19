# tauri-plugin-android-battery-optimization

> This plugin only works on Tauri v2 for Android.

Tauri plugin for managing Android battery optimization settings. Request unrestricted background usage to ensure timely notifications and background task execution.

## Why Use This Plugin?

Android's Doze mode and battery optimization can delay background tasks and notifications. This plugin allows you to:

- **Check** if your app is subject to battery optimization
- **Request** exemption via the system dialog (unrestricted background usage)
- **Opens a modal (see below) directly in your app**

<img width="377" height="225" alt="Screenshot 2026-01-19 at 17 48 53" src="https://github.com/user-attachments/assets/ed8f290c-c671-485c-aab1-4923a0fd4ae3" />

## Install

### Rust (Cargo)

```bash
cargo add tauri-plugin-android-battery-optimization
```

### JavaScript Guest Bindings

Install using your preferred JavaScript package manager:

```bash
pnpm add tauri-plugin-android-battery-optimization-api
# or
npm install tauri-plugin-android-battery-optimization-api
# or
yarn add tauri-plugin-android-battery-optimization-api
```

## Usage

### 1. Register the Plugin

`src-tauri/src/lib.rs`

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_android_battery_optimization::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Add Permissions

`src-tauri/capabilities/default.json`

```json
{
  "permissions": ["android-battery-optimization:default"]
}
```

### 3. Use the JavaScript API

```typescript
import {
  checkBatteryOptimizationStatus,
  requestBatteryOptimizationExemption,
  openBatterySettings,
} from 'tauri-plugin-android-battery-optimization-api';

// Check if the app is subject to battery optimization
const status = await checkBatteryOptimizationStatus();

if (status.isOptimized) {
  console.log('App is subject to battery optimization');

  // Request exemption (shows system dialog)
  await requestBatteryOptimizationExemption();
}

// Or open the battery settings page for manual configuration
await openBatterySettings();
```

## API Reference

### `checkBatteryOptimizationStatus()`

Check the current battery optimization status.

**Returns:** `Promise<BatteryStatus>`

```typescript
interface BatteryStatus {
  // true if the app is subject to battery optimization (Doze mode restrictions)
  isOptimized: boolean;
  // true if the app has unrestricted background access
  isIgnoringOptimizations: boolean;
}
```

### `requestBatteryOptimizationExemption()`

Request exemption from battery optimization. Shows a system dialog asking the user to allow unrestricted background usage.

**Returns:** `Promise<void>`

### `openBatterySettings()`

Open the system battery optimization settings page. Allows users to manually configure battery optimization for all apps.

**Returns:** `Promise<void>`

## Android Manifest

The plugin automatically adds the required permission to your Android manifest:

```xml
<uses-permission android:name="android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS" />
```

## Compatibility

- **Minimum Android SDK:** 21 (Android 5.0)
- **Target Android SDK:** 34 (Android 14)
- **Tauri:** v2

> **Note:** Battery optimization (Doze mode) was introduced in Android 6.0 (API 23). On older versions, `isOptimized` will always return `false`.

## Example Use Case

Show a banner prompting users to enable unrestricted background usage for timely notifications:

```typescript
import {
  checkBatteryOptimizationStatus,
  requestBatteryOptimizationExemption,
} from 'tauri-plugin-android-battery-optimization-api';

const status = await checkBatteryOptimizationStatus();

if (status.isOptimized) {
  // Show a UI banner
  showBanner({
    message: 'Enable unrestricted background usage for timely notifications',
    onPress: async () => {
      await requestBatteryOptimizationExemption();
      // Re-check status after user interaction
      const newStatus = await checkBatteryOptimizationStatus();
      if (!newStatus.isOptimized) {
        hideBanner();
      }
    },
  });
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.
