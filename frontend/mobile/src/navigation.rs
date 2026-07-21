use dioxus::prelude::*;

use crate::app_config::{app_spec, CopyKey, Language, MobileTab, NavIconKind};
use crate::theme::*;

#[component]
pub(crate) fn BottomNavigation(
    active_tab: MobileTab,
    language: Language,
    on_tab: EventHandler<MobileTab>,
) -> Element {
    rsx! {
        nav { style: BOTTOM_BAR, class: "ja-dock",
            for screen in app_spec().nav_screens().iter().filter(|screen| {
                screen.nav.is_some() && screen.nav_slot.unwrap_or(0) < 2
            }) {
                {
                    let tab = screen.tab;
                    let nav = screen.nav.expect("nav screen must have nav metadata");
                    rsx! {
                        NavButton {
                            active: active_tab == tab,
                            icon: nav.icon,
                            label: language.text(nav.label),
                            onclick: move |_| on_tab.call(tab),
                        }
                    }
                }
            }
            div {}
            for screen in app_spec().nav_screens().iter().filter(|screen| {
                screen.nav.is_some() && screen.nav_slot.unwrap_or(0) > 2
            }) {
                {
                    let tab = screen.tab;
                    let nav = screen.nav.expect("nav screen must have nav metadata");
                    rsx! {
                        NavButton {
                            active: active_tab == tab,
                            icon: nav.icon,
                            label: language.text(nav.label),
                            onclick: move |_| on_tab.call(tab),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn SosButton(active: bool, language: Language, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            style: if active { SOS_BUTTON_ACTIVE } else { SOS_BUTTON },
            class: if active { "ja-sos-orb ja-sos-active" } else { "ja-sos-orb" },
            onclick: move |_| on_click.call(()),
            span {
                {if active {
                    language.text(CopyKey::StopSosButton)
                } else {
                    language.text(CopyKey::SosButton)
                }}
            }
        }
    }
}

#[component]
pub(crate) fn NavButton(
    active: bool,
    icon: NavIconKind,
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: if active { NAV_BUTTON_ACTIVE } else { NAV_BUTTON },
            class: if active { "ja-nav-button ja-nav-active" } else { "ja-nav-button" },
            onclick: move |event| onclick.call(event),
            NavIcon { icon }
            span { "{label}" }
        }
    }
}

#[component]
fn NavIcon(icon: NavIconKind) -> Element {
    match icon {
        NavIconKind::Map => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M9 18 3 21V6l6-3 6 3 6-3v15l-6 3-6-3Z" }
                path { d: "M9 3v15" }
                path { d: "M15 6v15" }
            }
        },
        NavIconKind::Route => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "6", cy: "19", r: "2.2" }
                circle { cx: "18", cy: "5", r: "2.2" }
                path { d: "M8.2 19H17a4 4 0 0 0 0-8H7a4 4 0 0 1 0-8h8.8" }
            }
        },
        NavIconKind::Contacts => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H7a4 4 0 0 0-4 4v2" }
                circle { cx: "9.5", cy: "7", r: "4" }
                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
            }
        },
        NavIconKind::Profile => rsx! {
            svg {
                style: NAV_ICON,
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "8", r: "4" }
                path { d: "M4 21a8 8 0 0 1 16 0" }
            }
        },
    }
}
