use crate::app_config::ThemeSpec;

pub(crate) const APP: &str = "min-height:100dvh;background:radial-gradient(circle at 50% 108%,rgba(34,197,94,0.34),rgba(34,197,94,0) 34%),radial-gradient(circle at 14% 8%,rgba(37,99,235,0.22),rgba(37,99,235,0) 30%),linear-gradient(145deg,#27272a 0%,#17181c 46%,#0b0f14 100%);color:#f8fafc;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
pub(crate) const SCREEN: &str = "width:100vw;max-width:100vw;min-height:100dvh;background:radial-gradient(circle at 50% 92%,rgba(34,197,94,0.18),rgba(34,197,94,0) 28%),radial-gradient(circle at 86% 10%,rgba(14,165,233,0.18),rgba(14,165,233,0) 34%),linear-gradient(180deg,rgba(39,40,45,0.94) 0%,rgba(23,24,29,0.95) 48%,rgba(10,14,19,0.98) 100%);position:relative;overflow:hidden;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 0 0 1px rgba(255,255,255,0.08);";
pub(crate) const APP_LIGHT: &str = "min-height:100dvh;background:linear-gradient(145deg,#eff6ff 0%,#f8fafc 46%,#ecfeff 100%);color:#0f172a;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
pub(crate) const SCREEN_LIGHT: &str = "width:100vw;max-width:100vw;min-height:100dvh;background:linear-gradient(180deg,#ffffff 0%,#f8fafc 52%,#eef6ff 100%);position:relative;overflow:hidden;box-shadow:inset 0 1px 0 rgba(255,255,255,0.96),0 0 0 1px rgba(15,23,42,0.08);";
pub(crate) const HEADER: &str = "height:88px;padding:16px 18px;display:flex;align-items:center;justify-content:space-between;background:linear-gradient(180deg,rgba(58,59,66,0.72),rgba(30,32,39,0.58));border-bottom:1px solid rgba(255,255,255,0.13);box-sizing:border-box;box-shadow:0 18px 46px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.12);backdrop-filter:blur(26px) saturate(175%);-webkit-backdrop-filter:blur(26px) saturate(175%);";
pub(crate) const BRAND_WRAP: &str = "display:flex;align-items:center;gap:12px;min-width:0;";
pub(crate) const BRAND: &str = "font-size:21px;font-weight:950;color:#f8fafc;letter-spacing:0;line-height:1;text-shadow:0 1px 1px rgba(0,0,0,0.24);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
pub(crate) const SUBTITLE: &str = "margin-top:3px;font-size:10px;font-weight:800;color:#93c5fd;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
pub(crate) const HEADER_LOGO: &str = "width:42px;height:42px;object-fit:contain;flex-shrink:0;filter:drop-shadow(0 12px 20px rgba(37,99,235,0.34));";
pub(crate) const ICON_BUTTON: &str = "width:42px;height:42px;border-radius:14px;border:1px solid rgba(255,255,255,0.18);background:rgba(47,50,58,0.76);color:#e0f2fe;font-size:16px;font-weight:900;display:flex;align-items:center;justify-content:center;box-shadow:0 14px 32px rgba(0,0,0,0.24),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);";
pub(crate) const HEADER_ACTIONS: &str = "display:flex;align-items:center;gap:6px;flex-shrink:0;";
pub(crate) const LANGUAGE_TOGGLE: &str = "width:56px;height:42px;border-radius:14px;border:1px solid rgba(255,255,255,0.18);background:rgba(47,50,58,0.76);color:#e0f2fe;padding:5px;display:grid;grid-template-columns:1fr 1fr;gap:3px;align-items:center;box-shadow:0 14px 32px rgba(0,0,0,0.24),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.24);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);";
pub(crate) const LANGUAGE_SEGMENT_ACTIVE: &str = "height:30px;border-radius:10px;display:flex;align-items:center;justify-content:center;background:#1d4ed8;color:#ffffff;font-size:9px;font-weight:950;box-shadow:0 8px 16px rgba(37,99,235,0.22),inset 0 1px 0 rgba(255,255,255,0.22),inset 0 -1px 0 rgba(0,0,0,0.20);";
pub(crate) const LANGUAGE_SEGMENT_IDLE: &str = "height:30px;border-radius:10px;display:flex;align-items:center;justify-content:center;color:#cbd5e1;font-size:9px;font-weight:900;";
pub(crate) const CONTENT: &str = "position:absolute;top:88px;left:0;right:0;bottom:130px;padding:14px 14px 18px;overflow-y:auto;box-sizing:border-box;";
pub(crate) const MAP_CARD: &str = "height:clamp(326px,46dvh,368px);position:relative;overflow:hidden;border-radius:8px;background:rgba(44,46,53,0.66);border:1px solid rgba(255,255,255,0.16);box-shadow:0 24px 54px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.14);backdrop-filter:blur(18px) saturate(165%);-webkit-backdrop-filter:blur(18px) saturate(165%);";
pub(crate) const ROUTE_MAP_CARD: &str = "height:292px;margin-top:12px;position:relative;overflow:hidden;border-radius:8px;background:rgba(44,46,53,0.66);border:1px solid rgba(255,255,255,0.16);box-shadow:0 20px 44px rgba(0,0,0,0.30),inset 0 1px 0 rgba(255,255,255,0.14);backdrop-filter:blur(18px) saturate(165%);-webkit-backdrop-filter:blur(18px) saturate(165%);";
pub(crate) const MAP_IFRAME: &str =
    "position:absolute;inset:0;width:100%;height:100%;border:0;background:#111827;";
pub(crate) const MAP_LABEL: &str = "position:absolute;left:12px;top:12px;max-width:min(250px,calc(100% - 112px));padding:10px 12px;border-radius:8px;background:linear-gradient(180deg,rgba(45,47,55,0.78),rgba(23,25,31,0.62));border:1px solid rgba(255,255,255,0.18);color:#f8fafc;font-size:12px;font-weight:900;box-shadow:0 14px 30px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.16);backdrop-filter:blur(18px) saturate(170%);-webkit-backdrop-filter:blur(18px) saturate(170%);z-index:2;";
pub(crate) const MAP_PROVIDER: &str = "position:absolute;left:12px;bottom:14px;padding:8px 10px;border-radius:8px;background:rgba(10,38,91,0.78);border:1px solid rgba(255,255,255,0.25);color:#ffffff;font-size:10px;font-weight:850;box-shadow:0 12px 24px rgba(15,47,109,0.18);backdrop-filter:blur(14px) saturate(150%);-webkit-backdrop-filter:blur(14px) saturate(150%);z-index:2;";
pub(crate) const CARD: &str = "margin-top:12px;background:linear-gradient(180deg,rgba(52,54,62,0.64),rgba(22,24,31,0.54));border:1px solid rgba(255,255,255,0.14);border-radius:8px;padding:14px;box-shadow:0 18px 42px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.14);box-sizing:border-box;backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);";
pub(crate) const CARD_TIGHT: &str = "background:linear-gradient(180deg,rgba(52,54,62,0.66),rgba(22,24,31,0.56));border:1px solid rgba(255,255,255,0.14);border-radius:8px;padding:13px;box-shadow:0 18px 42px rgba(0,0,0,0.28),inset 0 1px 0 rgba(255,255,255,0.14);box-sizing:border-box;backdrop-filter:blur(24px) saturate(175%);-webkit-backdrop-filter:blur(24px) saturate(175%);";
pub(crate) const ROW: &str =
    "display:flex;align-items:center;justify-content:space-between;gap:12px;";
pub(crate) const EYEBROW: &str = "font-size:11px;color:#9ca3af;font-weight:800;margin-bottom:3px;";
pub(crate) const TITLE: &str = "font-size:14px;color:#f8fafc;font-weight:900;line-height:1.25;";
pub(crate) const BODY: &str = "font-size:12px;color:#cbd5e1;font-weight:700;line-height:1.48;";
pub(crate) const META_GRID: &str =
    "margin-top:12px;display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:8px;";
pub(crate) const META_CELL: &str = "border-radius:8px;background:rgba(255,255,255,0.07);border:1px solid rgba(255,255,255,0.12);padding:10px 8px;min-height:58px;box-sizing:border-box;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);";
pub(crate) const META_VALUE: &str = "font-size:14px;font-weight:950;color:#93c5fd;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;";
pub(crate) const META_LABEL: &str = "margin-top:2px;font-size:10px;font-weight:800;color:#9ca3af;";
pub(crate) const FIELD_GRID: &str =
    "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
pub(crate) const INPUT: &str = "width:100%;box-sizing:border-box;border:1px solid rgba(255,255,255,0.13);background:rgba(255,255,255,0.08);border-radius:8px;padding:12px 14px;color:#f8fafc;font-size:14px;font-weight:700;outline:none;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 8px 18px rgba(0,0,0,0.14);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
pub(crate) const TEXTAREA: &str = "width:100%;min-height:88px;box-sizing:border-box;border:1px solid rgba(255,255,255,0.13);background:rgba(255,255,255,0.08);border-radius:8px;padding:12px 14px;color:#f8fafc;font-size:14px;font-weight:700;outline:none;resize:none;font-family:inherit;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12),0 8px 18px rgba(0,0,0,0.14);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
pub(crate) const PRIMARY_BUTTON: &str = "width:100%;margin-top:10px;border:1px solid rgba(255,255,255,0.38);border-radius:8px;background:#1d4ed8;color:#ffffff;padding:12px 14px;font-size:14px;font-weight:950;box-shadow:0 16px 30px rgba(37,99,235,0.24),inset 0 1px 0 rgba(255,255,255,0.28),inset 0 -1px 0 rgba(0,0,0,0.20);";
pub(crate) const SECONDARY_BUTTON: &str = "width:100%;margin-top:10px;border:1px solid rgba(147,197,253,0.32);border-radius:8px;background:rgba(255,255,255,0.07);color:#bfdbfe;padding:12px 14px;font-size:14px;font-weight:900;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);backdrop-filter:blur(16px) saturate(150%);-webkit-backdrop-filter:blur(16px) saturate(150%);";
pub(crate) const CATEGORY_GRID: &str =
    "display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:10px;";
pub(crate) const CATEGORY_BUTTON: &str = "height:54px;border:1px solid rgba(255,255,255,0.12);border-radius:8px;background:rgba(255,255,255,0.07);color:#d1d5db;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:850;text-align:left;box-shadow:inset 0 1px 0 rgba(255,255,255,0.12);";
pub(crate) const CATEGORY_BUTTON_ACTIVE: &str = "height:54px;border:1px solid rgba(56,189,248,0.52);border-radius:8px;background:rgba(37,99,235,0.20);color:#f8fafc;display:flex;align-items:center;gap:8px;padding:0 11px;font-size:12px;font-weight:950;text-align:left;box-shadow:0 10px 22px rgba(14,165,233,0.16),inset 0 1px 0 rgba(255,255,255,0.16);";
pub(crate) const BOTTOM_BAR: &str = "position:absolute;left:18px;right:18px;bottom:16px;height:76px;border-radius:38px;background:rgba(31,34,40,0.78);border:1px solid rgba(255,255,255,0.18);display:grid;grid-template-columns:1fr 1fr 72px 1fr 1fr;align-items:center;padding:7px 9px;box-shadow:0 22px 54px rgba(2,6,23,0.34),inset 0 1px 0 rgba(255,255,255,0.20),inset 0 -1px 0 rgba(0,0,0,0.30);box-sizing:border-box;backdrop-filter:blur(26px) saturate(170%);-webkit-backdrop-filter:blur(26px) saturate(170%);overflow:visible;z-index:30;";
pub(crate) const NAV_BUTTON: &str = "height:60px;border:0;border-radius:30px;background:transparent;color:rgba(255,255,255,0.82);display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:850;text-shadow:0 1px 1px rgba(0,0,0,0.20);";
pub(crate) const NAV_BUTTON_ACTIVE: &str = "height:60px;border:1px solid rgba(255,255,255,0.20);background:rgba(255,255,255,0.13);color:#ffffff;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;font-size:10px;font-weight:950;border-radius:30px;box-shadow:0 12px 26px rgba(0,0,0,0.26),inset 0 1px 0 rgba(255,255,255,0.22),inset 0 -1px 0 rgba(0,0,0,0.20);backdrop-filter:blur(18px) saturate(160%);-webkit-backdrop-filter:blur(18px) saturate(160%);";
pub(crate) const NAV_ICON: &str = "width:20px;height:20px;display:block;stroke:currentColor;";
pub(crate) const SOS_BUTTON: &str = "position:absolute;left:50%;bottom:40px;transform:translateX(-50%);width:64px;height:64px;border-radius:50%;border:1px solid rgba(255,255,255,0.30);background:#ef4444;color:#ffffff;font-size:19px;font-weight:900;letter-spacing:0;box-shadow:0 0 0 7px rgba(239,68,68,0.16),0 20px 34px rgba(239,68,68,0.34),0 14px 34px rgba(0,0,0,0.32),inset 0 1px 0 rgba(255,255,255,0.34),inset 0 -12px 22px rgba(127,29,29,0.30);display:flex;align-items:center;justify-content:center;backdrop-filter:blur(20px) saturate(170%);-webkit-backdrop-filter:blur(20px) saturate(170%);z-index:31;";
pub(crate) const SOS_BUTTON_ACTIVE: &str = "position:absolute;left:50%;bottom:39px;transform:translateX(-50%);width:66px;height:66px;border-radius:50%;border:1px solid rgba(255,255,255,0.30);background:#dc2626;color:#ffffff;font-size:14px;font-weight:950;letter-spacing:0;box-shadow:0 0 0 9px rgba(239,68,68,0.20),0 20px 38px rgba(220,38,38,0.44),0 14px 34px rgba(0,0,0,0.34),inset 0 1px 0 rgba(255,255,255,0.30),inset 0 -12px 22px rgba(127,29,29,0.30);display:flex;align-items:center;justify-content:center;backdrop-filter:blur(20px) saturate(170%);-webkit-backdrop-filter:blur(20px) saturate(170%);z-index:31;";
pub(crate) const DASHBOARD_WRAP: &str = "min-height:100dvh;background:radial-gradient(circle at 50% 108%,rgba(34,197,94,0.30),rgba(34,197,94,0) 36%),linear-gradient(145deg,#27272a 0%,#17181c 48%,#0b0f14 100%);padding:18px;box-sizing:border-box;color:#f8fafc;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;";
pub(crate) const BACK_LINK: &str = "display:inline-flex;align-items:center;gap:6px;color:#93c5fd;text-decoration:none;font-size:13px;font-weight:850;margin-bottom:18px;";
pub(crate) const DASH_TITLE: &str =
    "font-size:24px;line-height:1.1;font-weight:900;color:#f8fafc;margin:0 0 14px;";
pub(crate) const MOTION_CSS: &str = r##"
    @keyframes ja-splash-in { from { opacity:0; transform:scale(.94); } to { opacity:1; transform:scale(1); } }
    @keyframes ja-logo-float { 0%,100% { transform:translateY(0); } 50% { transform:translateY(-7px); } }
    @keyframes ja-panel-in { from { opacity:0; transform:translateY(10px); } to { opacity:1; transform:translateY(0); } }
    @keyframes ja-sos-pulse {
      0%,100% { box-shadow:0 0 0 7px rgba(239,68,68,.16),0 20px 38px rgba(127,29,29,.38),0 14px 34px rgba(0,0,0,.34),inset 0 1px 0 rgba(255,255,255,.30),inset 0 -12px 22px rgba(0,0,0,.24); }
      60% { box-shadow:0 0 0 15px rgba(239,68,68,0),0 20px 38px rgba(127,29,29,.38),0 14px 34px rgba(0,0,0,.34),inset 0 1px 0 rgba(255,255,255,.30),inset 0 -12px 22px rgba(0,0,0,.24); }
    }
    html, body, #main, #root, #app { width:100%; min-height:100%; margin:0; padding:0; background:#0b0f14; overflow:hidden; }
    body > div { min-height:100dvh; margin:0; padding:0; background:#0b0f14; }
    * { -webkit-tap-highlight-color: transparent; }
    *, *::before, *::after { box-sizing: border-box; }
    button, input, textarea { font-family: inherit; }
    button { cursor: pointer; transition: transform 160ms ease, filter 160ms ease, box-shadow 160ms ease; }
    button:active { transform: scale(.98); filter: brightness(.98); }
    .ja-sos-orb { overflow:hidden; isolation:isolate; }
    .ja-sos-orb:active { transform:translateX(-50%) scale(.98); }
    .ja-sos-orb::before {
      content:"";
      position:absolute;
      left:11px;
      right:11px;
      top:7px;
      height:19px;
      border-radius:999px;
      background:rgba(255,255,255,.22);
      opacity:.64;
      pointer-events:none;
      z-index:0;
    }
    .ja-sos-orb::after {
      content:"";
      position:absolute;
      inset:1px;
      border-radius:inherit;
      border-top:1px solid rgba(255,255,255,.24);
      border-bottom:1px solid rgba(0,0,0,.22);
      pointer-events:none;
      z-index:0;
    }
    .ja-sos-orb > span { position:relative; z-index:1; text-shadow:0 2px 5px rgba(0,0,0,.35); }
    input::placeholder, textarea::placeholder { color: rgba(203,213,225,.64); }
    .ja-content { animation:ja-panel-in 320ms ease-out both; }
    .ja-splash { animation:ja-splash-in 420ms ease-out both; }
    .ja-splash-logo { animation:ja-logo-float 2.4s ease-in-out infinite; }
    .ja-dock::before {
      content:"";
      position:absolute;
      left:18%;
      right:18%;
      bottom:-54px;
      height:74px;
      border-radius:999px;
      background:radial-gradient(circle at 50% 30%, rgba(34,197,94,.58), rgba(14,165,233,.22) 42%, rgba(14,165,233,0) 72%);
      filter:blur(22px);
      opacity:.74;
      pointer-events:none;
      z-index:-1;
    }
    .ja-dock::after {
      content:"";
      position:absolute;
      inset:1px;
      border-radius:inherit;
      border-top:1px solid rgba(255,255,255,.20);
      pointer-events:none;
    }
    .ja-sos-active { animation:ja-sos-pulse 1.25s ease-out infinite; }
    .ja-theme-light, .ja-theme-light * { text-shadow:none !important; }
    .ja-theme-light [style*="rgba(58,59,66"],
    .ja-theme-light [style*="rgba(52,54,62"],
    .ja-theme-light [style*="rgba(44,46,53"],
    .ja-theme-light [style*="rgba(31,34,40"],
    .ja-theme-light [style*="rgba(47,50,58"],
    .ja-theme-light [style*="rgba(255,255,255,0.07"],
    .ja-theme-light [style*="rgba(255,255,255,0.08"],
    .ja-theme-light [style*="rgba(255,255,255,0.09"],
    .ja-theme-light [style*="rgba(255,255,255,0.13"] {
      background:#ffffff !important;
      border-color:rgba(15,23,42,.12) !important;
      box-shadow:0 12px 30px rgba(15,23,42,.10), inset 0 1px 0 rgba(255,255,255,.92) !important;
    }
    .ja-theme-light [style*="#f8fafc"] { color:#0f172a !important; }
    .ja-theme-light [style*="#cbd5e1"],
    .ja-theme-light [style*="#d1d5db"] { color:#475569 !important; }
    .ja-theme-light [style*="#9ca3af"] { color:#64748b !important; }
    .ja-theme-light [style*="#93c5fd"],
    .ja-theme-light [style*="#bfdbfe"],
    .ja-theme-light [style*="#e0f2fe"] { color:#1d4ed8 !important; }
    .ja-theme-light input,
    .ja-theme-light textarea {
      background:#ffffff !important;
      color:#0f172a !important;
      border-color:rgba(15,23,42,.14) !important;
    }
    .ja-theme-light input::placeholder,
    .ja-theme-light textarea::placeholder { color:rgba(71,85,105,.58) !important; }
    .ja-theme-light .ja-dock {
      background:rgba(255,255,255,.92) !important;
      border-color:rgba(15,23,42,.12) !important;
      box-shadow:0 18px 42px rgba(15,23,42,.14), inset 0 1px 0 rgba(255,255,255,.96) !important;
    }
    .ja-theme-light .ja-dock::before { opacity:.18; }
    .ja-theme-light .ja-dock::after { border-top-color:rgba(255,255,255,.82); }
    .ja-theme-light .ja-nav-button {
      background:transparent !important;
      border-color:transparent !important;
      box-shadow:none !important;
      color:#475569 !important;
    }
    .ja-theme-light .ja-nav-button svg { stroke:#475569 !important; }
    .ja-theme-light .ja-nav-button span { color:#475569 !important; }
    .ja-theme-light .ja-nav-button.ja-nav-active {
      background:#e8f0ff !important;
      border-color:rgba(37,99,235,.18) !important;
      box-shadow:0 10px 22px rgba(37,99,235,.12), inset 0 1px 0 rgba(255,255,255,.92) !important;
      color:#1d4ed8 !important;
    }
    .ja-theme-light .ja-nav-button.ja-nav-active svg { stroke:#1d4ed8 !important; }
    .ja-theme-light .ja-nav-button.ja-nav-active span { color:#1d4ed8 !important; }
    .ja-theme-light .ja-sos-orb span { color:#ffffff !important; }
    .ja-settings-list .ja-settings-row:last-child { border-bottom:0 !important; }
    .ja-theme-light .ja-settings-list {
      background:#ffffff !important;
      border-color:rgba(15,23,42,.12) !important;
      box-shadow:inset 0 1px 0 rgba(255,255,255,.92),0 10px 24px rgba(15,23,42,.06) !important;
    }
    .ja-theme-light .ja-settings-row {
      border-bottom-color:rgba(15,23,42,.10) !important;
      background:transparent !important;
      box-shadow:none !important;
    }
    .ja-theme-light .ja-settings-switch {
      background:rgba(100,116,139,.22) !important;
      border-color:rgba(100,116,139,.18) !important;
    }
    .ja-theme-light .ja-settings-switch-on {
      background:#3b82f6 !important;
      border-color:transparent !important;
    }
    .ja-theme-light .ja-settings-switch-knob {
      background:#ffffff !important;
      box-shadow:0 4px 12px rgba(15,23,42,.24) !important;
    }
"##;

pub(crate) const THEME: ThemeSpec = ThemeSpec {
    app: APP,
    screen: SCREEN,
    header: HEADER,
    brand_wrap: BRAND_WRAP,
    brand: BRAND,
    subtitle: SUBTITLE,
    header_logo: HEADER_LOGO,
    icon_button: ICON_BUTTON,
    header_actions: HEADER_ACTIONS,
    language_toggle: LANGUAGE_TOGGLE,
    language_segment_active: LANGUAGE_SEGMENT_ACTIVE,
    language_segment_idle: LANGUAGE_SEGMENT_IDLE,
    content: CONTENT,
    map_card: MAP_CARD,
    route_map_card: ROUTE_MAP_CARD,
    map_iframe: MAP_IFRAME,
    map_label: MAP_LABEL,
    map_provider: MAP_PROVIDER,
    card: CARD,
    card_tight: CARD_TIGHT,
    row: ROW,
    eyebrow: EYEBROW,
    title: TITLE,
    body: BODY,
    meta_grid: META_GRID,
    meta_cell: META_CELL,
    meta_value: META_VALUE,
    meta_label: META_LABEL,
    field_grid: FIELD_GRID,
    input: INPUT,
    textarea: TEXTAREA,
    primary_button: PRIMARY_BUTTON,
    secondary_button: SECONDARY_BUTTON,
    category_grid: CATEGORY_GRID,
    category_button: CATEGORY_BUTTON,
    category_button_active: CATEGORY_BUTTON_ACTIVE,
    bottom_bar: BOTTOM_BAR,
    nav_button: NAV_BUTTON,
    nav_button_active: NAV_BUTTON_ACTIVE,
    nav_icon: NAV_ICON,
    sos_button: SOS_BUTTON,
    sos_button_active: SOS_BUTTON_ACTIVE,
    dashboard_wrap: DASHBOARD_WRAP,
    back_link: BACK_LINK,
    dash_title: DASH_TITLE,
    motion_css: MOTION_CSS,
};
