use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Clone, Copy, PartialEq)]
enum MobileTab {
    Map,
    Route,
    Contacts,
    Profile,
}

const APP: &str = "min-height:100vh;background:#eef6ff;color:#0f172a;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const SCREEN: &str = "min-height:100vh;max-width:430px;margin:0 auto;background:#f8fbff;position:relative;overflow:hidden;box-shadow:0 20px 60px rgba(15,23,42,0.12);";
const HEADER: &str = "height:64px;padding:12px 18px 8px;display:flex;align-items:center;justify-content:space-between;background:#ffffff;border-bottom:1px solid #dbeafe;";
const BRAND_WRAP: &str = "display:flex;flex-direction:column;gap:1px;";
const BRAND: &str = "font-size:20px;font-weight:800;color:#1d4ed8;letter-spacing:0;";
const SUBTITLE: &str = "font-size:11px;font-weight:600;color:#64748b;";
const ICON_BUTTON: &str = "width:38px;height:38px;border-radius:19px;border:1px solid #bfdbfe;background:#eff6ff;color:#1d4ed8;font-size:18px;font-weight:800;display:flex;align-items:center;justify-content:center;";
const CONTENT: &str =
    "position:absolute;top:64px;left:0;right:0;bottom:0;padding:14px 14px 94px;overflow:hidden;";
const MAP_CARD: &str = "height:54%;min-height:292px;position:relative;overflow:hidden;border-radius:26px;background:#dbeafe;border:1px solid #bfdbfe;box-shadow:0 18px 36px rgba(37,99,235,0.14);";
const MAP_GRID: &str = "position:absolute;inset:0;background-image:linear-gradient(rgba(37,99,235,0.10) 1px,transparent 1px),linear-gradient(90deg,rgba(37,99,235,0.10) 1px,transparent 1px);background-size:40px 40px;";
const ROAD_MAIN: &str = "position:absolute;left:-24px;right:-18px;top:145px;height:42px;background:#ffffff;border-top:1px solid #bfdbfe;border-bottom:1px solid #bfdbfe;transform:rotate(-10deg);box-shadow:0 8px 24px rgba(37,99,235,0.08);";
const ROAD_SECONDARY: &str = "position:absolute;left:160px;top:-28px;width:42px;height:360px;background:#ffffff;border-left:1px solid #bfdbfe;border-right:1px solid #bfdbfe;transform:rotate(24deg);box-shadow:0 8px 24px rgba(37,99,235,0.08);";
const MAP_LABEL: &str = "position:absolute;left:14px;top:14px;padding:8px 10px;border-radius:18px;background:rgba(255,255,255,0.92);border:1px solid #dbeafe;color:#1e3a8a;font-size:12px;font-weight:800;box-shadow:0 10px 20px rgba(15,23,42,0.08);";
const CURRENT_POS: &str = "position:absolute;left:63%;top:56%;width:18px;height:18px;margin:-9px 0 0 -9px;border-radius:50%;background:#2563eb;border:3px solid #ffffff;box-shadow:0 0 0 8px rgba(37,99,235,0.16),0 10px 18px rgba(37,99,235,0.28);";
const REPORT_FAB: &str = "position:absolute;right:16px;bottom:18px;width:52px;height:52px;border-radius:26px;border:0;background:#2563eb;color:#ffffff;font-size:30px;line-height:1;font-weight:700;box-shadow:0 16px 28px rgba(37,99,235,0.34);display:flex;align-items:center;justify-content:center;";
const LEGEND: &str =
    "position:absolute;left:12px;right:76px;bottom:14px;display:flex;gap:7px;flex-wrap:wrap;";
const LEGEND_ITEM: &str = "padding:5px 9px;border-radius:999px;background:rgba(255,255,255,0.94);border:1px solid #dbeafe;font-size:10px;font-weight:800;color:#334155;";
const ROUTE_CARD: &str = "margin-top:12px;background:#ffffff;border:1px solid #dbeafe;border-radius:22px;padding:14px;box-shadow:0 12px 28px rgba(15,23,42,0.08);";
const ROUTE_TOP: &str = "display:flex;align-items:center;justify-content:space-between;gap:12px;";
const EYEBROW: &str = "font-size:11px;color:#64748b;font-weight:700;margin-bottom:3px;";
const ROUTE_TITLE: &str = "font-size:14px;color:#0f172a;font-weight:800;";
const SAFE_BADGE: &str = "flex-shrink:0;border-radius:999px;background:#dcfce7;color:#166534;padding:7px 11px;font-size:11px;font-weight:900;";
const ROUTE_META: &str =
    "margin-top:12px;display:grid;grid-template-columns:repeat(3,1fr);gap:8px;";
const META_CELL: &str =
    "border-radius:16px;background:#eff6ff;border:1px solid #dbeafe;padding:10px 8px;";
const META_VALUE: &str = "font-size:14px;font-weight:900;color:#1d4ed8;";
const META_LABEL: &str = "margin-top:2px;font-size:10px;font-weight:700;color:#64748b;";
const SHEET: &str = "position:absolute;left:14px;right:14px;bottom:96px;background:#ffffff;border:1px solid #bfdbfe;border-radius:24px;padding:14px;box-shadow:0 22px 48px rgba(15,23,42,0.18);";
const INPUT: &str = "width:100%;box-sizing:border-box;border:1px solid #bfdbfe;background:#f8fbff;border-radius:16px;padding:12px 14px;color:#0f172a;font-size:14px;font-weight:650;outline:none;";
const PRIMARY_BUTTON: &str = "width:100%;margin-top:10px;border:0;border-radius:16px;background:#2563eb;color:#ffffff;padding:12px 14px;font-size:14px;font-weight:900;box-shadow:0 12px 24px rgba(37,99,235,0.24);";
const BOTTOM_BAR: &str = "position:absolute;left:0;right:0;bottom:0;height:82px;background:#ffffff;border-top:1px solid #bfdbfe;display:grid;grid-template-columns:1fr 1fr 74px 1fr 1fr;align-items:center;padding:8px 10px 12px;box-shadow:0 -12px 28px rgba(15,23,42,0.08);";
const NAV_BUTTON: &str = "height:58px;border:0;background:transparent;color:#64748b;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:800;";
const NAV_BUTTON_ACTIVE: &str = "height:58px;border:0;background:#eff6ff;color:#1d4ed8;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:900;border-radius:18px;";
const NAV_ICON: &str = "font-size:20px;line-height:1;";
const SOS_BUTTON: &str = "position:absolute;left:50%;bottom:42px;transform:translateX(-50%);width:64px;height:64px;border-radius:32px;border:5px solid #ffffff;background:#ef4444;color:#ffffff;font-size:14px;font-weight:950;letter-spacing:0;box-shadow:0 18px 32px rgba(239,68,68,0.34);display:flex;align-items:center;justify-content:center;";
const SOS_BUTTON_ACTIVE: &str = "position:absolute;left:50%;bottom:42px;transform:translateX(-50%);width:64px;height:64px;border-radius:32px;border:5px solid #ffffff;background:#b91c1c;color:#ffffff;font-size:12px;font-weight:950;letter-spacing:0;box-shadow:0 0 0 10px rgba(239,68,68,0.16),0 18px 32px rgba(239,68,68,0.38);display:flex;align-items:center;justify-content:center;";
const ALERT: &str = "position:absolute;left:18px;right:18px;bottom:168px;border-radius:18px;border:1px solid #fecaca;background:#fff1f2;color:#991b1b;padding:11px 13px;font-size:12px;font-weight:800;box-shadow:0 14px 28px rgba(127,29,29,0.14);";
const DASHBOARD_WRAP: &str = "min-height:100vh;max-width:430px;margin:0 auto;background:#f8fbff;padding:18px;box-sizing:border-box;color:#0f172a;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
const BACK_LINK: &str = "display:inline-flex;align-items:center;gap:6px;color:#1d4ed8;text-decoration:none;font-size:13px;font-weight:800;margin-bottom:18px;";
const DASH_TITLE: &str =
    "font-size:24px;line-height:1.1;font-weight:900;color:#0f172a;margin:0 0 14px;";
const STAT_GRID: &str = "display:grid;grid-template-columns:1fr 1fr;gap:10px;margin-top:16px;";
const STAT_CARD: &str = "background:#ffffff;border:1px solid #dbeafe;border-radius:20px;padding:14px;box-shadow:0 12px 24px rgba(15,23,42,0.06);";

#[component]
fn Home() -> Element {
    let mut active_tab = use_signal(|| MobileTab::Map);
    let mut sos_active = use_signal(|| false);

    let is_route = *active_tab.read() == MobileTab::Route;
    let is_sos_active = *sos_active.read();

    rsx! {
        main { style: APP,
            div { style: SCREEN,
                header { style: HEADER,
                    div { style: BRAND_WRAP,
                        span { style: BRAND, "JalanAman" }
                        span { style: SUBTITLE, "Rute aman, laporan cepat" }
                    }
                    button { style: ICON_BUTTON, "!" }
                }

                section { style: CONTENT,
                    div { style: MAP_CARD,
                        div { style: MAP_GRID }
                        div { style: ROAD_MAIN }
                        div { style: ROAD_SECONDARY }
                        div { style: MAP_LABEL, "Area sekitar kamu" }
                        MapDot { top: "62px", left: "52px", color: "#f59e0b" }
                        MapDot { top: "116px", left: "164px", color: "#ef4444" }
                        MapDot { top: "190px", left: "96px", color: "#f97316" }
                        MapDot { top: "82px", left: "252px", color: "#6366f1" }
                        div { style: CURRENT_POS }
                        div { style: LEGEND,
                            span { style: LEGEND_ITEM, "Pencahayaan" }
                            span { style: LEGEND_ITEM, "Kriminal" }
                            span { style: LEGEND_ITEM, "Kecelakaan" }
                        }
                        button {
                            style: REPORT_FAB,
                            title: "Lapor cepat",
                            onclick: move |_| active_tab.set(MobileTab::Map),
                            "+"
                        }
                    }

                    div { style: ROUTE_CARD,
                        div { style: ROUTE_TOP,
                            div {
                                div { style: EYEBROW, "Skor rute ke tujuan" }
                                div { style: ROUTE_TITLE, "Jl. Kenanga - Kos Melati" }
                            }
                            span { style: SAFE_BADGE, "Aman" }
                        }
                        div { style: ROUTE_META,
                            Metric { value: "86", label: "Skor" }
                            Metric { value: "7m", label: "Estimasi" }
                            Metric { value: "3", label: "Laporan" }
                        }
                    }
                }

                if is_route {
                    RouteSheet {}
                }

                if is_sos_active {
                    div { style: ALERT, "SOS aktif. Alarm dan notifikasi darurat siap dikirim." }
                }

                nav { style: BOTTOM_BAR,
                    NavButton {
                        active: *active_tab.read() == MobileTab::Map,
                        icon: "⌖",
                        label: "Peta",
                        onclick: move |_| active_tab.set(MobileTab::Map),
                    }
                    NavButton {
                        active: *active_tab.read() == MobileTab::Route,
                        icon: "↗",
                        label: "Rute",
                        onclick: move |_| active_tab.set(MobileTab::Route),
                    }
                    div {}
                    NavButton {
                        active: *active_tab.read() == MobileTab::Contacts,
                        icon: "☎",
                        label: "Kontak",
                        onclick: move |_| active_tab.set(MobileTab::Contacts),
                    }
                    NavButton {
                        active: *active_tab.read() == MobileTab::Profile,
                        icon: "○",
                        label: "Profil",
                        onclick: move |_| active_tab.set(MobileTab::Profile),
                    }
                }

                button {
                    style: if is_sos_active { SOS_BUTTON_ACTIVE } else { SOS_BUTTON },
                    onclick: move |_| {
                        let next = !*sos_active.read();
                        sos_active.set(next);
                    },
                    if is_sos_active { "STOP" } else { "SOS" }
                }
            }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        main { style: APP,
            section { style: DASHBOARD_WRAP,
                Link { to: Route::Home {}, style: BACK_LINK, "← Kembali" }
                h1 { style: DASH_TITLE, "Dashboard keamanan" }
                div { style: ROUTE_CARD,
                    div { style: EYEBROW, "Ringkasan wilayah" }
                    div { style: ROUTE_TITLE, "Jakarta Pusat dalam 30 hari terakhir" }
                    div { style: ROUTE_META,
                        Metric { value: "24", label: "Laporan" }
                        Metric { value: "86", label: "Skor" }
                        Metric { value: "Aman", label: "Status" }
                    }
                }
                div { style: STAT_GRID,
                    StatCard { title: "Kriminal", value: "9", color: "#ef4444" }
                    StatCard { title: "Kecelakaan", value: "6", color: "#f97316" }
                    StatCard { title: "Pencahayaan", value: "7", color: "#f59e0b" }
                    StatCard { title: "Lainnya", value: "2", color: "#64748b" }
                }
            }
        }
    }
}

#[component]
fn MapDot(top: &'static str, left: &'static str, color: &'static str) -> Element {
    rsx! {
        div {
            style: "position:absolute;top:{top};left:{left};width:12px;height:12px;border-radius:50%;background:{color};border:2px solid #ffffff;box-shadow:0 8px 14px rgba(15,23,42,0.20);",
        }
    }
}

#[component]
fn Metric(value: &'static str, label: &'static str) -> Element {
    rsx! {
        div { style: META_CELL,
            div { style: META_VALUE, "{value}" }
            div { style: META_LABEL, "{label}" }
        }
    }
}

#[component]
fn RouteSheet() -> Element {
    rsx! {
        div { style: SHEET,
            div { style: EYEBROW, "Cari rute aman" }
            input {
                style: INPUT,
                placeholder: "Masukkan tujuan",
                value: "Kos Melati",
            }
            button { style: PRIMARY_BUTTON, "Cek rute" }
        }
    }
}

#[component]
fn NavButton(
    active: bool,
    icon: &'static str,
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: if active { NAV_BUTTON_ACTIVE } else { NAV_BUTTON },
            onclick: move |event| onclick.call(event),
            span { style: NAV_ICON, "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, color: &'static str) -> Element {
    rsx! {
        div { style: STAT_CARD,
            div {
                style: "width:10px;height:10px;border-radius:50%;background:{color};margin-bottom:12px;",
            }
            div { style: META_VALUE, "{value}" }
            div { style: META_LABEL, "{title}" }
        }
    }
}
