#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MOBILE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ANDROID_APP_DIR="$MOBILE_DIR/target/dx/jalanaman_mobile/debug/android/app"
MANIFEST="$ANDROID_APP_DIR/app/src/main/AndroidManifest.xml"
WEB_VIEW="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/RustWebView.kt"
WEB_CHROME_CLIENT="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/RustWebChromeClient.kt"
LOCATION_BRIDGE="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/JalanAmanLocationBridge.kt"

if [ ! -f "$MANIFEST" ]; then
  echo "Android generated manifest belum ada, skip patch permission."
  exit 0
fi

add_permission() {
  local permission="$1"
  local line="    <uses-permission android:name=\"android.permission.$permission\" />"
  local tmp_manifest

  if grep -q "android.permission.$permission" "$MANIFEST"; then
    return
  fi

  tmp_manifest="$(mktemp)"
  awk -v insert_line="$line" '
    /<application[[:space:]>]/ && inserted == 0 {
      print insert_line
      inserted = 1
    }
    { print }
  ' "$MANIFEST" > "$tmp_manifest"
  mv "$tmp_manifest" "$MANIFEST"
}

add_permission "ACCESS_COARSE_LOCATION"
add_permission "ACCESS_FINE_LOCATION"
add_permission "ACCESS_NETWORK_STATE"
add_permission "POST_NOTIFICATIONS"
add_permission "VIBRATE"

if [ -f "$WEB_CHROME_CLIENT" ]; then
  sed -i '/super\.onGeolocationPermissionsShowPrompt(origin, callback)/d' "$WEB_CHROME_CLIENT"
fi

cat > "$LOCATION_BRIDGE" <<'KOTLIN'
package dev.dioxus.main

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.location.Location
import android.location.LocationManager
import android.net.Uri
import android.os.Build
import android.os.CancellationSignal
import android.webkit.JavascriptInterface
import org.json.JSONObject
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executor
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference

class JalanAmanLocationBridge(private val context: Context) {
  @JavascriptInterface
  fun getCurrentLocationJson(): String {
    return try {
      requestRuntimePermissionsIfPossible()
      if (!hasLocationPermission()) {
        return JSONObject().put("error", "Izin lokasi sedang diminta. Pilih Izinkan saat digunakan lalu tekan Refresh GPS.").toString()
      }

      val manager = context.getSystemService(Context.LOCATION_SERVICE) as LocationManager
      val location = readCurrentLocation(manager) ?: readLastKnownLocation(manager)
      if (location == null) {
        JSONObject().put("error", "Lokasi native belum tersedia.").toString()
      } else {
        JSONObject()
          .put("lat", location.latitude)
          .put("lng", location.longitude)
          .put("provider", location.provider ?: "android")
          .toString()
      }
    } catch (ex: Exception) {
      JSONObject().put("error", ex.message ?: "Gagal membaca lokasi native.").toString()
    }
  }

  @JavascriptInterface
  fun openWhatsAppJson(phone: String, message: String): String {
    return try {
      val normalizedPhone = phone.filter { it.isDigit() }
      if (normalizedPhone.isBlank()) {
        return JSONObject().put("ok", false).put("error", "Nomor WhatsApp kosong.").toString()
      }

      val uri = Uri.parse("https://wa.me/$normalizedPhone")
        .buildUpon()
        .appendQueryParameter("text", message)
        .build()
      val intent = Intent(Intent.ACTION_VIEW, uri).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      context.startActivity(intent)
      JSONObject().put("ok", true).toString()
    } catch (ex: ActivityNotFoundException) {
      JSONObject().put("ok", false).put("error", "WhatsApp tidak ditemukan.").toString()
    } catch (ex: Exception) {
      JSONObject().put("ok", false).put("error", ex.message ?: "Gagal membuka WhatsApp.").toString()
    }
  }

  private fun hasLocationPermission(): Boolean {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) return true

    return context.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED ||
      context.checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
  }

  private fun requestRuntimePermissionsIfPossible() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) return
    val activity = context as? Activity ?: return

    val permissions = mutableListOf<String>()
    if (!hasLocationPermission()) {
      permissions.add(Manifest.permission.ACCESS_FINE_LOCATION)
      permissions.add(Manifest.permission.ACCESS_COARSE_LOCATION)
    }
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
      context.checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED
    ) {
      permissions.add(Manifest.permission.POST_NOTIFICATIONS)
    }

    if (permissions.isEmpty()) return

    activity.requestPermissions(
      permissions.toTypedArray(),
      6201
    )
  }

  @SuppressLint("MissingPermission")
  private fun readLastKnownLocation(manager: LocationManager): Location? =
    locationProviders(manager)
      .mapNotNull { provider -> runCatching { manager.getLastKnownLocation(provider) }.getOrNull() }
      .maxByOrNull { it.time }

  @SuppressLint("MissingPermission")
  private fun readCurrentLocation(manager: LocationManager): Location? {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) return null

    val providers = locationProviders(manager).take(2)
    if (providers.isEmpty()) return null

    val signal = CancellationSignal()
    val latch = CountDownLatch(1)
    val executor = Executor { command -> command.run() }
    val current = AtomicReference<Location?>(null)

    providers.forEach { provider ->
      runCatching {
        manager.getCurrentLocation(provider, signal, executor) { location ->
          if (location != null && current.compareAndSet(null, location)) {
            latch.countDown()
          }
        }
      }
    }

    latch.await(6, TimeUnit.SECONDS)
    signal.cancel()
    return current.get()
  }

  private fun locationProviders(manager: LocationManager): List<String> {
    val providers = mutableListOf<String>()
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
      providers.add(LocationManager.FUSED_PROVIDER)
    }
    providers.add(LocationManager.GPS_PROVIDER)
    providers.add(LocationManager.NETWORK_PROVIDER)
    providers.add(LocationManager.PASSIVE_PROVIDER)

    return providers.distinct().filter { provider ->
      runCatching { manager.isProviderEnabled(provider) }.getOrDefault(false)
    }
  }
}
KOTLIN

if [ -f "$WEB_VIEW" ] && ! grep -q "JalanAmanNative" "$WEB_VIEW"; then
  sed -i '/settings\.javaScriptCanOpenWindowsAutomatically = true/a\        addJavascriptInterface(JalanAmanLocationBridge(context), "JalanAmanNative")' "$WEB_VIEW"
fi

echo "Android generated project patched: network, location, notification, vibration permissions and WhatsApp bridge ready."
