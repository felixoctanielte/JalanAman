# JalanAman

**Garuda Hacks 7.0 — Track: Safety & Resilience**

> "Google Maps tapi tahu mana yang aman"

Aplikasi mobile PWA + dashboard web berbasis laporan komunitas (*crowd-sourced*) yang membantu pejalan kaki dan pengguna transportasi umum di Indonesia mengetahui tingkat keamanan suatu rute, lengkap dengan tombol SOS satu-tap yang membunyikan alarm keras di device dan mengirim Web Push ke kontak darurat.

---

## Fitur MVP

| Fitur | Deskripsi |
|---|---|
| 🗺 **Peta Laporan** | Pin report dari komunitas (pencahayaan buruk / rawan begal / kecelakaan) dalam radius 500m |
| ⚠️ **Lapor Cepat** | 1 tap → pilih kategori → kirim dengan geolokasi otomatis, tanpa akun |
| 📊 **Skor Rute** | Input tujuan → overlay skor **Aman / Waspada / Hindari** di atas Google Directions |
| 🆘 **Tombol SOS** | Alarm suara keras + vibrate lokal (instan) + Web Push ke kontak darurat (best-effort) |
| 🏘 **Dashboard** | Heatmap laporan per wilayah untuk RT/RW, satpam kampus, kepolisian |

### Batasan yang diketahui (disclosed)

- Push notification ke kontak darurat adalah **best-effort**: tunduk pada setting notifikasi perangkat penerima dan status koneksi. Bukan *critical alert* yang bypass DND (butuh entitlement khusus Apple).  
- Kontak darurat harus membuka link undangan dan mengizinkan notifikasi **sebelum** SOS diperlukan.  
- Alarm suara **lokal** di device pengirim selalu bunyi (tidak butuh koneksi/consent orang lain).

---

## Tech Stack

| Layer | Teknologi |
|---|---|
| Backend | Rust (Axum 0.7 + Tokio) |
| Frontend | Dioxus 0.5 (WASM PWA) + Trunk |
| Database | PostgreSQL 16 + `earthdistance` extension |
| Cache | Redis 7 |
| Maps | Google Maps JavaScript API + Directions API |
| Push | Web Push / VAPID (`web-push` crate) |
| Container | Docker & Docker Compose |
| Reverse proxy | nginx |

---

## Prasyarat

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Tools
cargo install trunk sqlx-cli cargo-watch

# Docker & Docker Compose (sudah terinstall)
docker --version
```

---

## Setup & Menjalankan

### 1. Clone & environment

```bash
git clone https://github.com/YOUR_USERNAME/JalanAman
cd JalanAman
cp .env.example .env
# Edit .env: isi GOOGLE_MAPS_API_KEY dan VAPID keys
```

### 2. Generate VAPID keys

```bash
./scripts/generate_vapid_keys.sh
# Salin output ke .env
```

### 3. Jalankan infrastruktur (DB + Redis)

```bash
docker compose up postgres redis -d
# Tunggu healthy, lalu:
cd backend && sqlx migrate run
```

### 4. Dev mode

```bash
# Terminal 1 – backend
cd backend && cargo watch -x run

# Terminal 2 – frontend
cd frontend && trunk serve
# Buka http://localhost:3000
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

## REST API

```
GET  /health
GET  /api/config                        → VAPID public key + Maps key (untuk frontend)

POST /api/reports                       → Buat laporan baru
GET  /api/reports?lat=&lng=&radius=     → Laporan dalam radius (meter)
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

POST /api/route-score                   → Hitung skor keamanan rute
     body: { waypoints: [{lat, lng}] }

POST /api/sos/trigger                   → Kirim SOS (alarm + push)
GET  /api/sos/contacts?device_hash=     → Daftar kontak darurat
POST /api/sos/contacts                  → Tambah kontak darurat
GET  /api/sos/invite/:token             → Info undangan (publik)
POST /api/sos/subscribe                 → Daftarkan push subscription kontak
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
├── backend/                  # Rust/Axum REST API
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── error.rs
│   │   ├── routes.rs
│   │   ├── models/           # report.rs, emergency_contact.rs
│   │   └── handlers/         # reports, route_score, sos, health
│   └── migrations/
│       └── 001_init.sql
├── frontend/                 # Dioxus WASM PWA
│   ├── src/
│   │   ├── main.rs
│   │   ├── api.rs
│   │   ├── js.rs             # wasm_bindgen bindings ke JS helpers
│   │   ├── types.rs
│   │   ├── components/       # map, report_form, sos_button, route_score
│   │   └── pages/            # home, dashboard, invite
│   ├── public/
│   │   ├── sw.js             # Service Worker (PWA + push)
│   │   └── manifest.json
│   └── index.html            # Entry + Google Maps + JS helpers
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
- **Service Worker dan JS helper** (Web Push, Web Audio API, Geolocation)

Semua logika bisnis utama (algoritma skor rute, mekanisme anti-spam, alur undangan kontak darurat) dirancang oleh tim dan diimplementasikan/diverifikasi sendiri.

---

## Tim

<!-- Isi nama anggota tim di sini -->

---

## Lisensi

MIT License – lihat [LICENSE](LICENSE)
