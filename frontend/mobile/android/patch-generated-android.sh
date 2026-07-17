#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MOBILE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ANDROID_APP_DIR="$MOBILE_DIR/target/dx/jalanaman_mobile/debug/android/app"
MANIFEST="$ANDROID_APP_DIR/app/src/main/AndroidManifest.xml"
WEB_VIEW="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/RustWebView.kt"
WEB_CHROME_CLIENT="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/RustWebChromeClient.kt"
WRY_ACTIVITY="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/WryActivity.kt"
LOCATION_BRIDGE="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/JalanAmanLocationBridge.kt"
SOS_SERVICE="$ANDROID_APP_DIR/app/src/main/kotlin/dev/dioxus/main/SosAlarmService.kt"
COLORS_XML="$ANDROID_APP_DIR/app/src/main/res/values/colors.xml"
STYLES_XML="$ANDROID_APP_DIR/app/src/main/res/values/styles.xml"

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
add_permission "FOREGROUND_SERVICE"
add_permission "FOREGROUND_SERVICE_MEDIA_PLAYBACK"

add_sos_service() {
  local tmp_manifest

  if grep -q 'SosAlarmService' "$MANIFEST"; then
    return
  fi

  tmp_manifest="$(mktemp)"
  awk '
    /<\/application>/ {
      print "        <service android:name=\"dev.dioxus.main.SosAlarmService\" android:exported=\"false\" android:foregroundServiceType=\"mediaPlayback\" />"
    }
    { print }
  ' "$MANIFEST" > "$tmp_manifest"
  mv "$tmp_manifest" "$MANIFEST"
}

add_sos_service

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
import android.os.Looper
import android.webkit.JavascriptInterface
import org.json.JSONObject
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executor
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference

class JalanAmanLocationBridge(private val context: Context) {
  @JavascriptInterface
  fun requestAppPermissionsJson(): String {
    return try {
      requestRuntimePermissionsIfPossible()
      JSONObject().put("ok", true).toString()
    } catch (ex: Exception) {
      JSONObject().put("ok", false).put("error", ex.message ?: "Izin aplikasi belum dapat diminta.").toString()
    }
  }

  @JavascriptInterface
  fun getCurrentLocationJson(): String {
    return try {
      requestRuntimePermissionsIfPossible()
      if (!hasLocationPermission()) {
        return JSONObject().put("error", "Izin lokasi sedang diminta. Pilih Izinkan saat digunakan lalu tekan Refresh GPS.").toString()
      }

      val manager = context.getSystemService(Context.LOCATION_SERVICE) as LocationManager
      if (!isLocationServiceEnabled(manager)) {
        return JSONObject().put("error", "Layanan lokasi atau GPS belum aktif di HP.").toString()
      }
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

  @JavascriptInterface
  fun startSosAlarmJson(): String {
    return try {
      requestRuntimePermissionsIfPossible()
      val intent = Intent(context, SosAlarmService::class.java).setAction(SosAlarmService.ACTION_START)
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
        context.startForegroundService(intent)
      } else {
        context.startService(intent)
      }
      JSONObject().put("ok", true).toString()
    } catch (ex: Exception) {
      JSONObject().put("ok", false).put("error", ex.message ?: "Alarm SOS tidak dapat dimulai.").toString()
    }
  }

  @JavascriptInterface
  fun stopSosAlarmJson(): String {
    return try {
      val intent = Intent(context, SosAlarmService::class.java).setAction(SosAlarmService.ACTION_STOP)
      context.startService(intent)
      JSONObject().put("ok", true).toString()
    } catch (ex: Exception) {
      JSONObject().put("ok", false).put("error", ex.message ?: "Alarm SOS tidak dapat dihentikan.").toString()
    }
  }

  @JavascriptInterface
  fun isSosAlarmActiveJson(): String =
    JSONObject().put("active", SosAlarmService.isAlarmActive()).toString()

  private fun hasLocationPermission(): Boolean {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) return true

    return context.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED ||
      context.checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
  }

  private fun requestRuntimePermissionsIfPossible() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) return
    val activity = context as? Activity ?: return

    val request = Runnable {
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

      if (permissions.isNotEmpty()) {
        activity.requestPermissions(permissions.toTypedArray(), 6201)
      }
    }

    if (Looper.myLooper() == Looper.getMainLooper()) {
      request.run()
    } else {
      activity.runOnUiThread(request)
    }
  }

  @SuppressLint("MissingPermission")
  private fun readLastKnownLocation(manager: LocationManager): Location? =
    locationProviders(manager)
      .mapNotNull { provider -> runCatching { manager.getLastKnownLocation(provider) }.getOrNull() }
      .maxByOrNull { it.time }

  @SuppressLint("MissingPermission")
  private fun readCurrentLocation(manager: LocationManager): Location? {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) return null

    val providers = locationProviders(manager).take(3)
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

  private fun isLocationServiceEnabled(manager: LocationManager): Boolean {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
      return manager.isLocationEnabled
    }
    return runCatching {
      manager.isProviderEnabled(LocationManager.GPS_PROVIDER) ||
        manager.isProviderEnabled(LocationManager.NETWORK_PROVIDER)
    }.getOrDefault(false)
  }
}
KOTLIN

cat > "$SOS_SERVICE" <<'KOTLIN'
package dev.dioxus.main

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.media.AudioAttributes
import android.media.MediaPlayer
import android.media.RingtoneManager
import android.os.Build
import android.os.IBinder
import android.os.VibrationEffect
import android.os.Vibrator

class SosAlarmService : Service() {
  companion object {
    const val ACTION_START = "dev.dioxus.main.SOS_START"
    const val ACTION_STOP = "dev.dioxus.main.SOS_STOP"
    private const val CHANNEL_ID = "jalanaman_sos_alarm"
    private const val NOTIFICATION_ID = 6202
    @Volatile private var alarmActive = false

    fun isAlarmActive(): Boolean = alarmActive
  }

  private var player: MediaPlayer? = null
  private var vibrator: Vibrator? = null

  override fun onBind(intent: Intent?): IBinder? = null

  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    if (intent?.action == ACTION_STOP) {
      stopAlarm()
      @Suppress("DEPRECATION")
      stopForeground(true)
      stopSelf()
      return START_NOT_STICKY
    }

    startForeground(NOTIFICATION_ID, buildNotification())
    startAlarm()
    alarmActive = true
    return START_STICKY
  }

  override fun onDestroy() {
    stopAlarm()
    super.onDestroy()
  }

  private fun buildNotification(): Notification {
    val manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      val channel = NotificationChannel(
        CHANNEL_ID,
        "Alarm SOS JalanAman",
        NotificationManager.IMPORTANCE_HIGH,
      ).apply {
        description = "Peringatan SOS yang sedang aktif"
        setSound(null, null)
        enableVibration(false)
      }
      manager.createNotificationChannel(channel)
    }

    val stopIntent = Intent(this, SosAlarmService::class.java).setAction(ACTION_STOP)
    val stopPendingIntent = PendingIntent.getService(
      this,
      1,
      stopIntent,
      PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
    )
    val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      Notification.Builder(this, CHANNEL_ID)
    } else {
      Notification.Builder(this)
    }

    return builder
      .setSmallIcon(android.R.drawable.ic_dialog_alert)
      .setContentTitle("SOS JalanAman aktif")
      .setContentText("Alarm sedang berbunyi. Tekan Hentikan untuk mematikan.")
      .setCategory(Notification.CATEGORY_ALARM)
      .setOngoing(true)
      .setOnlyAlertOnce(true)
      .addAction(Notification.Action.Builder(null, "Hentikan", stopPendingIntent).build())
      .build()
  }

  private fun startAlarm() {
    if (player == null) {
      val alarmUri = RingtoneManager.getDefaultUri(RingtoneManager.TYPE_ALARM)
        ?: RingtoneManager.getDefaultUri(RingtoneManager.TYPE_NOTIFICATION)
      player = MediaPlayer().apply {
        setAudioAttributes(
          AudioAttributes.Builder()
            .setUsage(AudioAttributes.USAGE_ALARM)
            .setContentType(AudioAttributes.CONTENT_TYPE_SONIFICATION)
            .build(),
        )
        setDataSource(this@SosAlarmService, alarmUri)
        isLooping = true
        prepare()
        start()
      }
    }

    vibrator = getSystemService(VIBRATOR_SERVICE) as? Vibrator
    vibrator?.let { deviceVibrator ->
      if (!deviceVibrator.hasVibrator()) return@let
      val pattern = longArrayOf(0, 800, 220, 800, 220)
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
        deviceVibrator.vibrate(VibrationEffect.createWaveform(pattern, 0))
      } else {
        @Suppress("DEPRECATION")
        deviceVibrator.vibrate(pattern, 0)
      }
    }
  }

  private fun stopAlarm() {
    alarmActive = false
    vibrator?.cancel()
    vibrator = null
    player?.runCatching {
      if (isPlaying) stop()
      release()
    }
    player = null
  }
}
KOTLIN

if [ -f "$WEB_VIEW" ] && ! grep -q "JalanAmanNative" "$WEB_VIEW"; then
  sed -i '/settings\.javaScriptCanOpenWindowsAutomatically = true/a\        addJavascriptInterface(JalanAmanLocationBridge(context), "JalanAmanNative")' "$WEB_VIEW"
fi

# Dioxus 0.6 initializes its Android app trampoline once per process. Keep the existing task alive
# when Android Back is pressed so reopening JalanAman cannot create a second root in that process.
if [ -f "$WRY_ACTIVITY" ] && ! grep -q "moveTaskToBack(true)" "$WRY_ACTIVITY"; then
  sed -i '/return super\.onKeyDown(keyCode, event)/i\        if (keyCode == KeyEvent.KEYCODE_BACK) {\
            moveTaskToBack(true)\
            return true\
        }' "$WRY_ACTIVITY"
fi

# Let the keyboard resize the WebView instead of covering form fields (report note, contact inputs).
if [ -f "$MANIFEST" ] && ! grep -q "windowSoftInputMode" "$MANIFEST"; then
  sed -i 's/android:name="dev\.dioxus\.main\.MainActivity">/android:name="dev.dioxus.main.MainActivity" android:windowSoftInputMode="adjustResize">/' "$MANIFEST"
fi

if [ -f "$MANIFEST" ] && ! grep -q 'android:launchMode="singleTask"' "$MANIFEST"; then
  sed -i 's/android:name="dev\.dioxus\.main\.MainActivity"/android:name="dev.dioxus.main.MainActivity" android:launchMode="singleTask"/' "$MANIFEST"
fi

# Brand the Android chrome: default Dioxus template ships a generic teal/pink Material theme,
# so the system status bar clashes with JalanAman's blue UI and the app reads as unfinished/boxed
# instead of full-bleed. AppCompat tints android:statusBarColor from colorPrimaryDark automatically.
if [ -f "$COLORS_XML" ]; then
  cat > "$COLORS_XML" <<'XML'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="colorPrimary">#1D4ED8</color>
    <color name="colorPrimaryDark">#0F3D91</color>
    <color name="colorAccent">#2563EB</color>
</resources>
XML
fi

if [ -f "$STYLES_XML" ]; then
  cat > "$STYLES_XML" <<'XML'
<resources>

    <!-- JalanAman theme: blue brand colors so the system status bar matches the in-app chrome. -->
    <style name="AppTheme" parent="@style/Theme.AppCompat.Light.NoActionBar">
        <item name="colorPrimary">@color/colorPrimary</item>
        <item name="colorPrimaryDark">@color/colorPrimaryDark</item>
        <item name="colorAccent">@color/colorAccent</item>
        <item name="android:windowBackground">@color/colorPrimaryDark</item>
    </style>
</resources>
XML
fi

echo "Android generated project patched: network, location, notification, vibration permissions, WhatsApp bridge, keyboard resize, and JalanAman brand theme ready."
