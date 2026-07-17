//! Google Maps / MapLibre component — web-only (uses JS interop).
use crate::utils::js;
use dioxus::prelude::*;
use dioxus::document::eval;
use crate::services::api::HeatmapPoint;

#[derive(Props, Clone, PartialEq)]
pub struct HeatmapMapViewProps {
    pub points: Vec<HeatmapPoint>,
    pub center_lat: f64,
    pub center_lng: f64,
}

#[component]
pub fn HeatmapMapView(props: HeatmapMapViewProps) -> Element {
    use_effect(move || {
        let points_json = serde_json::to_string(&props.points).unwrap_or_else(|_| "[]".to_string());
        
        let lat = props.center_lat;
        let lng = props.center_lng;

        let mut map_eval = eval(r#"
            let [raw_points, lat, lng] = await dioxus.recv();
            setTimeout(() => {
                if (window.ja_initMap) {
                    window.ja_initMap(lat, lng);
                    if (window.ja_initHeatmap) {
                        window.ja_initHeatmap(raw_points);
                    }
                }
            }, 50);
        "#);
        
        let _ = map_eval.send((points_json, lat, lng));
    });

    rsx! {
        div {
            id: "map-container",
            class: "w-full h-full min-h-[450px] bg-slate-900",
        }
    }
}