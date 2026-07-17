# JalanAman

**Garuda Hacks 7.0 — Track: Safety & Resilience**

> "Tahu mana yang aman sebelum melangkah"

Aplikasi mobile PWA + dashboard web berbasis laporan komunitas (*crowd-sourced*) yang membantu pejalan kaki dan pengguna transportasi umum di Indonesia mengetahui tingkat keamanan suatu rute, lengkap dengan tombol SOS satu-tap yang membunyikan alarm keras di device dan mengirim Web Push + email ke kontak darurat.

---

## Fitur MVP

| Fitur | Deskripsi |
|---|---|
| 🗺 **Peta Laporan** | Pin report komunitas (pencahayaan buruk / rawan begal / kecelakaan) dalam radius 500m — Leaflet + OpenStreetMap, gratis |
| ⚠️ **Lapor Cepat** | 1 tap → pilih kategori → kirim dengan geolokasi otomatis, tanpa akun |
| 📊 **Skor Rute** | Input tujuan → routing via OSRM (gratis) → overlay skor **Aman / Waspada / Hindari** |
| 🆘 **Tombol SOS** | Alarm suara keras + vibrate lokal (instan) + Web Push + email ke kontak darurat |
| 🏘 **Dashboard** | Heatmap laporan per wilayah untuk RT/RW, satpam kampus, kepolisian |

### Batasan yang diketahui (disclosed)

- Push notification ke kontak darurat adalah **best-effort**: tunduk pada setting notifikasi perangkat penerima dan status koneksi. Bukan *critical alert* yang bypass DND.
- Kontak darurat harus membuka link undangan dan mengizinkan notifikasi **sebelum** SOS diperlukan.
- Alarm suara **lokal** di device pengirim selalu bunyi — tidak butuh koneksi internet.
- Alarm berbasis email memerlukan rule Tasker/MacroDroid di HP kontak. Gunakan pemicu notifikasi Gmail yang memuat `[JALANAMAN-SOS]`, lalu atur aksi memainkan alarm, menyalakan layar, dan mengulang getaran. Isi email juga memiliki marker `JALANAMAN_SOS_TRIGGER=1` agar mudah difilter.

---

## Tech Stack

| Layer | Teknologi |
|---|---|
| Backend | Rust · Axum 0.7 · Tokio |
| Frontend Web | Dioxus 0.6 (WASM PWA) · `dx serve` |
| Frontend Mobile | Dioxus 0.6 · `dx serve --platform android/ios` |
| CSS | Tailwind CSS v3 (build via npm) |
| Database | PostgreSQL 16 · `earthdistance` + `cube` extensions |
| Cache | Redis 7 |
| Peta & Routing | Leaflet 1.9.4 · OpenStreetMap · OSRM · Nominatim — **gratis, tanpa API key** |
| Push | Web Push / VAPID (`web-push` crate) |
| Email SOS | SMTP via `lettre` (Gmail App Password) |
| Container | Docker & Docker Compose |
| Reverse proxy | nginx |

---

## Prasyarat

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Dioxus CLI 0.6.x sesuai lockfile project
cargo install dioxus-cli --version 0.6.3 --locked

# sqlx CLI (migrasi database)
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# cargo-watch (hot reload backend)
cargo install cargo-watch

# Node.js >= 18 (untuk build Tailwind CSS)
node --version
cd frontend/web && npm ci && cd ../..

# Docker & Docker Compose
docker --version
```

---

## Setup & Menjalankan

### 1. Clone & environment

```bash
git clone https://github.com/YOUR_USERNAME/JalanAman
cd JalanAman
cp .env.example .env
# Edit .env: isi VAPID keys, SMTP credentials
```

### 2. Generate VAPID keys

```bash
./scripts/generate_vapid_keys.sh
# Salin VAPID_PUBLIC_KEY dan VAPID_PRIVATE_KEY_PEM ke .env
```

### 3. Jalankan infrastruktur (DB + Redis)

```bash
make dev-infra         # docker compose up postgres redis -d
cd backend && sqlx migrate run
```

### 4. Dev mode (3 terminal)

```bash
# Terminal 1 – backend (hot reload)
make dev-backend       # cargo watch -x run di port 8080

# Terminal 2 – frontend web
make dev-web           # build Tailwind + dx serve, lalu buka Chrome

# Terminal 3 – mobile Android WSL (opsional)
make dev-android       # wrapper android/serve-android-wsl.sh
```

Kalau jalan manual, masuk ke folder targetnya:

```bash
cd frontend/web
bash serve-web-wsl.sh  # buka Chrome di http://localhost:8080

cd ../mobile
dx serve               # shell wrapper menambahkan --platform android
```

### 5. Full Docker stack

```bash
docker compose up -d --build
# Buka http://localhost
```

### 6. Seed demo data

```bash
make seed
```

---

## Environment Variables

| Variabel | Keterangan |
|---|---|
| `DATABASE_URL` | Connection string PostgreSQL |
| `REDIS_URL` | Connection string Redis |
| `PORT` | Port backend (default `8080`) |
| `VAPID_PUBLIC_KEY` | Base64url public key untuk Web Push |
| `VAPID_PRIVATE_KEY_PEM` | PEM private key untuk Web Push |
| `SMTP_HOST` / `SMTP_PORT` | Server SMTP (default: `smtp.gmail.com:587`) |
| `SMTP_USER` / `SMTP_PASS` | Kredensial SMTP (Gmail: gunakan App Password) |
| `SMTP_FROM` | Alamat pengirim email SOS |

> **Tidak perlu API key peta** — peta, routing, dan geocoding menggunakan Leaflet + OpenStreetMap + OSRM (semua gratis).

---

## REST API

```
GET  /health

# Laporan keamanan
POST /api/reports                       Buat laporan baru
GET  /api/reports?lat=&lng=&radius=     Laporan dalam radius (meter)
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

# Skor rute
POST /api/route-score                   Hitung skor keamanan rute
     body: { waypoints: [{lat, lng}] }
     → { level: "Aman"|"Waspada"|"Hindari", score: f64, ... }

# SOS & kontak darurat
POST   /api/sos/trigger                 Kirim SOS (alarm + push + email)
GET    /api/sos/contacts?device_hash=   Daftar kontak darurat
POST   /api/sos/contacts                Tambah kontak darurat
DELETE /api/sos/contacts/:id            Hapus kontak darurat
GET    /api/sos/invite/:token           Info undangan (publik)
POST   /api/sos/subscribe               Daftarkan push subscription kontak
```

---

## Arsitektur Anti-Spam / Anti-Abuse

- **Cooldown laporan**: 1 laporan per device per 10 menit di lokasi yang sama (Redis TTL)
- **Auto-hide**: laporan dengan ≥3 downvote otomatis disembunyikan
- **Decay laporan**: bobot laporan menurun linier selama 30 hari
- **Rate limit SOS**: maks 1 trigger per menit per device (Redis TTL)
- **Privasi**: tidak menyimpan nama asli, hanya `device_hash`; lokasi SOS hanya dikirim ke kontak yang didaftarkan sendiri

---

## Struktur Proyek

```
JalanAman/
├── backend/                   Rust/Axum REST API
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── routes.rs
│   │   ├── models/            report.rs · emergency_contact.rs
│   │   └── handlers/          reports · route_score · sos · health
│   └── migrations/
│       └── 001_init.sql
├── frontend/
│   ├── shared/                Types & komponen bersama (Dioxus)
│   │   └── src/
│   │       ├── components/    report_form · route_score · sos_button
│   │       └── utils/types.rs
│   ├── web/                   PWA Web (dx serve → WASM)
│   │   ├── src/
│   │   │   ├── main.rs        Entry point dx
│   │   │   ├── app.rs         Router & routes
│   │   │   ├── pages/         home · dashboard · contacts · invite
│   │   │   ├── components/    map
│   │   │   ├── hooks/         use_geolocation
│   │   │   ├── services/      api · push
│   │   │   └── utils/         js · device
│   │   ├── public/
│   │   │   ├── sw.js          Service Worker (PWA + push)
│   │   │   └── manifest.json
│   │   ├── assets/
│   │   │   └── tailwind.css   Input Tailwind (→ tw.css saat build)
│   │   ├── index.html         Shell HTML + Leaflet CDN + JS helpers
│   │   └── Dioxus.toml
│   └── mobile/                Android/iOS (dx serve --platform android/ios)
│       ├── src/main.rs
│       ├── android/           Script SDK/NDK + wrapper Android WSL
│       └── Dioxus.toml
├── nginx/nginx.conf
├── docker-compose.yml
├── Makefile
└── scripts/
    ├── generate_vapid_keys.sh
    └── seed_demo.sql
```

---

## AI Usage Disclosure (Garuda Hacks)

Sesuai aturan Garuda Hacks 7.0, berikut bagian yang dibantu AI (Claude):

- **Scaffold infrastruktur awal** (struktur proyek, Cargo.toml, docker-compose.yml, migration SQL)
- **Template handler Axum** (boilerplate CRUD dan error handling)
- **Template komponen Dioxus** (struktur RSX, prop types)
- **Service Worker dan JS helper** (Web Push, Web Audio API, Geolocation, Leaflet integration)

Semua logika bisnis utama (algoritma skor rute, mekanisme anti-spam, alur undangan kontak darurat) dirancang oleh tim dan diimplementasikan/diverifikasi sendiri.

---

## Tim

<!-- Isi nama anggota tim di sini -->

---

## Lisensi

MIT License — lihat [LICENSE](LICENSE)
