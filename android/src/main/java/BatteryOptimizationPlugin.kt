package com.plugin.android_battery_optimization

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.PowerManager
import android.provider.Settings
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class BatteryOptimizationPlugin(private val activity: Activity) : Plugin(activity) {

    /**
     * Check the current battery optimization status.
     * Returns whether the app is subject to battery optimization and if it's ignoring optimizations.
     */
    @Command
    fun checkBatteryOptimizationStatus(invoke: Invoke) {
        val result = JSObject()
        
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            val powerManager = activity.getSystemService(Context.POWER_SERVICE) as PowerManager
            val packageName = activity.packageName
            val isIgnoring = powerManager.isIgnoringBatteryOptimizations(packageName)
            result.put("isOptimized", !isIgnoring)
            result.put("isIgnoringOptimizations", isIgnoring)
        } else {
            // Battery optimization (Doze mode) was introduced in Android 6.0 (API 23)
            // For earlier versions, report as not optimized
            result.put("isOptimized", false)
            result.put("isIgnoringOptimizations", true)
        }
        
        invoke.resolve(result)
    }

    /**
     * Request exemption from battery optimization.
     * Shows a system dialog asking the user to allow unrestricted background usage.
     */
    @Command
    fun requestBatteryOptimizationExemption(invoke: Invoke) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            try {
                val intent = Intent(Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS)
                intent.data = Uri.parse("package:${activity.packageName}")
                activity.startActivity(intent)
                invoke.resolve()
            } catch (e: Exception) {
                invoke.reject("Failed to open battery optimization dialog: ${e.message}")
            }
        } else {
            // Not needed for older Android versions
            invoke.resolve()
        }
    }

    /**
     * Open the system battery optimization settings page.
     * Allows users to manually configure battery optimization for all apps.
     */
    @Command
    fun openBatterySettings(invoke: Invoke) {
        try {
            val intent = Intent(Settings.ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS)
            activity.startActivity(intent)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Failed to open battery settings: ${e.message}")
        }
    }
}
