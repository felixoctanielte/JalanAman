# Android WSL

Project ini pakai Dioxus `0.6.3`, jadi `dx` juga harus versi `0.6.3`.

Cara paling cepat di WSL:

```bash
cd frontend/mobile/android
bash install-android-sdk-wsl.sh
source ~/.bashrc
```

Script itu akan menginstall Android command-line tools Linux, SDK Platform 33, Build Tools 33.0.2, NDK 28.2.13676358, Rust Android targets, dan `dioxus-cli` 0.6.3.

Kalau SDK/NDK sudah ada tapi env WSL bermasalah setelah restart:

```bash
cd frontend/mobile
bash android/repair-android-env-wsl.sh
source ~/.bashrc
```

Manual install `dx` saja:

```bash
cargo install dioxus-cli --version 0.6.3 --locked --force --jobs 1
hash -r
dx --version
```

Run dari WSL di folder `frontend/mobile`, bukan dari folder `frontend/mobile/android`:

```bash
cd frontend/mobile
dx serve
```

Mobile sekarang mengambil data real dari backend:

- `GET /api/reports` untuk pin laporan sekitar user
- `POST /api/reports` untuk lapor cepat
- `GET /api/directions` untuk geocode + rute OSRM
- `POST /api/route-score` untuk skor Aman/Waspada/Hindari
- `POST /api/sos/trigger` untuk SOS email

Jalankan backend dulu di port `8080`. Saat HP fisik terhubung via USB, wrapper `android/serve-android-wsl.sh` otomatis menjalankan:

```bash
adb reverse tcp:8080 tcp:8080
```

Jadi app di HP bisa mengakses backend WSL melalui `http://127.0.0.1:8080/api`. Kalau backend ada di host/port lain, override saat build/serve:

```bash
JALANAMAN_API_BASE_URL="http://192.168.1.10:8080/api" bash android/serve-android-wsl.sh
```

Script install/repair menambahkan function `dx()` ke `~/.bashrc`. Function itu hanya menambahkan `--platform android` saat kamu berada di folder `frontend/mobile` dan menjalankan `dx serve` atau `dx build` tanpa `--platform`.
Untuk `dx serve`, function itu juga memanggil `android/serve-android-wsl.sh`, jadi env Android/cache dan deteksi `adb.exe` Windows tetap ikut aktif.

Kalau ingin wrapper yang sekalian set cache/env dan mencoba memakai `adb.exe` Windows, jalankan:

```bash
bash android/serve-android-wsl.sh
```

Untuk melihat hasil app:

1. Buka Android Studio di Windows.
2. Buka Device Manager.
3. Start emulator, misalnya `Pixel_6`.
4. Cek di Windows PowerShell:

```powershell
& "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe" devices
```

Harus muncul device seperti:

```bash
emulator-5554    device
```

Lalu jalankan dari WSL:

```bash
cd frontend/mobile
dx serve
```

Untuk menjalankan di HP Android fisik:

1. Aktifkan Developer options di HP.
2. Aktifkan USB debugging.
3. Sambungkan HP ke Windows via USB.
4. Pilih mode USB `File transfer` jika diminta.
5. Terima prompt `Allow USB debugging?` di HP.
6. Cek dari Windows PowerShell:

```powershell
& "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe" devices
```

Harus muncul device seperti:

```bash
R5CXXXXXXX    device
```

Kalau statusnya `unauthorized`, buka layar HP dan terima prompt debugging. Setelah itu jalankan dari WSL:

```bash
cd frontend/mobile
dx serve
```

Untuk test build tanpa membuka emulator:

```bash
cd frontend/mobile
dx build
```

Atau pakai wrapper langsung:

```bash
cd frontend/mobile
bash android/build-android-wsl.sh
```

Wrapper ini memakai `CARGO_TARGET_DIR=~/.cache/jalanaman/android-target` dan `GRADLE_USER_HOME=~/.cache/jalanaman/gradle`, jadi cache build Android tidak memenuhi folder project.

Jika muncul error `ANDROID_NDK_HOME`, install Android SDK/NDK untuk Linux di WSL lalu arahkan environment variable ke NDK. NDK Windows dari Android Studio biasanya punya toolchain `prebuilt/windows-x86_64`, sedangkan `dx` yang berjalan di WSL membutuhkan `prebuilt/linux-x86_64`.

```bash
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/28.2.13676358"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"

dx serve --platform android
```

Simpan export di `~/.bashrc` kalau sudah benar.

## Platform Files

Folder ini juga bisa dipakai untuk file Android khusus jika nanti diperlukan:

- `AndroidManifest.xml` untuk permission jaringan, lokasi presisi, notifikasi SOS, dan getar alarm. Aplikasi tidak meminta kamera, kontak, SMS, telepon, atau penyimpanan karena fitur MVP tidak memerlukannya.
- `res/` untuk drawable, icon, dan string
- `build.gradle` untuk konfigurasi build Android

Android push uses FCM. The web VAPID approach doesn't apply natively.
For native Android, integrate `firebase-android-sdk` or use `fcm-push` in Rust.
