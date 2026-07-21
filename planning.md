# JalanAman - Planning Fitur Lanjutan Android

File ini adalah planning baru setelah review kode pada 2026-07-21. `PLANNING.md` lama tetap dipakai sebagai dokumen MVP awal, sedangkan file ini fokus ke kebutuhan juri: voice SOS, widget Android, dan pilih/rekomendasi rute.

## 1. Ringkasan Kondisi Kode Saat Ini

### Arsitektur

- Backend: Rust + Axum di `backend/`.
- Shared contract dan komponen: `frontend/shared/`.
- Web: Dioxus WASM PWA di `frontend/web/`.
- Mobile Android/iOS: Dioxus mobile di `frontend/mobile/`.
- Android native layer saat ini dibuat lewat script patch `frontend/mobile/android/patch-generated-android.sh`, bukan file Kotlin permanen di source tree.

### Fitur yang sudah ada

- Report map:
  - `GET /api/reports`
  - `POST /api/reports`
  - validasi kategori, validasi note maksimal 100 karakter, cooldown Redis, auto-hide laporan downvote.
- Heatmap:
  - `GET /api/reports/heatmap`.
- Route:
  - `GET /api/places` untuk saran tempat.
  - `GET /api/directions` untuk geocode + OSRM route.
  - `POST /api/route-score` untuk skor Aman/Waspada/Hindari.
  - Mobile sudah punya `RouteView`, input tujuan, suggestions, polyline map, skor, jarak, estimasi.
- SOS:
  - `POST /api/sos/trigger`.
  - Kontak darurat email/WhatsApp/push.
  - Backend sudah punya email alert via `lettre`.
  - Android bridge sudah punya alarm lokal, vibration, foreground service, WhatsApp fallback, dan baca lokasi native.

### Gap utama

- Voice trigger belum ada.
- Widget Android belum ada.
- Pilih rute belum benar-benar "rekomendasi"; saat ini backend hanya mengembalikan satu route OSRM, lalu route itu discoring.
- Native Android masih di-generate lewat patch script, jadi penambahan voice/widget harus dibuat disiplin agar tidak hilang saat Dioxus regenerate project.
- Shared `SosTriggerResponse` hanya membaca `notified_count` dan `total_contacts`, padahal backend sudah mengembalikan detail `results`. Untuk demo, detail ini sebaiknya dipakai.

## 2. Prioritas Fitur

Prioritas saya:

1. Voice SOS.
2. Android SOS widget.
3. Rekomendasi beberapa rute.
4. Polish demo dan reliability.

Alasannya: voice dan widget langsung menjawab request juri dan sangat mudah didemokan. Rekomendasi rute adalah value utama produk, tetapi butuh perubahan backend + UI yang lebih besar.

## 3. Fitur A - Voice SOS

### Tujuan

User bisa memicu SOS memakai suara, misalnya:

- "JalanAman SOS"
- "tolong"
- "bantuan"
- "aktifkan SOS"

### Scope MVP

- Voice hanya aktif saat aplikasi terbuka.
- Tambah tombol mic di dekat tombol SOS atau di header.
- Saat user tekan mic, Android membuka speech recognizer.
- Hasil transkrip dikirim ke Dioxus lewat JavaScript bridge.
- Jika transkrip cocok keyword darurat, jalankan flow SOS yang sudah ada:
  - ambil lokasi,
  - start alarm,
  - trigger backend SOS,
  - buka WhatsApp fallback jika perlu.

### Scope lanjutan

- Mode "listen while app open" dengan indikator jelas.
- Voice confirmation untuk menghindari false trigger:
  - app mendeteksi keyword,
  - tampil countdown 3 detik,
  - user bisa batal,
  - jika tidak batal, SOS aktif.
- Quick command selain SOS:
  - "lapor lampu mati"
  - "lapor begal"
  - "cari rute aman ke kampus"

### Implementasi teknis

#### Android native

Edit `frontend/mobile/android/patch-generated-android.sh` agar menambahkan:

- Permission:
  - `RECORD_AUDIO`
- Kotlin bridge method:
  - `startVoiceCommandJson()`
  - `stopVoiceCommandJson()`
- Android `SpeechRecognizer` atau `RecognizerIntent`.
- Callback hasil suara ke WebView:
  - `window.dispatchEvent(new CustomEvent("jalanaman-voice-command", { detail: { text } }))`

Catatan: karena native Android project di-generate oleh Dioxus, semua file Kotlin tambahan perlu tetap dibuat dari patch script atau nanti dipindah ke template/source native permanen bila Dioxus mendukung.

#### Dioxus mobile

Tambahkan di `frontend/mobile/src/platform.rs`:

- `start_voice_command() -> Result<(), String>`
- listener event `jalanaman-voice-command`.

Tambahkan state di `frontend/mobile/src/app.rs`:

- `voice_listening: bool`
- `voice_last_text: Option<String>`
- `voice_error: Option<String>`

Refactor flow SOS saat ini menjadi function internal agar bisa dipanggil dari:

- tombol SOS,
- voice trigger,
- widget/deep link.

Nama yang disarankan:

- `run_sos_flow(...)`
- `trigger_sos_from_source(source: SosSource)`

#### UI

Tambahkan mic control:

- Lokasi: dekat floating SOS button atau di header.
- State:
  - idle: "Voice SOS"
  - listening: "Mendengarkan..."
  - matched: countdown "SOS aktif dalam 3..."
  - failed: "Perintah tidak dikenali"

### Acceptance criteria

- User tap mic, ucap "SOS", alarm menyala dan request backend terkirim.
- Jika keyword tidak cocok, SOS tidak aktif.
- Jika permission mic belum ada, app meminta permission.
- Jika backend mati, alarm lokal tetap aktif dan WhatsApp fallback tetap dicoba.

## 4. Fitur B - Android SOS Widget

### Tujuan

User bisa menekan SOS dari home screen Android tanpa membuka aplikasi penuh dulu.

### Scope MVP

- App widget ukuran 1x1 atau 2x1.
- Tombol besar "SOS".
- Saat ditekan:
  - buka aplikasi ke flow SOS,
  - atau langsung start native `SosAlarmService`,
  - lalu app mengirim request backend setelah activity terbuka dan lokasi siap.

Untuk MVP paling aman: widget membuka app dengan intent extra `jalanaman_action=sos`, lalu Dioxus menjalankan flow SOS. Ini mengurangi risiko lokasi/background permission.

### Scope lanjutan

- Widget 2x2:
  - tombol SOS,
  - status kontak,
  - lokasi terakhir,
  - tombol "Saya aman".
- Quick report widget:
  - "Lampu mati"
  - "Rawan"
  - "Kecelakaan"
- Lock screen/notification shortcut jika memungkinkan.

### Implementasi teknis

Edit `frontend/mobile/android/patch-generated-android.sh` agar menghasilkan:

- `JalanAmanSosWidgetProvider.kt`
- `res/xml/jalanaman_sos_widget.xml`
- `res/layout/jalanaman_sos_widget.xml`
- manifest receiver:
  - `android.appwidget.action.APPWIDGET_UPDATE`
- PendingIntent ke `MainActivity` dengan extra:
  - `jalanaman_action=sos`

Tambahkan bridge:

- `consumeLaunchActionJson()`
  - mengembalikan `{ "action": "sos" }` sekali, lalu clear.

Di `frontend/mobile/src/platform.rs`:

- `read_launch_action() -> Option<String>`

Di `frontend/mobile/src/app.rs`:

- saat mount, panggil `read_launch_action()`.
- jika action `sos`, jalankan flow SOS yang sama dengan tombol utama.

### Acceptance criteria

- Setelah install APK, widget JalanAman muncul di launcher.
- Widget dapat ditambahkan ke home screen.
- Tekan widget membuka app dan memicu SOS.
- Alarm lokal aktif walau backend gagal.
- Tidak ada double trigger saat app dibuka ulang.

## 5. Fitur C - Pilih Rute dan Rekomendasi Rute

### Tujuan

User tidak hanya melihat satu rute, tetapi bisa memilih dari beberapa opsi:

- Rute direkomendasikan
- Rute tercepat
- Rute paling sepi laporan
- Rute hindari titik rawan

### Masalah saat ini

`GET /api/directions` hanya mengambil satu `OsrmRoute`. `POST /api/route-score` sudah bisa menilai satu polyline, tetapi belum ada endpoint untuk membandingkan beberapa rute.

### Data model baru

Tambahkan shared type di `frontend/shared/src/utils/types.rs`:

```rust
pub struct RouteOption {
    pub id: String,
    pub label: String,
    pub polyline: Vec<Waypoint>,
    pub waypoints: Vec<Waypoint>,
    pub distance_m: f64,
    pub duration_s: f64,
    pub score: f64,
    pub level: String,
    pub report_count: usize,
    pub recommendation_reason: String,
}

pub struct RouteRecommendationsResponse {
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub provider: String,
    pub recommended_route_id: String,
    pub routes: Vec<RouteOption>,
}
```

### Backend

Buat endpoint baru:

```text
GET /api/route-recommendations?origin_lat=&origin_lng=&destination=&mode=walking
```

Tahap backend:

1. Reuse geocode dari `backend/src/handlers/directions.rs`.
2. Ambil beberapa alternatif rute dari OSRM jika tersedia.
3. Jika provider hanya memberi satu rute, buat alternatif sederhana:
   - route utama,
   - route via titik geser kiri,
   - route via titik geser kanan,
   - lalu gabungkan origin -> via -> destination.
4. Untuk setiap rute:
   - sample waypoints,
   - hitung route score,
   - hitung penalti jarak/durasi.
5. Ranking:
   - safety paling penting,
   - lalu durasi,
   - lalu jarak.

Formula awal:

```text
rank_score = safety_score + duration_penalty + distance_penalty
duration_penalty = max(0, (duration_s - fastest_duration_s) / 60) * 0.35
distance_penalty = max(0, (distance_m - shortest_distance_m) / 100) * 0.15
```

Route dengan `rank_score` paling kecil menjadi rekomendasi.

### Mobile UI

Update `RouteView`:

- Setelah search tujuan, tampilkan list opsi rute.
- Setiap opsi punya:
  - label,
  - badge Aman/Waspada/Hindari,
  - jarak,
  - estimasi,
  - jumlah laporan,
  - alasan rekomendasi singkat.
- Tap opsi rute mengganti polyline di map.
- Opsi rekomendasi diberi badge "Direkomendasikan".

State baru di `frontend/mobile/src/app.rs`:

- `route_options: Vec<RouteOption>`
- `selected_route_id: Option<String>`
- `route_recommendations_loading`
- `route_recommendations_error`

Service baru di `frontend/mobile/src/services.rs`:

- `get_route_recommendations(origin, destination)`.

### Acceptance criteria

- Search tujuan menampilkan minimal 2 opsi jika alternatif tersedia.
- App otomatis memilih rute yang direkomendasikan.
- User bisa tap opsi lain dan peta berubah.
- Rute dengan banyak laporan rawan tidak direkomendasikan walau lebih cepat.
- Jika endpoint rekomendasi gagal, fallback ke flow lama `GET /api/directions` + `POST /api/route-score`.

## 6. Fitur Tambahan yang Saya Sarankan

### 6.1 Safety Check-In

User memilih kontak dan durasi, misalnya 15 menit. Jika user tidak menekan "Saya aman" sebelum waktu habis, app otomatis mengirim SOS ringan ke kontak.

Nilai demo: sangat kuat untuk skenario pulang malam.

### 6.2 Share Live Trip

Saat mengikuti rute, user bisa share link lokasi real-time ke kontak selama perjalanan. Kontak bisa melihat posisi dan status rute dari web.

Nilai demo: memperjelas manfaat kontak darurat, bukan hanya saat panik.

### 6.3 Panic Gesture

Trigger SOS dari gesture:

- tekan tombol power 3 kali,
- shake phone,
- long press tombol volume.

Catatan: ini butuh native Android lebih dalam dan harus hati-hati agar tidak false trigger.

### 6.4 Mode Senyap

Tidak semua kondisi aman untuk alarm keras. Tambahkan pilihan:

- SOS keras: alarm + getar + kontak.
- SOS senyap: tanpa alarm, langsung kirim lokasi ke kontak.

Nilai produk: lebih realistis untuk kasus user merasa diikuti.

### 6.5 Trust Score Laporan

Tambahkan kredibilitas laporan:

- laporan baru butuh konfirmasi komunitas,
- laporan yang banyak downvote disembunyikan,
- laporan yang sering valid punya bobot lebih tinggi.

Backend sudah punya upvote/downvote, tinggal dikembangkan ke scoring dan UI.

### 6.6 Safe Place dan Pos Bantuan

Tampilkan titik aman:

- kantor polisi,
- rumah sakit,
- minimarket 24 jam,
- pos security kampus,
- tempat ramai.

Di route recommendation, beri bonus jika rute melewati safe place.

### 6.7 Demo Mode

Tambahkan toggle demo:

- lokasi demo Jakarta,
- seeded reports,
- kontak dummy,
- route demo yang sudah pasti beda skor.

Nilai hackathon: presentasi lebih stabil walau internet/GPS venue bermasalah.

## 7. Rencana Sprint

### Sprint 1 - Refactor SOS flow

- Extract flow SOS di `frontend/mobile/src/app.rs` supaya bisa dipanggil dari tombol, voice, dan widget.
- Update `SosTriggerResponse` shared agar punya `results`.
- Tambahkan copy/status untuk sumber SOS: tombol, voice, widget.
- Test manual tombol SOS tetap bekerja.

### Sprint 2 - Voice SOS MVP

- Tambah permission `RECORD_AUDIO`.
- Tambah native speech bridge di patch Android.
- Tambah listener event voice di Dioxus.
- Tambah tombol mic dan matching keyword.
- Tambah countdown cancel 3 detik.
- Test dengan keyword Indonesia.

### Sprint 3 - Android Widget MVP

- Generate widget provider, layout, xml metadata, manifest receiver.
- Buat intent action `sos`.
- Tambah bridge untuk consume launch action.
- Hubungkan launch action ke flow SOS.
- Test add widget, tap widget, stop alarm.

### Sprint 4 - Route Recommendations

- Tambah type `RouteOption` dan response recommendations.
- Tambah endpoint backend `/api/route-recommendations`.
- Reuse route scoring untuk setiap opsi.
- Update mobile service.
- Update `RouteView` dengan list opsi.
- Tambah fallback ke flow lama.

### Sprint 5 - Polish Demo

- Tambah Demo Mode.
- Tambah status kontak lebih detail di SOS overlay.
- Tambah loading/error yang jelas untuk voice dan widget.
- Siapkan skenario demo:
  - report map,
  - cari rute,
  - pilih rute aman,
  - voice SOS,
  - widget SOS.

## 8. Risiko dan Mitigasi

| Risiko | Dampak | Mitigasi |
|---|---|---|
| Speech recognizer tidak tersedia di beberapa device | Voice tidak jalan | Fallback ke tombol SOS dan tampilkan pesan jelas |
| False trigger voice | SOS aktif tidak sengaja | Keyword spesifik + countdown cancel |
| Widget background location dibatasi Android | Lokasi SOS gagal dari widget | Widget buka app dulu, app ambil lokasi foreground |
| Native patch Dioxus rapuh | File Android hilang saat regenerate | Semua native file tetap dibuat idempotent di `patch-generated-android.sh` |
| OSRM tidak selalu memberi alternatif | Opsi rute kurang banyak | Fallback generate via waypoint sederhana |
| GPS/internet venue demo buruk | Demo gagal | Demo Mode dengan koordinat dan data seed |

## 9. Checklist Demo untuk Juri

- Buka app Android.
- Tunjukkan peta dan laporan sekitar.
- Cari tujuan, tampilkan beberapa rute.
- Pilih rute yang direkomendasikan dan jelaskan alasan: lebih sedikit laporan rawan.
- Ucap "JalanAman SOS" atau "tolong".
- Tunjukkan countdown, lalu alarm lokal menyala.
- Tunjukkan kontak menerima email/WhatsApp fallback.
- Stop alarm.
- Tambahkan widget ke home screen.
- Tekan widget SOS dan tunjukkan flow yang sama berjalan.

## 10. Rekomendasi Urutan Implementasi

Kalau waktunya sempit, kerjakan dalam urutan ini:

1. Refactor SOS flow agar reusable.
2. Voice SOS MVP.
3. Widget SOS MVP.
4. Route recommendations.
5. Demo Mode.

Kalau hanya punya 1 hari, targetkan voice SOS + widget SOS dulu karena paling sesuai feedback juri dan paling terlihat saat demo.
