use dioxus::prelude::*;

use crate::app_config::{app_spec, CopyKey, Language};
use crate::theme::*;

#[component]
pub(crate) fn Header(
    language: Language,
    on_language: EventHandler<Language>,
    on_help: EventHandler<()>,
) -> Element {
    let next_language = language.toggled();
    let brand = app_spec().brand;

    rsx! {
        header { style: HEADER,
            div { style: BRAND_WRAP,
                img { src: app_spec().assets.logo, alt: brand.logo_alt, style: HEADER_LOGO }
                div { style: "min-width:0;",
                    div { style: BRAND, "{brand.name}" }
                    div { style: SUBTITLE, "{language.text(CopyKey::HeaderSubtitle)}" }
                }
            }
            div { style: HEADER_ACTIONS,
                button {
                    style: LANGUAGE_TOGGLE,
                    title: "{language.text(CopyKey::LanguageToggleTitle)}",
                    onclick: move |_| on_language.call(next_language),
                    span { style: if language.is_indonesian() { LANGUAGE_SEGMENT_ACTIVE } else { LANGUAGE_SEGMENT_IDLE }, "ID" }
                    span { style: if language.is_indonesian() { LANGUAGE_SEGMENT_IDLE } else { LANGUAGE_SEGMENT_ACTIVE }, "EN" }
                }
                button {
                    style: ICON_BUTTON,
                    title: "{language.text(CopyKey::HeaderHelpTitle)}",
                    onclick: move |_| on_help.call(()),
                    span { style: "font-size:22px;line-height:1;font-weight:950;color:#e0f2fe;text-shadow:0 1px 1px rgba(0,0,0,0.26);", "?" }
                }
            }
        }
    }
}
