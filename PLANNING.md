# JalanAman — Planning & Task Division
**Garuda Hacks 7.0 · Safety & Resilience · 30 jam**

---

## Daftar Isi
1. [Team & Role](#1-team--role)
2. [Arsitektur Singkat](#2-arsitektur-singkat)
3. [API Contract (Backend → semua)](#3-api-contract)
4. [Fitur 1 — Peta Laporan](#4-fitur-1--peta-laporan-keamanan)
5. [Fitur 2 — Lapor Cepat](#5-fitur-2--lapor-cepat)
6. [Fitur 3 — Skor Rute](#6-fitur-3--skor-rute)
7. [Fitur 4 — Tombol SOS](#7-fitur-4--tombol-sos)
8. [Fitur 5 — Dashboard Heatmap](#8-fitur-5--dashboard-web-heatmap)
9. [Shared Components (frontend/shared)](#9-shared-components)
10. [Timeline 30 Jam](#10-timeline-30-jam)
11. [Dependency & Blokade](#11-dependency--blokade)
12. [Environment & Setup](#12-environment--setup)

---

## 1. Team & Role

| Role | Crate / Layer | PIC |
|---|---|---|
| **Backend** | `backend/` — Rust/Axum REST API, PostgreSQL, Redis | _isi nama_ |
| **Web** | `frontend/web/` — Dioxus WASM PWA + Dashboard | _isi nama_ |
| **Mobile (Android)** | `frontend/mobile/` — Dioxus Mobile, Android target | _isi nama_ |
| **iOS** | `frontend/mobile/` — Dioxus Mobile, iOS target | _isi nama_ |

> **Prinsip pembagian:**
> - `frontend/shared/` diisi **bersama** (koordinasi Web + Mobile + iOS). Jangan edit file orang lain tanpa diskusi.
> - Backend adalah **sumber kebenaran data**; semua platform hanya memanggil REST API.
> - Setiap orang buat branch sendiri: `feat/backend-xxx`, `feat/web-xxx`, `feat/mobile-xxx`, `feat/ios-xxx`.

---

## 2. Arsitektur Singkat

```
┌─────────────────────────────────────────────┐
│              Client Layer                    │
│  Web (WASM PWA)  │  Android  │  iOS         │
│  Dioxus + Trunk  │  Dioxus   │  Dioxus      │
└──────────┬───────┴─────┬─────┴──────┬───────┘
           │  REST /api  │            │
           ▼             ▼            ▼
┌─────────────────────────────────────────────┐
│         Backend  (Rust / Axum)               │
│  Reports · RouteScore · SOS · Email         │
└──────────┬──────────────────────────────────┘
           │
   ┌───────┴────────┐
   │  PostgreSQL 16 │  Redis 7
   │  + earthdist.  │  (cooldown, cache)
   └────────────────┘
```

---

## 3. API Contract

> Base URL dev: `http://localhost:8080`  
> Semua response JSON. Error format: `{ "error": "pesan" }`

### 3.1 Config publik
```
GET /api/config
→ { vapid_public_key, google_maps_api_key }
```

### 3.2 Reports
```
POST /api/reports
Body: { category, lat, lng, note?, device_hash }
→ Report

GET /api/reports?lat=&lng=&radius=
→ Report[]

POST /api/reports/:id/upvote
POST /api/reports/:id/downvote
→ Report
```

**Report object:**
```json
{
  "id": "uuid",
  "category": "crime|lighting|accident|other",
  "lat": -6.2088,
  "lng": 106.8456,
  "note": "opsional, max 100 char",
  "device_hash": "string",
  "created_at": "ISO8601",
  "upvote_count": 0,
  "downvote_count": 0,
  "status": "active|hidden"
}
```

### 3.3 Route Score
```
POST /api/route-score
Body: { waypoints: [{lat, lng}] }
→ { score: float, level: "Aman"|"Waspada"|"Hindari", report_count: int, cache_hit: bool }
```

### 3.4 SOS
```
POST /api/sos/trigger
Body: { device_hash, lat, lng, message? }
→ { notified_count, total_contacts, results: [{name, push_sent, email_sent, error?}] }

POST /api/sos/contacts
Body: { device_hash, name, email }
→ EmergencyContact

GET /api/sos/contacts?device_hash=
→ EmergencyContact[]

GET /api/sos/invite/:token
→ { contact_name, already_connected }

POST /api/sos/subscribe
Body: { invite_token, contact_device_hash, push_endpoint, push_p256dh, push_auth }
→ { status: "subscribed" }
```

**EmergencyContact object:**
```json
{
  "id": "uuid",
  "device_hash": "string",
  "name": "string",
  "email": "string",
  "push_endpoint": "string|null",
  "invite_token": "string",
  "created_at": "ISO8601"
}
```

---

## 4. Fitur 1 — Peta Laporan Keamanan

> Pin dari user lain, dikategorikan, muncul dalam radius sekitar user.

### Backend
- [x] `GET /api/reports?lat=&lng=&radius=` — query `earthdistance`, radius default 800m
- [x] Index spasial `ll_to_earth(lat, lng)` di tabel `reports`
- [ ] Pastikan auto-hide laporan status `hidden` tidak muncul di response
- [ ] **Test**: seed 10 laporan, panggil endpoint, pastikan hanya yang dalam radius keluar

### Web (`frontend/web/`)
- [ ] `hooks/use_geolocation.rs` — wrap JS Geolocation, simpan ke Signal
- [ ] Saat lokasi dapat → `api::get_reports(lat, lng, 800)` → set ke state
- [ ] `components/map.rs` → `ja_initMap(lat, lng)` lalu loop `ja_addReportMarker(...)`
- [ ] Reload marker saat user geser peta (optional, bisa skip untuk demo)
- [ ] Tampilkan badge count: "⚠️ N laporan aktif"

### Mobile (Android) & iOS
- [ ] Minta permission `ACCESS_FINE_LOCATION` (Android) / `NSLocationWhenInUseUsageDescription` (iOS)
- [ ] Gunakan native location API → kirim ke `api::get_reports`
- [ ] Render pin di atas native map (MapLibre / Apple Maps / Google Maps SDK)
- [ ] Warna pin per kategori: 🔴 crime, 🟠 accident, 🟡 lighting, ⚪ other

**Pin categories:**

| Value | Label | Warna |
|---|---|---|
| `crime` | Rawan Begal/Copet | Merah 🔴 |
| `accident` | Rawan Kecelakaan | Oranye 🟠 |
| `lighting` | Pencahayaan Buruk | Kuning 🟡 |
| `other` | Lainnya | Abu ⚪ |

---

## 5. Fitur 2 — Lapor Cepat

> 1 tap pilih kategori + geolokasi otomatis + catatan opsional (maks 100 karakter).

### Backend
- [x] `POST /api/reports` — insert ke DB
- [x] Cooldown Redis: 1 laporan per `device_hash` per 10 menit di lokasi sama
- [x] Auto-hide: `downvote_count >= 3` → `status = 'hidden'`
- [ ] **Validasi** di handler: `category` harus salah satu dari enum, `note` ≤ 100 char

### Web
- [ ] Gunakan `shared::components::report_form::ReportForm`
- [ ] Props yang perlu diisi: `lat`, `lng` dari `use_geolocation`, `device_hash` dari `ja_getDeviceHash()`
- [ ] `on_submit` → panggil `services::api::create_report(payload)` → update marker di peta
- [ ] Handle HTTP 429: tampilkan pesan "Tunggu 10 menit..."
- [ ] Animasi loading saat request in-flight

### Mobile & iOS
- [ ] Bottom sheet / modal dengan 4 tombol kategori
- [ ] Geolokasi diisi otomatis dari native location service
- [ ] `device_hash` = UUID yang disimpan di `SharedPreferences` / `UserDefaults`
- [ ] Setelah submit sukses: tampilkan snackbar "Laporan terkirim" + tambah pin ke peta

### Shared (`frontend/shared/`)
- [x] `components/report_form.rs` — UI form sudah ada, terima `on_submit: EventHandler<CreateReportPayload>`
- [ ] Pastikan form di-reset setelah submit sukses (set note & category ke default)

---

## 6. Fitur 3 — Skor Rute

> User input tujuan → Google Directions API → waypoints → score backend → label Aman/Waspada/Hindari.

### Backend
- [x] `POST /api/route-score` — terima `{ waypoints: [{lat, lng}] }`
- [x] Query reports dalam radius 50m dari setiap waypoint (dedup by ID)
- [x] Algoritma skor: `weight = recency × category_weight × community_weight`
- [x] Cache Redis: TTL 10 menit, key = SHA256 dari waypoints
- [ ] **Threshold** skor: `< 5` = Aman, `5–15` = Waspada, `> 15` = Hindari

**Algoritma detail:**
```
recency_weight     = 1.0 - (age_days / 30) × 0.9   (min 0.1)
category_weight    = crime=3.0 | accident=2.0 | lighting=1.0 | other=1.0
community_weight   = 1.0 + (upvote_count × 0.2)
score             += recency × category × community
```

### Web
- [ ] Integrasikan Google Directions API via JS (tambahkan ke `index.html`)
  ```js
  window.ja_getDirections = async function(origin, destination) {
    const service = new google.maps.DirectionsService();
    const result = await service.route({
      origin, destination,
      travelMode: google.maps.TravelMode.WALKING,  // walking lebih relevan
    });
    // Decode overview_polyline → array {lat, lng}
    return JSON.stringify(decodePolyline(result.routes[0].overview_polyline.points));
  };
  ```
- [ ] Tambahkan binding di `utils/js.rs`:
  ```rust
  pub fn get_directions(origin: &str, destination: &str) -> js_sys::Promise;
  ```
- [ ] Di `pages/home.rs` → `on_search` handler:
  1. Panggil `get_directions(origin, dest)`
  2. Parse JSON → `Vec<Waypoint>`
  3. Kirim ke `api::calculate_route_score(waypoints)`
  4. Gambar polyline di peta: `ja_drawRoutePolyline(pts_json, level)`
- [ ] Tampilkan chip hasil: `✅ Aman` / `⚠️ Waspada` / `🚫 Hindari`
- [ ] Gunakan `shared::components::route_score::RouteScorePanel` (sudah ada)

### Mobile & iOS
- [ ] Input field "Tujuan" di screen utama
- [ ] Gunakan Google Maps SDK native `DirectionsAPI` atau platform routing
- [ ] Extract waypoints dari rute → kirim ke `api::calculate_route_score`
- [ ] Overlay warna pada polyline rute di native map (hijau/kuning/merah)

---

## 7. Fitur 4 — Tombol SOS

> Force alert: (a) alarm suara keras di device pengirim + (b) email ke kontak darurat + (c) Web Push ke device kontak.

### Backend — **EMAIL (belum diimplementasi, harus dikerjakan)**

Tambahkan dependency ke `backend/Cargo.toml`:
```toml
lettre = { version = "0.11", features = ["tokio1-rustls-tls", "builder"] }
```

Tambahkan ke `handlers/sos.rs`:
```rust
async fn send_email_alert(
    smtp_user: &str,
    smtp_pass: &str,
    to_email: &str,
    contact_name: &str,
    lat: f64,
    lng: f64,
    message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
```

Tambahkan ke `Config`:
```
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your@gmail.com
SMTP_PASS=app_password
SMTP_FROM=JalanAman SOS <sos@jalanaman.id>
```

Tambahkan kolom `email` ke tabel `emergency_contacts`:
```sql
ALTER TABLE emergency_contacts ADD COLUMN IF NOT EXISTS email TEXT;
```

Atau buat migration baru `002_add_email.sql`.

Email body template:
```
Subject: 🆘 SOS dari [nama kontak] — JalanAman

[nama] menekan tombol SOS!

Pesan: [message]
Lokasi sekarang: https://maps.google.com/?q=[lat],[lng]

Waktu: [created_at WIB]

---
Ini adalah notifikasi otomatis dari JalanAman.
Jika ini tidak relevan, abaikan email ini.
```

Update response `SosTriggerResponse`:
```json
{
  "notified_count": 2,
  "total_contacts": 3,
  "results": [
    { "name": "Ibu", "push_sent": true, "email_sent": true },
    { "name": "Teman", "push_sent": false, "email_sent": true, "error": "push endpoint expired" },
    { "name": "Kakak", "push_sent": false, "email_sent": false, "error": "email tidak diset" }
  ]
}
```

### Backend — Push (sudah ada, cek)
- [x] `web-push` crate, `ContentEncoding::Aes128Gcm`
- [x] Rate limit SOS: 1x per menit per device
- [ ] Pastikan VAPID keys sudah di-generate dan masuk `.env`

### Web
- [ ] Tombol SOS besar di pojok kanan bawah (sudah ada di `shared::components::sos_button`)
- [ ] **Tap 1**: langsung `ja_playSOSAlarm()` (suara + vibrate) — tidak butuh internet
- [ ] **Tap 1 (background)**: kirim `api::trigger_sos(device_hash, lat, lng)`
- [ ] Tampilkan hasil: "N kontak diberitahu (push + email)"
- [ ] **Tap 2**: `ja_stopSOSAlarm()` → set active=false
- [ ] Setup kontak darurat (UI terpisah, halaman `/contacts`):
  - Form: nama + email kontak
  - Setelah submit → backend buat `invite_token`
  - Tampilkan link undangan: `https://app.jalanaman.id/invite/{token}`
  - Kirim link itu ke kontak via WhatsApp/SMS secara manual
  - Status badge: ✅ terhubung (push_endpoint ada) / ⏳ belum buka link

### Mobile (Android)
- [ ] `AudioManager` atau `MediaPlayer` untuk alarm lokal
- [ ] `Vibrator` / `VibrationEffect` untuk getaran pola SOS
- [ ] Notification channel priority HIGH untuk menerima notifikasi (sebagai kontak)
- [ ] FCM integration untuk push (berbeda dari Web Push VAPID — catat ini!)
  > **Catatan**: Android native menggunakan FCM, bukan Web Push VAPID. Backend perlu endpoint tambahan untuk FCM token, atau untuk MVP gunakan email saja di mobile.

### iOS
- [ ] `AVAudioSession` + `AudioServicesPlaySystemSound` untuk alarm
- [ ] `UIFeedbackGenerator` untuk haptic
- [ ] `UNUserNotificationCenter` untuk push notification (APNs)
- [ ] Untuk MVP: cukup andalkan alarm lokal + email; APNs butuh provisioning profile

### Shared
- [x] `components/sos_button.rs` — pure UI, terima `on_sos`, `on_stop` EventHandler
- [ ] Pastikan animasi pulse CSS (`.sos-btn`) terdefinisi di masing-masing platform

---

## 8. Fitur 5 — Dashboard Web Heatmap

> Read-only, satu halaman, heatmap untuk RT/RW / satpam kampus.

### Backend
- [x] `GET /api/reports?lat=&lng=&radius=` dengan radius besar (mis. 50.000m) untuk area luas
- [ ] Pertimbangkan endpoint khusus `GET /api/reports/all` dengan pagination (skip untuk MVP)
- [ ] Pastikan query cepat dengan index `ll_to_earth`

### Web
- [ ] Halaman `/dashboard` sudah ada di `pages/dashboard.rs`
- [ ] Saat mount: fetch laporan dalam radius 50km dari titik pusat (hardcode Jakarta untuk demo)
- [ ] Kirim koordinat ke `ja_initHeatmap(points_json)` → Google Maps `HeatmapLayer`
- [ ] Stat card: jumlah per kategori (crime / accident / lighting / other)
- [ ] Tabel laporan terbaru (max 50 baris)
- [ ] Link "← Kembali" ke halaman utama
- [ ] **Pastikan Google Maps `visualization` library di-load**: `libraries=visualization` di script URL

> Dashboard bisa diakses via URL `/dashboard` — bisa dibuka di laptop stakeholder tanpa install app apapun.

### Mobile & iOS
- [ ] **Tidak perlu** di mobile MVP; cukup web untuk demo stakeholder.

---

## 9. Shared Components

> File di `frontend/shared/src/components/` — **jangan tambah dependensi platform-spesifik di sini.**

| File | Status | Keterangan |
|---|---|---|
| `report_form.rs` | ✅ Ada | Props: lat, lng, device_hash, loading, error, on_close, on_submit |
| `sos_button.rs` | ✅ Ada | Props: active, status_msg, location, on_sos, on_stop |
| `route_score.rs` | ✅ Ada | Props: result, loading, error, on_search |

**Cara pakai di Web:**
```rust
use jalanaman_shared::components::report_form::ReportForm;

// Di RSX:
ReportForm {
    lat: location.read().map(|(la, _)| la),
    lng: location.read().map(|(_, lo)| lo),
    device_hash: get_device_hash(),
    loading: *report_loading.read(),
    error: report_error.read().clone(),
    on_close: move |_| show_report.set(false),
    on_submit: move |payload| { /* panggil api::create_report */ },
}
```

**Cara pakai di Mobile/iOS (sama persis):**
```rust
use jalanaman_shared::components::report_form::ReportForm;
// Props identik, hanya implementasi on_submit berbeda (pakai reqwest, bukan gloo-net)
```

---

## 10. Timeline 30 Jam

```
JAM  00–03  BACKEND               WEB                    MOBILE/iOS
     ────────────────────────────────────────────────────────────────
     Setup: docker compose up     Setup: trunk serve      Setup: dx serve / Xcode
     sqlx migrate run             Cek render peta kosong  Cek build target jalan
     Seed demo data               Cek geolokasi browser   Cek location permission

JAM  03–10  Fitur 1 + 2           Fitur 1 + 2            Fitur 1
     ────────────────────────────────────────────────────────────────
     GET /api/reports ✓           Map + pin marker        Native map + pin
     POST /api/reports ✓          ReportForm component    ReportForm component
     Cooldown Redis ✓             Lapor cepat flow        Lapor cepat flow

JAM  10–16  Fitur 3               Fitur 3                Fitur 2 lanjut
     ────────────────────────────────────────────────────────────────
     POST /api/route-score ✓      Directions API JS       Fitur 3 (routing)
     Scoring algorithm ✓          RouteScorePanel UI      Score overlay peta
     Redis cache ✓                Polyline overlay

JAM  16–22  Fitur 4 (SOS)         Fitur 4 (SOS)          Fitur 4 (SOS)
     ────────────────────────────────────────────────────────────────
     Email (lettre crate) ←KRITIS Web Push subscribe      Alarm lokal native
     Push notification ✓          Alarm lokal (Web Audio) Kontak darurat UI
     Rate limit ✓                 Kontak darurat UI       Email sebagai fallback

JAM  22–26  Fitur 5 + Polish       Fitur 5 Dashboard      Polish
     ────────────────────────────────────────────────────────────────
     Endpoint sudah cukup         Heatmap layer           Bug fix
     Anti-spam review             Stat cards              UI polish

JAM  26–28  Deploy                 Build release          Build release
     ────────────────────────────────────────────────────────────────
     docker compose up --build    trunk build --release   dx build --release
     VPS / Fly.io / Railway       Copy dist ke nginx      APK / IPA untuk demo

JAM  28–30  BUFFER + SUBMISSION
     ────────────────────────────────────────────────────────────────
     Demo video 2 menit · Devpost submission · README final
```

---

## 11. Dependency & Blokade

```
Backend selesai GET /api/reports
    ↓ unblocks
    Web: render pin di peta
    Mobile: render pin di peta
    iOS: render pin di peta

Backend selesai POST /api/reports
    ↓ unblocks
    Web: form lapor cepat bisa dicoba end-to-end
    Mobile: idem
    iOS: idem

Backend selesai POST /api/route-score
    ↓ unblocks
    Web: route scoring (perlu juga Google Maps Directions JS)
    Mobile: route scoring

Backend selesai POST /api/sos/trigger (email + push)
    ↓ unblocks
    Web: demo SOS end-to-end (butuh 2 device)
    Mobile: demo SOS end-to-end

Web selesai /invite/:token page
    ↓ unblocks
    Demo push notification ke kontak darurat
```

**Blokade kritis yang harus selesai paling awal:**
1. ⚡ Backend: `GET /api/reports` + `POST /api/reports` → **jam 0–5**
2. ⚡ Backend: Email SOS (`lettre` crate) → **jam 16–18** (paling sering lupa)
3. ⚡ Web: Google Maps API key aktif + script loaded → **jam 0–2**
4. ⚡ Mobile/iOS: Location permission + native map rendering → **jam 0–3**

---

## 12. Environment & Setup

### Backend
```bash
cp .env.example .env
# Isi: DATABASE_URL, REDIS_URL, GOOGLE_MAPS_API_KEY
# Isi: VAPID_PUBLIC_KEY, VAPID_PRIVATE_KEY_PEM (dari scripts/generate_vapid_keys.sh)
# Isi: SMTP_HOST, SMTP_PORT, SMTP_USER, SMTP_PASS

docker compose up postgres redis -d
cd backend && sqlx migrate run
cargo watch -x run
```

### Web
```bash
# Install tools (sekali saja)
rustup target add wasm32-unknown-unknown
cargo install trunk

cd frontend/web
trunk serve        # dev server port 3000, proxy ke backend 8080
```

### Mobile (Android)
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install dioxus-cli

cd frontend/mobile
dx serve --platform android   # atau dx build --platform android
```

### iOS
```bash
rustup target add aarch64-apple-ios
# Butuh macOS + Xcode terinstall

cd frontend/mobile
dx serve --platform ios
# atau: dx build --platform ios → buka .xcodeproj di Xcode
```

### Google Maps API Key
1. Buka [console.cloud.google.com](https://console.cloud.google.com)
2. Enable: **Maps JavaScript API**, **Directions API**, **Geocoding API**
3. Buat API Key, restrict ke domain demo
4. Masukkan ke `.env` sebagai `GOOGLE_MAPS_API_KEY`

### VAPID Keys (untuk Web Push SOS)
```bash
./scripts/generate_vapid_keys.sh
# Copy output ke .env
```

### SMTP (untuk Email SOS)
Gunakan Gmail App Password:
1. Google Account → Security → 2-Step Verification aktif
2. App passwords → buat password untuk "Mail"
3. Masukkan ke `.env` sebagai `SMTP_PASS`

---

## Checklist Submission Garuda Hacks

- [ ] Repo GitHub **public** sejak jam pertama
- [ ] Commit pertama kosong/init di jam ke-0
- [ ] Minimal **3 commit** selama event
- [ ] README ada **AI usage disclosure**
- [ ] Video demo **maks 2 menit** di-upload ke Devpost
- [ ] Link deploy aktif (atau demo live saat presentasi)
- [ ] Devpost submission sebelum deadline

---

## Catatan Jujur untuk Juri (siapkan jawaban ini)

| Pertanyaan Juri | Jawaban |
|---|---|
| "Email bisa di-bypass silent mode?" | Tidak, tapi email pasti terkirim ke inbox. Alarm suara lokal di HP pengirim langsung bunyi tanpa butuh izin siapapun. |
| "Laporan bisa dipalsukan?" | Cooldown 10 menit per device, downvote auto-hide, tidak ada nama — hanya lokasi & kategori. |
| "Beda dari SafetiPin / bSafe?" | Fokus konteks Indonesia (rawan begal/motor, area gelap sekitar kos/kampus). SOS 2 channel (alarm lokal + email/push). Dashboard RT/RW. |
| "Kalau kontak belum buka link undangan?" | Alarm lokal tetap bunyi. Email tetap terkirim (tidak butuh kontak install apapun). Push hanya bonus jika sudah terhubung. |
| "Skalabilitas?" | PostgreSQL + earthdistance handle ratusan ribu laporan. Redis cache skor rute. Stateless backend, bisa di-scale horizontal. |
