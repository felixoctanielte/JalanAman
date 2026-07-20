use crate::app::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*; // Menggunakan path route yang benar sesuai module tree

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen w-full bg-slate-50 flex flex-col relative font-sans",

            // Navbar Biru Mobile bawaan
            header { class: "bg-[#2563eb] text-white px-6 py-4 flex justify-between items-center shadow-md shrink-0",
                div { class: "flex items-baseline gap-2",
                    span { class: "text-xl font-bold tracking-tight", "JalanAman" },
                    span { class: "text-xs text-blue-200", "Rute Teraman Indonesia" }
                },
                div { class: "flex items-center gap-4",
                    Link {
                        to: Route::Dashboard {},
                        class: "text-sm font-semibold bg-blue-700/50 hover:bg-blue-700 px-3 py-1.5 rounded transition-all",
                        "🗺️ Dashboard RT/RW"
                    }
                }
            },

            // Area tengah putih mobile
            main { class: "flex-1 flex items-center justify-center bg-white",
                p { class: "text-slate-400 text-sm", "[ Tampilan Utama Peta Mobile ]" }
            },

            // Floating Buttons Lapor Bahaya
            div { class: "absolute bottom-6 left-6 z-30 flex flex-col gap-3 items-start",
                button { class: "bg-white hover:bg-slate-50 text-slate-800 font-bold px-4 py-2.5 rounded-full shadow-lg border border-slate-200 text-sm",
                    "⚠️ Lapor Bahaya"
                }
            },

            // Tombol SOS Merah Mobile
            button { class: "absolute bottom-6 right-6 w-16 h-16 rounded-full bg-[#ef4444] text-white font-black text-sm shadow-2xl flex items-center justify-center",
                "SOS"
            }
        }
    }
}
