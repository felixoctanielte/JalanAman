use dioxus::prelude::*;
use dioxus::document::eval;
use crate::components::map::HeatmapMapView;
use crate::services::api::{HeatmapPoint, get_heatmap_data}; 

#[component]
pub fn Dashboard() -> Element {
    // State management untuk Light/Dark Mode (Default: true/Dark)
    let mut is_dark = use_signal(|| true);
    
    // State Filter Checkbox (Ditambahkan Kategori Lainnya)
    let mut show_gelap = use_signal(|| true);
    let mut show_rawan = use_signal(|| true);
    let mut show_kecelakaan = use_signal(|| true);
    let mut show_lainnya = use_signal(|| true); // ✨ BARU: State untuk kategori Lainnya (other)
    
    // State untuk Search Bar
    let mut search_query = use_signal(|| String::new());

    // State untuk koordinat riil user dan error handling
    let mut user_location = use_signal(|| None::<(f64, f64)>);
    let mut location_error = use_signal(|| None::<String>);

    // STATE: Menampung kumpulan data HeatmapPoint riil hasil fetch dari Backend API
    let mut raw_points_data = use_signal(|| Vec::<HeatmapPoint>::new());

    // Alur inisialisasi awal: Minta izin lokasi pengguna & Fetch data dari Backend
    use_effect(move || {
        let mut geo_eval = eval(r#"
            if (navigator.geolocation) {
                navigator.geolocation.getCurrentPosition(
                    (position) => {
                        dioxus.send(JSON.stringify({
                            status: "success",
                            lat: position.coords.latitude,
                            lng: position.coords.longitude
                        }));
                    },
                    (error) => {
                        let msg = "Akses lokasi ditolak. Mohon aktifkan izin lokasi pada browser Anda untuk melihat peta keamanan di sekitar Anda.";
                        if (error.code === error.PERMISSION_DENIED) {
                            dioxus.send(JSON.stringify({ status: "error", message: msg }));
                        } else {
                            dioxus.send(JSON.stringify({ status: "error", message: "Gagal mendapatkan lokasi terkini." }));
                        }
                    },
                    { enableHighAccuracy: true }
                );
            } else {
                dioxus.send(JSON.stringify({ status: "error", message: "Browser Anda tidak mendukung fitur Geolocation." }));
            }
        "#);

        spawn(async move {
            // 1. Ambil koordinat lokasi user dari sisi browser web
            if let Ok(response) = geo_eval.recv::<String>().await {
                if let Ok(res_json) = serde_json::from_str::<serde_json::Value>(&response) {
                    if res_json["status"] == "success" {
                        let lat = res_json["lat"].as_f64().unwrap_or(0.0);
                        let lng = res_json["lng"].as_f64().unwrap_or(0.0);
                        user_location.set(Some((lat, lng)));
                    } else {
                        let err_msg = res_json["message"].as_str().unwrap_or("Unknown error").to_string();
                        location_error.set(Some(err_msg));
                    }
                }
            }

            // 🚀 2. KONEKSI BACKEND: Ambil data heatmap aktual dari database
            match get_heatmap_data().await {
                Ok(points) => {
                    raw_points_data.set(points);
                }
                Err(e) => {
                    log::error!("Gagal memuat titik heatmap dari backend: {}", e);
                }
            }
        });
    });

    // ✨ FILTER DATA: Memproses penyaringan berdasarkan kategori (Termasuk Kategori "other" / "Lainnya")
    let filtered_points = raw_points_data()
        .into_iter()
        .filter(|point| {
            if point.category == "lighting" && !show_gelap() { return false; }
            if point.category == "crime" && !show_rawan() { return false; }
            if point.category == "accident" && !show_kecelakaan() { return false; }
            if point.category == "other" && !show_lainnya() { return false; } // ✨ BARU: Menyaring kategori other
            true
        })
        .collect::<Vec<HeatmapPoint>>();

    // Fungsi Aksi Search via Nominatim OpenStreetMap
    let on_search = move |_| {
        let query = search_query();
        if query.is_empty() { return; }

        let mut search_eval = eval(r#"
            let query = await dioxus.recv();
            try {
                let url = 'https://nominatim.openstreetmap.org/search?' + new URLSearchParams({ q: query, format: 'json', limit: '1', countrycodes: 'id' });
                let res = await fetch(url, { headers: { 'User-Agent': 'JalanAman/1.0' } });
                let data = await res.json();
                if (data && data.length > 0) {
                    let lat = parseFloat(data[0].lat);
                    let lng = parseFloat(data[0].lon);
                    if (window.ja_panTo) {
                        window.ja_panTo(lat, lng);
                    }
                } else {
                    alert("Lokasi tidak ditemukan di Indonesia!");
                }
            } catch (e) {
                console.error("Gagal melakukan pencarian:", e);
            }
        "#);
        let _ = search_eval.send(query);
    };

    // Fungsi untuk memicu reposisi ke live location secara manual dari tombol
    let on_live_location = move |_| {
        let _ = eval(r#"
            if (navigator.geolocation) {
                navigator.geolocation.getCurrentPosition((position) => {
                    if (window.ja_panTo) {
                        window.ja_panTo(position.coords.latitude, position.coords.longitude);
                    }
                }, null, { enableHighAccuracy: true });
            }
        "#);
    };

    // Styling dinamis sesuai tema Light/Dark
    let page_bg = if is_dark() { "bg-slate-950" } else { "bg-slate-50" };
    let text_main = if is_dark() { "text-slate-100" } else { "text-slate-900" };
    let text_muted = if is_dark() { "text-slate-400" } else { "text-slate-500" };
    
    let card_style = if is_dark() { 
        "bg-slate-900/40 border-slate-800" 
    } else { 
        "bg-white border-slate-200" 
    };

    // ✨ BARU: Styling Search Bar Container yang adaptif penuh terhadap Light/Dark Mode
    let search_bar_bg = if is_dark() {
        "bg-slate-900/60 border-slate-800"
    } else {
        "bg-white border-slate-300/80 shadow-sm"
    };

    rsx! {
        div { class: "min-h-screen w-full {page_bg} p-6 md:p-12 font-sans transition-colors duration-300",
            
            // Header
            div { class: "max-w-6xl mx-auto mb-10 flex justify-between items-center",
                div {
                    h1 { class: "text-3xl font-extrabold tracking-tight {text_main}", "Dashboard Keamanan" }
                    p { class: "text-sm mt-1 {text_muted}", "Memantau situasi wilayah secara real-time" }
                },
                button {
                    onclick: move |_| is_dark.set(!is_dark()),
                    class: "px-5 py-2.5 rounded-full text-xs font-bold border border-slate-700/20 hover:scale-105 transition-all {text_main}",
                    if is_dark() { "☀️ Light Mode" } else { "🌙 Dark Mode" }
                }
            },

            // Container Utama
            div { class: "max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8",
                
                // Sidebar Filter
                div { class: "md:col-span-1 space-y-10",
                    div { class: "space-y-4",
                        h2 { class: "text-xs font-bold uppercase tracking-widest {text_muted}", "Kategori Laporan" },
                        div { class: "space-y-3",
                            label { class: "flex items-center gap-3 cursor-pointer group",
                                input { "type": "checkbox", checked: show_gelap(), onclick: move |_| show_gelap.set(!show_gelap()), class: "w-5 h-5 rounded border-slate-300 accent-amber-500 cursor-pointer" }
                                span { class: "text-sm font-medium group-hover:text-blue-500 transition-colors {text_main}", "Gelap" }
                            },
                            label { class: "flex items-center gap-3 cursor-pointer group",
                                input { "type": "checkbox", checked: show_rawan(), onclick: move |_| show_rawan.set(!show_rawan()), class: "w-5 h-5 rounded border-slate-300 accent-red-500 cursor-pointer" }
                                span { class: "text-sm font-medium group-hover:text-blue-500 transition-colors {text_main}", "Rawan" }
                            },
                            label { class: "flex items-center gap-3 cursor-pointer group",
                                input { "type": "checkbox", checked: show_kecelakaan(), onclick: move |_| show_kecelakaan.set(!show_kecelakaan()), class: "w-5 h-5 rounded border-slate-300 accent-rose-500 cursor-pointer" }
                                span { class: "text-sm font-medium group-hover:text-blue-500 transition-colors {text_main}", "Kecelakaan" }
                            },
                            // ✨ BARU: Checkbox Kategori "Lainnya"
                            label { class: "flex items-center gap-3 cursor-pointer group",
                                input { "type": "checkbox", checked: show_lainnya(), onclick: move |_| show_lainnya.set(!show_lainnya()), class: "w-5 h-5 rounded border-slate-300 accent-slate-500 cursor-pointer" }
                                span { class: "text-sm font-medium group-hover:text-blue-500 transition-colors {text_main}", "Lainnya" }
                            }
                        }
                    }
                },

                // Area Konten Peta & Search Bar
                div { class: "md:col-span-3 space-y-4",
                    
                    // Search Bar & Tombol Lokasi Saya (Menggunakan class adaptif {search_bar_bg})
                    if user_location().is_some() {
                        div { class: "flex gap-2 w-full max-w-lg rounded-2xl overflow-hidden border p-1.5 transition-colors duration-300 {search_bar_bg}",
                            input {
                                "type": "text",
                                placeholder: "Cari daerah (contoh: Gading Serpong)...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                                class: "w-full bg-transparent px-4 py-2 text-sm focus:outline-none {text_main}"
                            }
                            button {
                                onclick: on_search,
                                class: "bg-blue-600 hover:bg-blue-700 text-white font-bold text-xs px-5 py-2.5 rounded-xl transition-all whitespace-nowrap",
                                "Cari"
                            }
                            button {
                                onclick: on_live_location,
                                class: "bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs px-4 py-2.5 rounded-xl transition-all flex items-center gap-1 whitespace-nowrap",
                                span { "📍" }
                                "Lokasi Saya"
                            }
                        }
                    }

                    // Box Konten Utama (Peta atau Wording Error)
                    div { class: "h-[500px] rounded-3xl border {card_style} relative overflow-hidden transition-all duration-500 flex items-center justify-center p-6 text-center",
                        if let Some((lat, lng)) = user_location() {
                            // Tampilkan peta asli menggunakan koordinat user ter-update
                            HeatmapMapView { points: filtered_points, center_lat: lat, center_lng: lng }
                        } else if let Some(err_msg) = location_error() {
                            // Tampilkan Wording yang Sesuai jika Permission ditolak
                            div { class: "max-w-md space-y-4",
                                div { class: "text-4xl", "🔒" }
                                h3 { class: "text-lg font-bold {text_main}", "Akses Peta Diblokir" }
                                p { class: "text-sm {text_muted} leading-relaxed", "{err_msg}" }
                            }
                        } else {
                            // Loading state saat mencari koordinat GPS pertama kali
                            div { class: "text-sm font-medium animate-pulse {text_muted}", "Meminta akses lokasi saat ini..." }
                        }
                    }
                }
            }
        }
    }
}