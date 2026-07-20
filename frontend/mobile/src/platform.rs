use dioxus::prelude::*;
use jalanaman_shared::EmergencyContact;
use serde::Deserialize;

use crate::app_config::{app_spec, CopyKey, Language};
use crate::models::{GeoPoint, LocationEval};

pub(crate) async fn read_device_hash() -> String {
    let eval = document::eval(
        r#"
        const key = 'ja_device_hash';
        let hash = localStorage.getItem(key);
        if (!hash) {
            hash = (crypto && crypto.randomUUID)
                ? crypto.randomUUID()
                : `mobile-${Date.now()}-${Math.random().toString(36).slice(2)}`;
            localStorage.setItem(key, hash);
        }
        return hash;
        "#,
    );

    eval.await
        .ok()
        .and_then(|value| String::deserialize(&value).ok())
        .unwrap_or_else(|| format!("mobile-{}", uuid::Uuid::new_v4()))
}

pub(crate) async fn read_location() -> Result<GeoPoint, String> {
    let eval = document::eval(
        r#"
        return await new Promise((resolve) => {
            let nativeError = '';
            try {
                if (window.JalanAmanNative && (window.JalanAmanNative.getCurrentLocationJson || window.JalanAmanNative.getLastLocationJson)) {
                    const getter = window.JalanAmanNative.getCurrentLocationJson || window.JalanAmanNative.getLastLocationJson;
                    const native = JSON.parse(getter.call(window.JalanAmanNative));
                    if (Number.isFinite(native.lat) && Number.isFinite(native.lng)) {
                        resolve({ lat: native.lat, lng: native.lng });
                        return;
                    }
                    nativeError = native.error || '';
                    if (/izin|permission|layanan lokasi|location service|gps/i.test(nativeError)) {
                        resolve({ error: nativeError });
                        return;
                    }
                }
            } catch (err) {
                nativeError = err && err.message ? err.message : 'Lokasi native gagal dibaca.';
            }

            if (!navigator.geolocation) {
                resolve({ error: nativeError || 'Geolocation tidak tersedia di perangkat ini.' });
                return;
            }
            navigator.geolocation.getCurrentPosition(
                (pos) => resolve({
                    lat: pos.coords.latitude,
                    lng: pos.coords.longitude,
                }),
                (err) => resolve({ error: nativeError ? `${nativeError}. ${err.message || 'Izin lokasi ditolak.'}` : (err.message || 'Izin lokasi ditolak.') }),
                { enableHighAccuracy: true, timeout: 12000, maximumAge: 15000 }
            );
        });
        "#,
    );

    let value = match eval.await {
        Ok(value) => value,
        Err(err) => {
            return Err(normalize_location_error(format!(
                "Gagal membaca GPS: {err}"
            )))
        }
    };
    let result = LocationEval::deserialize(&value).map_err(|err| err.to_string())?;

    match (result.lat, result.lng) {
        (Some(lat), Some(lng)) if lat.is_finite() && lng.is_finite() => Ok(GeoPoint { lat, lng }),
        _ => {
            Err(normalize_location_error(result.error.unwrap_or_else(
                || "GPS belum memberi koordinat.".to_string(),
            )))
        }
    }
}

fn normalize_location_error(message: String) -> String {
    let lower = message.to_lowercase();

    if lower.contains("permission")
        || lower.contains("denied")
        || lower.contains("ditolak")
        || lower.contains("izin")
    {
        return "Izin lokasi belum aktif. Izinkan lokasi untuk JalanAman, lalu coba lagi."
            .to_string();
    }

    if lower.contains("gps")
        || lower.contains("layanan lokasi")
        || lower.contains("location service")
    {
        return "GPS belum aktif. Nyalakan Lokasi pada HP, lalu coba lagi.".to_string();
    }

    "Lokasi belum tersedia. Pastikan GPS dan koneksi internet aktif, lalu coba lagi.".to_string()
}

pub(crate) async fn request_app_permissions() {
    let _ = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.requestAppPermissionsJson) {
                window.JalanAmanNative.requestAppPermissionsJson();
            }
        } catch (_) {}
        return true;
        "#,
    )
    .await;
}

pub(crate) fn normalize_whatsapp_phone(value: &str) -> Option<String> {
    let mut digits = value
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>();

    if digits.is_empty() {
        return None;
    }

    if digits.starts_with("00") {
        digits = digits.trim_start_matches("00").to_string();
    } else if digits.starts_with('0') {
        digits = format!("62{}", digits.trim_start_matches('0'));
    } else if digits.starts_with('8') {
        digits = format!("62{digits}");
    }

    if digits.len() < 8 {
        None
    } else {
        Some(digits)
    }
}

pub(crate) async fn open_whatsapp_sos(
    contacts: &[EmergencyContact],
    point: GeoPoint,
) -> Result<bool, String> {
    let phone = contacts
        .iter()
        .filter_map(|contact| contact.phone.as_deref())
        .find_map(normalize_whatsapp_phone);

    let Some(phone) = phone else {
        return Ok(false);
    };

    let message = format!(
        "SOS JalanAman: saya butuh bantuan sekarang.\nLokasi saya: https://maps.google.com/?q={},{}",
        point.lat, point.lng
    );
    let phone_json = serde_json::to_string(&phone).map_err(|err| err.to_string())?;
    let message_json = serde_json::to_string(&message).map_err(|err| err.to_string())?;
    let script = format!(
        r#"
        const phone = {phone_json};
        const message = {message_json};
        try {{
            if (window.JalanAmanNative && window.JalanAmanNative.openWhatsAppJson) {{
                const result = JSON.parse(window.JalanAmanNative.openWhatsAppJson(phone, message));
                return !!result.ok;
            }}
            const url = `https://wa.me/${{phone}}?text=${{encodeURIComponent(message)}}`;
            window.location.href = url;
            return true;
        }} catch (err) {{
            return false;
        }}
        "#
    );

    let eval = document::eval(&script);
    let value = eval.await.map_err(|err| err.to_string())?;
    bool::deserialize(&value).map_err(|err| err.to_string())
}

pub(crate) async fn start_sos_alarm() -> Result<(), String> {
    let eval = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.startSosAlarmJson) {
                const result = JSON.parse(window.JalanAmanNative.startSosAlarmJson());
                return !!result.ok;
            }
        } catch (_) {}
        return false;
            "#,
    );

    let started = eval
        .await
        .ok()
        .and_then(|value| bool::deserialize(&value).ok())
        .unwrap_or(false);

    if started {
        Ok(())
    } else {
        Err(app_spec()
            .copy
            .text(Language::Indonesian, CopyKey::SosAlarmPermissionMissing)
            .to_string())
    }
}

pub(crate) async fn is_sos_alarm_active() -> bool {
    let eval = document::eval(
        r#"
        try {
            if (window.JalanAmanNative && window.JalanAmanNative.isSosAlarmActiveJson) {
                const result = JSON.parse(window.JalanAmanNative.isSosAlarmActiveJson());
                return !!result.active;
            }
        } catch (_) {}
        return false;
        "#,
    );

    eval.await
        .ok()
        .and_then(|value| bool::deserialize(&value).ok())
        .unwrap_or(false)
}

pub(crate) fn stop_sos_alarm() {
    spawn(async {
        let _ = document::eval(
            r#"
            try {
                if (window.JalanAmanNative && window.JalanAmanNative.stopSosAlarmJson) {
                    window.JalanAmanNative.stopSosAlarmJson();
                }
            } catch (_) {}
            if (navigator.vibrate) navigator.vibrate(0);
            return true;
            "#,
        )
        .await;
    });
}
