use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    // State management
    let mut is_dark = use_signal(|| true);
    let mut show_gelap = use_signal(|| true);
    let mut show_rawan = use_signal(|| true);
    let mut show_kecelakaan = use_signal(|| true);

    // Styling dinamis
    let page_bg = if is_dark() { "bg-slate-950" } else { "bg-slate-50" };
    let text_main = if is_dark() { "text-slate-100" } else { "text-slate-900" };
    let text_muted = if is_dark() { "text-slate-400" } else { "text-slate-500" };
    
    // Menggunakan gaya "soft" tanpa border kaku
    let card_style = if is_dark() { 
        "bg-slate-900/40 border-slate-800" 
    } else { 
        "bg-white border-slate-200" 
    };

    rsx! {
        div { class: "min-h-screen w-full {page_bg} p-6 md:p-12 font-sans transition-colors duration-300",
            
            // Header: Dibuat lebih lega dengan tipografi kontras
            div { class: "max-w-6xl mx-auto mb-10 flex justify-between items-center",
                div {
                    h1 { class: "text-3xl font-extrabold tracking-tight {text_main}", "Dashboard Keamanan" }
                    p { class: "text-sm mt-1 {text_muted}", "Memantau situasi wilayah Kel. Sukamaju secara real-time" }
                },
                button {
                    onclick: move |_| is_dark.set(!is_dark()),
                    class: "px-5 py-2.5 rounded-full text-xs font-bold border border-slate-700/20 hover:scale-105 transition-all {text_main}",
                    if is_dark() { "☀️ Light Mode" } else { "🌙 Dark Mode" }
                }
            },

            // Container Utama: Menggunakan grid untuk layout yang lebih natural
            div { class: "max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8",
                
                // Sidebar Filter
                div { class: "md:col-span-1 space-y-10",
                    div { class: "space-y-4",
                        h2 { class: "text-xs font-bold uppercase tracking-widest {text_muted}", "Kategori Laporan" },
                        div { class: "space-y-3",
                            // Checkbox manual agar state mutasi aman (tidak kena borrow error)
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
                            }
                        }
                    }
                    
                    div { class: "space-y-3",
                        h2 { class: "text-xs font-bold uppercase tracking-widest {text_muted}", "Rentang Waktu" },
                        select { class: "w-full p-3 rounded-xl border border-slate-200 dark:border-slate-800 bg-transparent text-sm {text_main}",
                            option { "30 hari terakhir" }
                            option { "7 hari terakhir" }
                            option { "Hari ini" }
                        }
                    }
                },

                // Area Heatmap (dibuat lebih luas & modern)
                div { class: "md:col-span-3 h-[500px] rounded-3xl shadow-2xl shadow-slate-200/50 dark:shadow-none border border-slate-200/60 dark:border-slate-800/50 {card_style} relative overflow-hidden transition-all duration-500",
                    if show_rawan() {
                        div { class: "absolute top-[20%] left-[20%] w-32 h-32 rounded-full bg-red-500/20 blur-3xl animate-pulse" }
                    },
                    if show_gelap() {
                        div { class: "absolute bottom-[30%] left-[40%] w-40 h-40 rounded-full bg-amber-500/20 blur-3xl" }
                    },
                    if show_kecelakaan() {
                        div { class: "absolute bottom-[20%] left-[25%] w-24 h-24 rounded-full bg-rose-600/30 blur-2xl" }
                    }
                }
            }
        }
    }
}