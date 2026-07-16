use axum::{extract::Query, Json};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;

use crate::error::AppError;

const NOMINATIM_URL: &str = "https://nominatim.openstreetmap.org/search";
const PHOTON_URL: &str = "https://photon.komoot.io/api/";
const OSRM_BASE_URL: &str = "https://router.project-osrm.org/route/v1";
const DEFAULT_NOMINATIM_USER_AGENT: &str =
    "JalanAman/0.1 (development; contact: felix@jalanaman.local)";
const DEFAULT_NOMINATIM_EMAIL: &str = "felix@jalanaman.local";

#[derive(Debug, Deserialize)]
pub struct DirectionsParams {
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination: String,
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Waypoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize)]
pub struct DirectionsResponse {
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub waypoints: Vec<Waypoint>,
    pub polyline: Vec<Waypoint>,
    pub distance_m: f64,
    pub duration_s: f64,
    pub provider: String,
}

#[derive(Debug, Deserialize)]
struct NominatimResult {
    lat: String,
    lon: String,
}

#[derive(Debug)]
struct GeocodedDestination {
    point: Waypoint,
    provider: String,
}

#[derive(Debug, Deserialize)]
struct PhotonResponse {
    features: Vec<PhotonFeature>,
}

#[derive(Debug, Deserialize)]
struct PhotonFeature {
    geometry: PhotonGeometry,
}

#[derive(Debug, Deserialize)]
struct PhotonGeometry {
    coordinates: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct OsrmResponse {
    code: String,
    routes: Vec<OsrmRoute>,
}

#[derive(Debug, Deserialize)]
struct OsrmRoute {
    geometry: OsrmGeometry,
    distance: f64,
    duration: f64,
}

#[derive(Debug, Deserialize)]
struct OsrmGeometry {
    coordinates: Vec<[f64; 2]>,
}

pub async fn get_directions(
    Query(params): Query<DirectionsParams>,
) -> Result<Json<DirectionsResponse>, AppError> {
    validate_origin(params.origin_lat, params.origin_lng)?;

    let destination = params.destination.trim();
    if destination.len() < 3 {
        return Err(AppError::BadRequest(
            "Tujuan minimal 3 karakter".to_string(),
        ));
    }

    let client = reqwest::Client::builder()
        .default_headers(default_headers()?)
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let destination_point = geocode_destination(&client, destination).await?;
    let profile = osrm_profile(params.mode.as_deref());
    let route = fetch_osrm_route(
        &client,
        profile,
        params.origin_lat,
        params.origin_lng,
        destination_point.point.lat,
        destination_point.point.lng,
    )
    .await?;

    if route.geometry.coordinates.len() < 2 {
        return Err(AppError::BadRequest("Rute tidak ditemukan".to_string()));
    }

    let polyline: Vec<Waypoint> = route
        .geometry
        .coordinates
        .iter()
        .map(|point| Waypoint {
            lng: point[0],
            lat: point[1],
        })
        .collect();

    let waypoints = sample_waypoints(&polyline, 24);

    Ok(Json(DirectionsResponse {
        destination_lat: destination_point.point.lat,
        destination_lng: destination_point.point.lng,
        waypoints,
        polyline,
        distance_m: route.distance,
        duration_s: route.duration,
        provider: format!("{} + OSRM", destination_point.provider),
    }))
}

fn default_headers() -> Result<HeaderMap, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&nominatim_user_agent())
            .map_err(|e| AppError::Internal(e.to_string()))?,
    );
    Ok(headers)
}

fn nominatim_user_agent() -> String {
    env::var("NOMINATIM_USER_AGENT").unwrap_or_else(|_| DEFAULT_NOMINATIM_USER_AGENT.to_string())
}

fn nominatim_email() -> String {
    env::var("NOMINATIM_EMAIL").unwrap_or_else(|_| DEFAULT_NOMINATIM_EMAIL.to_string())
}

fn validate_origin(lat: f64, lng: f64) -> Result<(), AppError> {
    if !lat.is_finite() || !lng.is_finite() || lat.abs() > 90.0 || lng.abs() > 180.0 {
        return Err(AppError::BadRequest(
            "Koordinat asal tidak valid".to_string(),
        ));
    }
    Ok(())
}

fn osrm_profile(mode: Option<&str>) -> &'static str {
    match mode {
        Some("driving") => "driving",
        Some("car") => "driving",
        _ => "foot",
    }
}

async fn geocode_destination(
    client: &reqwest::Client,
    destination: &str,
) -> Result<GeocodedDestination, AppError> {
    if let Some(point) = parse_coordinate_destination(destination)? {
        return Ok(GeocodedDestination {
            point,
            provider: "Koordinat manual".to_string(),
        });
    }

    match geocode_with_nominatim(client, destination).await {
        Ok(point) => {
            return Ok(GeocodedDestination {
                point,
                provider: "Nominatim".to_string(),
            });
        }
        Err(err) => {
            tracing::warn!("Nominatim geocode failed, falling back to Photon: {err}");
        }
    }

    match geocode_with_photon(client, destination).await {
        Ok(point) => Ok(GeocodedDestination {
            point,
            provider: "Photon".to_string(),
        }),
        Err(err) => Err(AppError::BadRequest(format!(
            "Tujuan tidak ditemukan. Coba tulis lebih spesifik atau pakai format lat,lng. Detail: {err}"
        ))),
    }
}

async fn geocode_with_nominatim(
    client: &reqwest::Client,
    destination: &str,
) -> Result<Waypoint, String> {
    let email = nominatim_email();
    let response = client
        .get(NOMINATIM_URL)
        .query(&[
            ("q", destination),
            ("format", "json"),
            ("limit", "1"),
            ("countrycodes", "id"),
            ("email", email.as_str()),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Nominatim HTTP {status}: {body}"));
    }

    let results = response
        .json::<Vec<NominatimResult>>()
        .await
        .map_err(|e| e.to_string())?;

    let first = results
        .first()
        .ok_or_else(|| "Tujuan tidak ditemukan di Nominatim".to_string())?;

    let lat = first
        .lat
        .parse::<f64>()
        .map_err(|_| "Koordinat tujuan tidak valid".to_string())?;
    let lng = first
        .lon
        .parse::<f64>()
        .map_err(|_| "Koordinat tujuan tidak valid".to_string())?;

    validate_origin(lat, lng).map_err(|e| e.to_string())?;
    Ok(Waypoint { lat, lng })
}

async fn geocode_with_photon(
    client: &reqwest::Client,
    destination: &str,
) -> Result<Waypoint, String> {
    let query = format!("{destination}, Indonesia");
    let response = client
        .get(PHOTON_URL)
        .query(&[("q", query.as_str()), ("limit", "1")])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Photon HTTP {status}: {body}"));
    }

    let data = response
        .json::<PhotonResponse>()
        .await
        .map_err(|e| e.to_string())?;

    let coordinates = data
        .features
        .first()
        .map(|feature| feature.geometry.coordinates.as_slice())
        .ok_or_else(|| "Tujuan tidak ditemukan di Photon".to_string())?;

    let [lng, lat] = coordinates else {
        return Err("Koordinat tujuan Photon tidak valid".to_string());
    };

    validate_origin(*lat, *lng).map_err(|e| e.to_string())?;
    Ok(Waypoint {
        lat: *lat,
        lng: *lng,
    })
}

fn parse_coordinate_destination(destination: &str) -> Result<Option<Waypoint>, AppError> {
    let cleaned = destination.replace(',', " ").replace(';', " ");
    let normalized = cleaned
        .split_whitespace()
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if normalized.len() != 2 {
        return Ok(None);
    }

    let Ok(lat) = normalized[0].parse::<f64>() else {
        return Ok(None);
    };
    let Ok(lng) = normalized[1].parse::<f64>() else {
        return Ok(None);
    };

    validate_origin(lat, lng)?;
    Ok(Some(Waypoint { lat, lng }))
}

async fn fetch_osrm_route(
    client: &reqwest::Client,
    profile: &str,
    origin_lat: f64,
    origin_lng: f64,
    destination_lat: f64,
    destination_lng: f64,
) -> Result<OsrmRoute, AppError> {
    let url = format!(
        "{OSRM_BASE_URL}/{profile}/{origin_lng},{origin_lat};{destination_lng},{destination_lat}"
    );

    let data = client
        .get(url)
        .query(&[("overview", "full"), ("geometries", "geojson")])
        .send()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .error_for_status()
        .map_err(|e| AppError::Internal(e.to_string()))?
        .json::<OsrmResponse>()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if data.code != "Ok" {
        return Err(AppError::BadRequest("Rute tidak ditemukan".to_string()));
    }

    data.routes
        .into_iter()
        .next()
        .ok_or_else(|| AppError::BadRequest("Rute tidak ditemukan".to_string()))
}

fn sample_waypoints(points: &[Waypoint], target_count: usize) -> Vec<Waypoint> {
    if points.len() <= target_count {
        return points.to_vec();
    }

    let step = ((points.len() - 1) as f64 / (target_count - 1) as f64).ceil() as usize;
    let mut sampled: Vec<Waypoint> = points.iter().step_by(step.max(1)).cloned().collect();

    if sampled.last() != points.last() {
        if let Some(last) = points.last() {
            sampled.push(last.clone());
        }
    }

    sampled
}
