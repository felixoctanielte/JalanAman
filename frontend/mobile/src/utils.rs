use jalanaman_shared::{Report, RouteScoreResponse, Waypoint};

use crate::app_config::{CopyKey, Language};
use crate::models::GeoPoint;

pub(crate) fn local_route_score(waypoints: &[Waypoint], reports: &[Report]) -> RouteScoreResponse {
    let mut score = 0.0;
    let mut report_count = 0;

    for report in reports {
        if distance_to_route_m(report.lat, report.lng, waypoints) <= 50.0 {
            report_count += 1;
            score += match report.category.as_str() {
                "crime" => 3.0,
                "accident" => 2.0,
                "lighting" => 1.0,
                _ => 1.0,
            };
        }
    }

    let level = if score < 5.0 {
        "Aman"
    } else if score < 15.0 {
        "Waspada"
    } else {
        "Hindari"
    };

    RouteScoreResponse {
        score,
        level: level.to_string(),
        report_count,
        cache_hit: false,
    }
}

fn distance_to_route_m(lat: f64, lng: f64, waypoints: &[Waypoint]) -> f64 {
    match waypoints {
        [] => f64::INFINITY,
        [only] => haversine_m(lat, lng, only.lat, only.lng),
        points => points
            .windows(2)
            .map(|segment| distance_to_route_segment_m(lat, lng, &segment[0], &segment[1]))
            .fold(f64::INFINITY, f64::min),
    }
}

fn distance_to_route_segment_m(lat: f64, lng: f64, a: &Waypoint, b: &Waypoint) -> f64 {
    let origin_lat = ((lat + a.lat + b.lat) / 3.0).to_radians();
    let meters_per_deg_lat = 111_320.0;
    let meters_per_deg_lng = 111_320.0 * origin_lat.cos().abs().max(0.01);

    let px = (lng - a.lng) * meters_per_deg_lng;
    let py = (lat - a.lat) * meters_per_deg_lat;
    let bx = (b.lng - a.lng) * meters_per_deg_lng;
    let by = (b.lat - a.lat) * meters_per_deg_lat;
    let len_sq = bx * bx + by * by;

    if len_sq <= f64::EPSILON {
        return (px * px + py * py).sqrt();
    }

    let t = ((px * bx + py * by) / len_sq).clamp(0.0, 1.0);
    let dx = px - t * bx;
    let dy = py - t * by;

    (dx * dx + dy * dy).sqrt()
}

pub(crate) fn level_bg(level: &str) -> &'static str {
    match level {
        "Aman" => "rgba(37,99,235,0.24)",
        "Waspada" => "rgba(146,64,14,0.28)",
        _ => "rgba(127,29,29,0.30)",
    }
}

pub(crate) fn level_color(level: &str) -> &'static str {
    match level {
        "Aman" => "#bfdbfe",
        "Waspada" => "#fde68a",
        _ => "#fecdd3",
    }
}

pub(crate) fn distance_label(meters: f64) -> String {
    if meters < 1000.0 {
        format!("{:.0} m", meters)
    } else {
        format!("{:.1} km", meters / 1000.0)
    }
}

pub(crate) fn duration_label(seconds: f64, language: Language) -> String {
    let minutes = (seconds / 60.0).round().max(1.0);
    if minutes < 60.0 {
        if language.is_indonesian() {
            format!("{minutes:.0} mnt")
        } else {
            format!("{minutes:.0} min")
        }
    } else {
        if language.is_indonesian() {
            format!("{:.1} jam", minutes / 60.0)
        } else {
            format!("{:.1} h", minutes / 60.0)
        }
    }
}

pub(crate) fn localized_level(level: &str, language: Language) -> &'static str {
    match language {
        Language::Indonesian => match level {
            "Aman" => "Aman",
            "Waspada" => "Waspada",
            _ => "Hindari",
        },
        Language::English => match level {
            "Aman" => "Safe",
            "Waspada" => "Caution",
            _ => "Avoid",
        },
    }
}

pub(crate) fn route_overlay_title(score: &RouteScoreResponse, language: Language) -> String {
    if language.is_indonesian() {
        format!(
            "{} dengan {} {}",
            localized_level(&score.level, language),
            score.report_count,
            language.text(CopyKey::RouteReportsSuffix)
        )
    } else {
        format!(
            "{} with {} {}",
            localized_level(&score.level, language),
            score.report_count,
            language.text(CopyKey::RouteReportsSuffix)
        )
    }
}

pub(crate) fn haversine_m(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let radius = 6_371_000.0_f64;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lng / 2.0).sin().powi(2);
    radius * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
}

pub(crate) fn parse_manual_location(lat_text: &str, lng_text: &str) -> Result<GeoPoint, CopyKey> {
    let lat = parse_coordinate(lat_text).ok_or(CopyKey::LatitudeInvalid)?;
    let lng = parse_coordinate(lng_text).ok_or(CopyKey::LongitudeInvalid)?;

    if !(-90.0..=90.0).contains(&lat) {
        return Err(CopyKey::LatitudeRange);
    }

    if !(-180.0..=180.0).contains(&lng) {
        return Err(CopyKey::LongitudeRange);
    }

    Ok(GeoPoint { lat, lng })
}

fn parse_coordinate(value: &str) -> Option<f64> {
    value
        .trim()
        .replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|coord| coord.is_finite())
}

pub(crate) fn limit_text(value: String, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}
