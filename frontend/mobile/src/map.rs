use jalanaman_shared::{Report, Waypoint};
use serde::Serialize;

use crate::app_config::{app_spec, CopyKey, Language};
use crate::models::GeoPoint;

#[derive(Serialize)]
struct MapReport {
    id: String,
    category: String,
    lat: f64,
    lng: f64,
    note: Option<String>,
}

pub(crate) fn map_srcdoc(
    location: Option<GeoPoint>,
    reports: &[Report],
    route: Option<&[Waypoint]>,
    route_level: Option<&str>,
    three_dimensional: bool,
    language: Language,
    selectable: bool,
) -> String {
    let location_json = serde_json::to_string(&location).unwrap_or_else(|_| "null".to_string());
    let reports_json = serde_json::to_string(
        &reports
            .iter()
            .map(|report| MapReport {
                id: report.id.clone(),
                category: report.category.clone(),
                lat: report.lat,
                lng: report.lng,
                note: report.note.clone(),
            })
            .collect::<Vec<_>>(),
    )
    .unwrap_or_else(|_| "[]".to_string());
    let route_json =
        serde_json::to_string(&route.unwrap_or(&[])).unwrap_or_else(|_| "[]".to_string());
    let route_level_json = serde_json::to_string(&route_level.unwrap_or("Aman"))
        .unwrap_or_else(|_| "\"Aman\"".to_string());
    let three_dimensional_json = if three_dimensional { "true" } else { "false" };
    let selectable_json = if selectable { "true" } else { "false" };
    let copy = &app_spec().copy;
    let loading_title = copy.text(language, CopyKey::MapSrcLoadingTitle);
    let loading_body = copy.text(language, CopyKey::MapSrcLoadingBody);
    let map_failed_title = copy.text(language, CopyKey::MapSrcFailedTitle);
    let map_failed_body = copy.text(language, CopyKey::MapSrcFailedBody);
    let three_d_failed_title = copy.text(language, CopyKey::MapSrcThreeDFailedTitle);
    let three_d_failed_body = copy.text(language, CopyKey::MapSrcThreeDFailedBody);
    let mark_hint = copy.text(language, CopyKey::MapSrcMarkHint);
    let mark_confirm = copy.text(language, CopyKey::MapSrcMarkConfirm);
    let mark_cancel = copy.text(language, CopyKey::MapSrcMarkCancel);
    let mark_accept = copy.text(language, CopyKey::MapSrcMarkAccept);

    r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <link rel="stylesheet" href="https://unpkg.com/maplibre-gl@5.24.0/dist/maplibre-gl.css" />
  <style>
    html, body, #map { margin:0; width:100%; height:100%; overflow:hidden; background:linear-gradient(145deg,#27272a,#10141a); }
    #map { position:relative; touch-action:none; cursor:grab; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; }
    #map.dragging { cursor:grabbing; }
    #viewport { position:absolute; inset:0; }
    #tiles, #overlay, #points { position:absolute; inset:0; }
    #tiles img { position:absolute; width:256px; height:256px; image-rendering:auto; user-select:none; -webkit-user-drag:none; pointer-events:none; }
    #overlay { pointer-events:none; z-index:3; }
    #points { z-index:4; pointer-events:none; }
    #fallback { position:absolute; inset:0; z-index:6; display:flex; flex-direction:column; align-items:center; justify-content:center; gap:6px; padding:24px; box-sizing:border-box; background:linear-gradient(145deg,rgba(39,40,45,.92),rgba(13,18,24,.86)); color:#f8fafc; font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; text-align:center; backdrop-filter:blur(14px) saturate(150%); -webkit-backdrop-filter:blur(14px) saturate(150%); }
    #fallback strong { font-size:14px; font-weight:900; }
    #fallback span { max-width:220px; font-size:11px; font-weight:700; line-height:1.4; color:#cbd5e1; }
    .pin { position:absolute; width:24px; height:24px; margin:-12px 0 0 -12px; border-radius:50%; border:3px solid #fff; box-shadow:0 10px 18px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; color:#fff; font:900 11px system-ui; }
    .me { position:absolute; width:18px; height:18px; margin:-9px 0 0 -9px; border-radius:50%; background:#1d4ed8; border:3px solid #fff; box-shadow:0 0 0 14px rgba(37,99,235,.17),0 10px 22px rgba(15,23,42,.26); }
    .route-end { position:absolute; width:28px; height:28px; margin:-28px 0 0 -14px; border-radius:9px 9px 9px 2px; transform:rotate(-45deg); background:#1d4ed8; border:3px solid #fff; box-shadow:0 12px 22px rgba(15,23,42,.25); display:flex; align-items:center; justify-content:center; }
    .route-end span { transform:rotate(45deg); color:#fff; font:900 12px system-ui; }
    #zoomctl { position:absolute; right:10px; top:50%; transform:translateY(-50%); z-index:5; display:flex; flex-direction:column; border-radius:12px; overflow:hidden; border:1px solid rgba(255,255,255,.18); box-shadow:0 12px 24px rgba(15,23,42,.24),inset 0 1px 0 rgba(255,255,255,.16); backdrop-filter:blur(18px) saturate(170%); -webkit-backdrop-filter:blur(18px) saturate(170%); }
    #zoomctl button { display:block; width:36px; height:36px; border:0; background:rgba(24,27,34,0.72); color:#bfdbfe; font:950 19px/36px system-ui; padding:0; }
    #zoomctl button:first-child { border-bottom:1px solid rgba(255,255,255,.12); }
    #markhint { position:absolute; left:50%; bottom:12px; transform:translateX(-50%); z-index:5; white-space:nowrap; border:1px solid rgba(255,255,255,.20); border-radius:999px; background:rgba(24,27,34,.80); color:#fff; padding:8px 12px; font:850 10px system-ui; box-shadow:0 10px 22px rgba(0,0,0,.24); pointer-events:none; }
    @keyframes person-pulse { 0%,100% { transform:scale(1); opacity:.52; } 50% { transform:scale(1.45); opacity:.18; } }
    #reportperson { position:absolute; right:12px; top:76px; z-index:9; width:42px; height:48px; border:1px solid rgba(255,255,255,.72); border-radius:12px; background:rgba(255,255,255,.96); color:#f59e0b; display:flex; align-items:center; justify-content:center; font:950 27px/1 system-ui; box-shadow:0 12px 28px rgba(0,0,0,.28); touch-action:none; user-select:none; -webkit-user-select:none; cursor:grab; transition:transform 150ms ease,box-shadow 150ms ease; will-change:left,top,transform; }
    #reportperson::before { content:""; position:absolute; left:9px; right:9px; bottom:-8px; height:8px; border-radius:50%; background:rgba(15,23,42,.38); filter:blur(3px); transition:all 150ms ease; z-index:-1; }
    #reportperson.dragging { cursor:grabbing; transform:translateY(-12px) scale(1.16); box-shadow:0 22px 38px rgba(0,0,0,.38); }
    #reportperson.dragging::before { left:3px; right:3px; bottom:-19px; opacity:.62; animation:person-pulse .9s ease-in-out infinite; }
    #dragcoords { position:absolute; left:50%; top:-27px; transform:translateX(-50%); display:none; white-space:nowrap; padding:5px 8px; border-radius:7px; background:rgba(15,23,42,.92); border:1px solid rgba(255,255,255,.18); color:#fff; font:850 9px/1 system-ui; box-shadow:0 8px 18px rgba(0,0,0,.28); pointer-events:none; }
    #reportperson.dragging #dragcoords { display:block; }
    #selectedline { position:absolute; inset:0; z-index:4; pointer-events:none; }
    #confirmmark { position:absolute; left:50%; bottom:14px; transform:translateX(-50%); width:calc(100% - 28px); max-width:340px; z-index:12; display:none; padding:12px; border:1px solid rgba(255,255,255,.20); border-radius:12px; background:rgba(24,27,34,.96); color:#fff; box-shadow:0 18px 38px rgba(0,0,0,.42); font-family:system-ui; }
    #confirmmark strong { display:block; font-size:12px; }
    #confirmmark span { display:block; margin-top:3px; color:#cbd5e1; font-size:10px; }
    #confirmmark .actions { display:grid; grid-template-columns:1fr 1fr; gap:7px; margin-top:10px; }
    #confirmmark button { height:35px; border-radius:8px; border:1px solid rgba(255,255,255,.18); font:900 11px system-ui; }
    #cancelmark { background:rgba(255,255,255,.08); color:#cbd5e1; }
    #acceptmark { background:#1d4ed8; color:#fff; }
    #map.is-3d #viewport, #map.is-3d #zoomctl { display:none; }
    .ja-me-3d { width:20px; height:20px; border-radius:50%; background:#1d4ed8; border:3px solid #fff; box-shadow:0 0 0 14px rgba(37,99,235,.17),0 10px 22px rgba(15,23,42,.26); box-sizing:border-box; }
    #map.is-3d .maplibregl-ctrl-top-right { top:82px; right:10px; }
    #map.is-3d .maplibregl-ctrl-top-right .maplibregl-ctrl { margin:0; }
    .maplibregl-ctrl-group { border-radius:8px; overflow:hidden; border:1px solid rgba(255,255,255,.22); box-shadow:0 12px 24px rgba(15,23,42,.24); backdrop-filter:blur(18px) saturate(170%); -webkit-backdrop-filter:blur(18px) saturate(170%); }
  </style>
</head>
<body>
<div id="map">
  <div id="viewport">
    <div id="tiles"></div>
    <svg id="overlay"></svg>
    <div id="points"></div>
  </div>
  <div id="zoomctl">
    <button id="zoomIn" type="button">+</button>
    <button id="zoomOut" type="button">&minus;</button>
  </div>
  <svg id="selectedline"></svg>
  <div id="reportperson" aria-label="__MARK_HINT__">🚶<span id="dragcoords"></span></div>
  <div id="markhint">__MARK_HINT__</div>
  <div id="confirmmark">
    <strong>__MARK_CONFIRM__</strong>
    <span id="confirmcoords"></span>
    <div class="actions">
      <button id="cancelmark" type="button">__MARK_CANCEL__</button>
      <button id="acceptmark" type="button">__MARK_ACCEPT__</button>
    </div>
  </div>
  <div id="fallback"><strong>__LOADING_TITLE__</strong><span>__LOADING_BODY__</span></div>
</div>
<script src="https://unpkg.com/maplibre-gl@5.24.0/dist/maplibre-gl.js"></script>
<script>
const locationPoint = __LOCATION__;
const reports = __REPORTS__;
const route = __ROUTE__;
const routeLevel = __ROUTE_LEVEL__;
const threeDimensional = __THREE_DIMENSIONAL__;
const selectable = __SELECTABLE__;
const mapEl = document.getElementById('map');
const viewportEl = document.getElementById('viewport');
const tilesEl = document.getElementById('tiles');
const pointsEl = document.getElementById('points');
const overlay = document.getElementById('overlay');
const fallback = document.getElementById('fallback');
const markHint = document.getElementById('markhint');
const reportPerson = document.getElementById('reportperson');
const dragCoords = document.getElementById('dragcoords');
const selectedLine = document.getElementById('selectedline');
const confirmMark = document.getElementById('confirmmark');
const confirmCoords = document.getElementById('confirmcoords');
if (!selectable) {
  markHint.style.display = 'none';
  reportPerson.style.display = 'none';
}
const MIN_ZOOM = 3;
const MAX_ZOOM = 18;
let activeCenter = null;
let activeZoom = null;
let map3d = null;
let pendingReportPoint = null;
function showFallback(title, body) {
  fallback.style.display = 'flex';
  fallback.innerHTML = `<strong>${title}</strong><span>${body}</span>`;
}
const colors = { lighting:'#f59e0b', crime:'#ef4444', accident:'#f97316', other:'#94a3b8' };
const levelColors = { Aman:'#3b82f6', Waspada:'#f59e0b', Hindari:'#ef4444' };
const firstReport = reports[0];
const tileSize = 256;

function renderThreeDimensionalMap() {
  mapEl.classList.add('is-3d');
  if (!window.maplibregl) {
    showFallback('__THREE_D_FAILED_TITLE__', '__THREE_D_FAILED_BODY__');
    return;
  }

  const map = new maplibregl.Map({
    container: 'map',
    style: 'https://tiles.openfreemap.org/styles/liberty',
    center: baseCenter(),
    zoom: route.length > 1 ? 13.5 : (locationPoint || firstReport ? 16 : 11),
    pitch: 54,
    bearing: -24,
    maxPitch: 70,
    attributionControl: false,
    canvasContextAttributes: { antialias: true },
  });
  map3d = map;

  let styleReady = false;
  map.addControl(new maplibregl.NavigationControl({ showCompass: true }), 'top-right');
  map.on('load', () => {
    styleReady = true;
    fallback.style.display = 'none';

    const layers = map.getStyle().layers || [];
    const labelLayer = layers.find(layer => layer.type === 'symbol' && layer.layout && layer.layout['text-field']);
    map.addSource('jalanaman-buildings', { type: 'vector', url: 'https://tiles.openfreemap.org/planet' });
    map.addLayer({
      id: 'jalanaman-3d-buildings',
      source: 'jalanaman-buildings',
      'source-layer': 'building',
      type: 'fill-extrusion',
      minzoom: 15,
      filter: ['!=', ['get', 'hide_3d'], true],
      paint: {
        'fill-extrusion-color': ['interpolate', ['linear'], ['get', 'render_height'], 0, '#cbd5e1', 80, '#60a5fa', 220, '#1d4ed8'],
        'fill-extrusion-height': ['interpolate', ['linear'], ['zoom'], 15, 0, 16, ['get', 'render_height']],
        'fill-extrusion-base': ['case', ['>=', ['zoom'], 16], ['get', 'render_min_height'], 0],
        'fill-extrusion-opacity': 0.88,
      },
    }, labelLayer && labelLayer.id);

    if (route.length > 1) {
      map.addSource('jalanaman-route', {
        type: 'geojson',
        data: { type: 'Feature', properties: {}, geometry: { type: 'LineString', coordinates: route.map(point => [point.lng, point.lat]) } },
      });
      map.addLayer({
        id: 'jalanaman-route-line', type: 'line', source: 'jalanaman-route',
        paint: { 'line-color': levelColors[routeLevel] || '#1d4ed8', 'line-width': 6, 'line-opacity': 0.94 },
      });
      const destination = route[route.length - 1];
      new maplibregl.Marker({ color: '#1d4ed8' }).setLngLat([destination.lng, destination.lat]).addTo(map);
      const bounds = route.reduce((value, point) => value.extend([point.lng, point.lat]), new maplibregl.LngLatBounds(route[0], route[0]));
      map.fitBounds(bounds, { padding: { top: 64, right: 52, bottom: 92, left: 52 }, maxZoom: 16, pitch: 54, bearing: -24, duration: 0 });
    }

    if (locationPoint) {
      const marker = document.createElement('div');
      marker.className = 'ja-me-3d';
      new maplibregl.Marker({ element: marker, anchor: 'center' }).setLngLat([locationPoint.lng, locationPoint.lat]).addTo(map);
    }

    if (reports.length) {
      map.addSource('jalanaman-reports', {
        type: 'geojson',
        data: { type: 'FeatureCollection', features: reports.map(report => ({ type: 'Feature', properties: { category: report.category }, geometry: { type: 'Point', coordinates: [report.lng, report.lat] } })) },
      });
      map.addLayer({
        id: 'jalanaman-report-points', type: 'circle', source: 'jalanaman-reports',
        paint: {
          'circle-radius': 8,
          'circle-color': ['match', ['get', 'category'], 'lighting', '#f59e0b', 'crime', '#ef4444', 'accident', '#f97316', '#94a3b8'],
          'circle-stroke-color': '#ffffff', 'circle-stroke-width': 2,
        },
      });
    }
  });
  setTimeout(() => {
    if (!styleReady) showFallback('__THREE_D_FAILED_TITLE__', '__THREE_D_FAILED_BODY__');
  }, 10000);
}

function baseCenter() {
  if (locationPoint) return [locationPoint.lng, locationPoint.lat];
  if (firstReport) return [firstReport.lng, firstReport.lat];
  return [106.8456, -6.2088];
}

function project(lng, lat, zoomLevel) {
  const sin = Math.sin(lat * Math.PI / 180);
  const scale = tileSize * (2 ** zoomLevel);
  return {
    x: (lng + 180) / 360 * scale,
    y: (0.5 - Math.log((1 + sin) / (1 - sin)) / (4 * Math.PI)) * scale,
  };
}

function unproject(x, y, zoomLevel) {
  const scale = tileSize * (2 ** zoomLevel);
  const lng = x / scale * 360 - 180;
  const n = Math.PI - (2 * Math.PI * y) / scale;
  const lat = (180 / Math.PI) * Math.atan(0.5 * (Math.exp(n) - Math.exp(-n)));
  return [lng, lat];
}

function chooseViewport(width, height) {
  if (route.length <= 1) {
    return { center: baseCenter(), zoom: locationPoint || firstReport ? 15 : 11 };
  }

  const routePoints = route.concat(locationPoint ? [locationPoint] : []);
  const lngs = routePoints.map(p => p.lng);
  const lats = routePoints.map(p => p.lat);
  const minLng = Math.min(...lngs);
  const maxLng = Math.max(...lngs);
  const minLat = Math.min(...lats);
  const maxLat = Math.max(...lats);
  const center = [(minLng + maxLng) / 2, (minLat + maxLat) / 2];
  const padX = 52;
  const padY = 52;

  for (let z = 16; z >= 9; z -= 1) {
    const a = project(minLng, maxLat, z);
    const b = project(maxLng, minLat, z);
    if (Math.abs(b.x - a.x) <= width - padX && Math.abs(b.y - a.y) <= height - padY) {
      return { center, zoom: z };
    }
  }

  return { center, zoom: 9 };
}

function screenPoint(lng, lat, centerPx, width, height, zoomLevel) {
  const point = project(lng, lat, zoomLevel);
  return {
    x: point.x - centerPx.x + width / 2,
    y: point.y - centerPx.y + height / 2,
  };
}

function addPoint(className, lng, lat, bg, text, centerPx, width, height, zoomLevel) {
  const point = screenPoint(lng, lat, centerPx, width, height, zoomLevel);
  const el = document.createElement('div');
  el.className = className;
  if (bg) el.style.background = bg;
  if (text) {
    if (className === 'route-end') {
      const span = document.createElement('span');
      span.textContent = text;
      el.appendChild(span);
    } else {
      el.textContent = text;
    }
  }
  el.style.left = `${point.x}px`;
  el.style.top = `${point.y}px`;
  pointsEl.appendChild(el);
}

function drawRoute(centerPx, width, height, zoomLevel) {
  overlay.setAttribute('viewBox', `0 0 ${width} ${height}`);
  overlay.setAttribute('width', String(width));
  overlay.setAttribute('height', String(height));
  overlay.innerHTML = '';

  if (route.length > 1) {
    const points = route
      .map(p => screenPoint(p.lng, p.lat, centerPx, width, height, zoomLevel))
      .map(p => `${p.x.toFixed(1)},${p.y.toFixed(1)}`)
      .join(' ');
    const polyline = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');
    polyline.setAttribute('points', points);
    polyline.setAttribute('fill', 'none');
    polyline.setAttribute('stroke', levelColors[routeLevel] || '#1d4ed8');
    polyline.setAttribute('stroke-width', '7');
    polyline.setAttribute('stroke-linecap', 'round');
    polyline.setAttribute('stroke-linejoin', 'round');
    polyline.setAttribute('opacity', '0.92');
    overlay.appendChild(polyline);
  }
}

function renderMap(overrideCenter, overrideZoom) {
  const width = mapEl.clientWidth || 360;
  const height = mapEl.clientHeight || 360;
  let center;
  let zoom;
  if (overrideCenter && Number.isFinite(overrideZoom)) {
    center = overrideCenter;
    zoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, overrideZoom));
  } else {
    const viewport = chooseViewport(width, height);
    center = viewport.center;
    zoom = viewport.zoom;
  }
  activeCenter = center;
  activeZoom = zoom;
  viewportEl.style.transform = 'translate(0px, 0px)';
  const worldTiles = 2 ** zoom;
  const centerPx = project(center[0], center[1], zoom);
  const startX = Math.floor((centerPx.x - width / 2) / tileSize);
  const endX = Math.floor((centerPx.x + width / 2) / tileSize);
  const startY = Math.floor((centerPx.y - height / 2) / tileSize);
  const endY = Math.floor((centerPx.y + height / 2) / tileSize);
  let loaded = 0;
  let failed = 0;
  let total = 0;

  tilesEl.innerHTML = '';
  pointsEl.innerHTML = '';
  drawRoute(centerPx, width, height, zoom);

  for (let x = startX; x <= endX; x++) {
    for (let y = startY; y <= endY; y++) {
      if (y < 0 || y >= worldTiles) continue;
      total += 1;
      const wrappedX = ((x % worldTiles) + worldTiles) % worldTiles;
      const img = document.createElement('img');
      img.alt = '';
      img.decoding = 'async';
      img.referrerPolicy = 'no-referrer';
      img.src = `https://tile.openstreetmap.org/${zoom}/${wrappedX}/${y}.png`;
      img.style.left = `${Math.round(x * tileSize - centerPx.x + width / 2)}px`;
      img.style.top = `${Math.round(y * tileSize - centerPx.y + height / 2)}px`;
      img.onload = () => {
        loaded += 1;
        fallback.style.display = 'none';
      };
      img.onerror = () => {
        failed += 1;
        if (failed >= total && loaded === 0) {
          showFallback('__MAP_FAILED_TITLE__', '__MAP_FAILED_BODY__');
        }
      };
      tilesEl.appendChild(img);
    }
  }

  if (locationPoint) {
    addPoint('me', locationPoint.lng, locationPoint.lat, null, '', centerPx, width, height, zoom);
  }
  if (route.length > 1) {
    const destination = route[route.length - 1];
    addPoint('route-end', destination.lng, destination.lat, null, 'T', centerPx, width, height, zoom);
  }
  reports.forEach(report => {
    addPoint('pin', report.lng, report.lat, colors[report.category] || colors.other, '!', centerPx, width, height, zoom);
  });

  setTimeout(() => {
    if (loaded === 0) showFallback('__MAP_FAILED_TITLE__', '__MAP_FAILED_BODY__');
  }, 6000);
}

function pointFromEvent(evt) {
  if (evt.changedTouches && evt.changedTouches.length) {
    return { x: evt.changedTouches[0].clientX, y: evt.changedTouches[0].clientY };
  }
  if (evt.touches && evt.touches.length) {
    return { x: evt.touches[0].clientX, y: evt.touches[0].clientY };
  }
  return { x: evt.clientX, y: evt.clientY };
}

let dragging = false;
let dragStart = null;
let dragCenterPx = null;
let dragMoved = false;

function onDragStart(evt) {
  if (personDragging || evt.target === reportPerson || reportPerson.contains(evt.target)) return;
  if (evt.touches && evt.touches.length > 1) return;
  dragging = true;
  dragMoved = false;
  dragStart = pointFromEvent(evt);
  dragCenterPx = project(activeCenter[0], activeCenter[1], activeZoom);
  mapEl.classList.add('dragging');
}

function onDragMove(evt) {
  if (personDragging) return;
  if (!dragging) return;
  const p = pointFromEvent(evt);
  const dx = p.x - dragStart.x;
  const dy = p.y - dragStart.y;
  if (Math.abs(dx) > 3 || Math.abs(dy) > 3) dragMoved = true;
  viewportEl.style.transform = `translate(${dx}px, ${dy}px)`;
  if (evt.cancelable) evt.preventDefault();
}

function onDragEnd(evt) {
  if (personDragging) return;
  if (!dragging) return;
  dragging = false;
  mapEl.classList.remove('dragging');
  if (!dragMoved) {
    viewportEl.style.transform = 'translate(0px, 0px)';
    return;
  }
  const p = pointFromEvent(evt);
  const dx = p.x - dragStart.x;
  const dy = p.y - dragStart.y;
  const newCenter = unproject(dragCenterPx.x - dx, dragCenterPx.y - dy, activeZoom);
  renderMap(newCenter, activeZoom);
}

mapEl.addEventListener('mousedown', onDragStart);
window.addEventListener('mousemove', onDragMove);
window.addEventListener('mouseup', onDragEnd);
mapEl.addEventListener('touchstart', onDragStart, { passive: true });
mapEl.addEventListener('touchmove', onDragMove, { passive: false });
mapEl.addEventListener('touchend', onDragEnd);
mapEl.addEventListener('touchcancel', onDragEnd);
mapEl.addEventListener('wheel', (evt) => {
  evt.preventDefault();
  const delta = evt.deltaY < 0 ? 1 : -1;
  renderMap(activeCenter, activeZoom + delta);
}, { passive: false });
mapEl.addEventListener('dblclick', (evt) => {
  evt.preventDefault();
  renderMap(activeCenter, activeZoom + 1);
});

document.getElementById('zoomIn').addEventListener('click', () => renderMap(activeCenter, activeZoom + 1));
document.getElementById('zoomOut').addEventListener('click', () => renderMap(activeCenter, activeZoom - 1));

let personDragging = false;
function reportPointFromScreen(clientX, clientY) {
  const rect = mapEl.getBoundingClientRect();
  const x = Math.max(12, Math.min(rect.width - 12, clientX - rect.left));
  const y = Math.max(12, Math.min(rect.height - 12, clientY - rect.top));
  if (threeDimensional && map3d) {
    const point = map3d.unproject([x, y]);
    return { lat:point.lat, lng:point.lng, x, y };
  }
  const centerPx = project(activeCenter[0], activeCenter[1], activeZoom);
  const selected = unproject(
    centerPx.x + x - rect.width / 2,
    centerPx.y + y - rect.height / 2,
    activeZoom
  );
  return { lat:selected[1], lng:selected[0], x, y };
}
function moveReportPerson(clientX, clientY) {
  const point = reportPointFromScreen(clientX, clientY);
  reportPerson.style.left = `${point.x - 21}px`;
  reportPerson.style.top = `${point.y - 43}px`;
  reportPerson.style.right = 'auto';
  return point;
}
function drawSelectedConnector(point) {
  if (!locationPoint || threeDimensional) {
    selectedLine.innerHTML = '';
    return;
  }
  const rect = mapEl.getBoundingClientRect();
  const centerPx = project(activeCenter[0], activeCenter[1], activeZoom);
  const start = screenPoint(locationPoint.lng, locationPoint.lat, centerPx, rect.width, rect.height, activeZoom);
  selectedLine.setAttribute('viewBox', `0 0 ${rect.width} ${rect.height}`);
  selectedLine.innerHTML = `<line x1='${start.x}' y1='${start.y}' x2='${point.x}' y2='${point.y}' stroke='rgb(245,158,11)' stroke-width='4' stroke-linecap='round' stroke-dasharray='7 7' opacity='.9'/>`;
}
function startPersonDrag(event) {
  if (!selectable) return;
  dragging = false;
  viewportEl.style.transform = 'translate(0px, 0px)';
  mapEl.classList.remove('dragging');
  personDragging = true;
  if (map3d) {
    map3d.dragPan.disable();
    map3d.touchZoomRotate.disable();
    map3d.scrollZoom.disable();
  }
  confirmMark.style.display = 'none';
  markHint.style.display = 'none';
  reportPerson.classList.add('dragging');
  reportPerson.setPointerCapture?.(event.pointerId);
  event.stopPropagation();
  event.preventDefault();
}
function movePersonDrag(event) {
  if (!personDragging) return;
  pendingReportPoint = moveReportPerson(event.clientX, event.clientY);
  dragCoords.textContent = `${pendingReportPoint.lat.toFixed(5)}, ${pendingReportPoint.lng.toFixed(5)}`;
  drawSelectedConnector(pendingReportPoint);
  event.stopPropagation();
  event.preventDefault();
}
function finishPersonDrag(event) {
  if (!personDragging) return;
  personDragging = false;
  pendingReportPoint = moveReportPerson(event.clientX, event.clientY);
  drawSelectedConnector(pendingReportPoint);
  reportPerson.classList.remove('dragging');
  if (map3d) {
    map3d.dragPan.enable();
    map3d.touchZoomRotate.enable();
    map3d.scrollZoom.enable();
  }
  confirmCoords.textContent = `${pendingReportPoint.lat.toFixed(6)}, ${pendingReportPoint.lng.toFixed(6)}`;
  confirmMark.style.display = 'block';
  markHint.style.display = 'none';
  event.stopPropagation();
  event.preventDefault();
}
reportPerson.addEventListener('pointerdown', startPersonDrag);
reportPerson.addEventListener('pointermove', movePersonDrag);
reportPerson.addEventListener('pointerup', finishPersonDrag);
reportPerson.addEventListener('pointercancel', finishPersonDrag);
for (const eventName of ['touchstart', 'touchmove', 'touchend', 'mousedown', 'mousemove', 'mouseup']) {
  reportPerson.addEventListener(eventName, event => {
    event.stopPropagation();
    if (event.cancelable) event.preventDefault();
  }, { passive:false });
}
window.addEventListener('pointermove', movePersonDrag);
window.addEventListener('pointerup', finishPersonDrag);
document.getElementById('cancelmark').addEventListener('click', () => {
  confirmMark.style.display = 'none';
  markHint.style.display = 'block';
});
document.getElementById('acceptmark').addEventListener('click', () => {
  if (!pendingReportPoint) return;
  window.parent.postMessage({
    type:'jalanaman-map-report',
    lat:pendingReportPoint.lat,
    lng:pendingReportPoint.lng
  }, '*');
  confirmMark.style.display = 'none';
});

try {
  if (threeDimensional) {
    renderThreeDimensionalMap();
  } else {
    renderMap();
  }
} catch (_) {
  showFallback(threeDimensional ? '__THREE_D_FAILED_TITLE__' : '__MAP_FAILED_TITLE__', threeDimensional ? '__THREE_D_FAILED_BODY__' : '__MAP_FAILED_BODY__');
}
</script>
</body>
</html>"#
        .replace("__LOCATION__", &location_json)
        .replace("__REPORTS__", &reports_json)
        .replace("__ROUTE__", &route_json)
        .replace("__ROUTE_LEVEL__", &route_level_json)
        .replace("__THREE_DIMENSIONAL__", three_dimensional_json)
        .replace("__SELECTABLE__", selectable_json)
        .replace("__LOADING_TITLE__", loading_title)
        .replace("__LOADING_BODY__", loading_body)
        .replace("__MAP_FAILED_TITLE__", map_failed_title)
        .replace("__MAP_FAILED_BODY__", map_failed_body)
        .replace("__THREE_D_FAILED_TITLE__", three_d_failed_title)
        .replace("__THREE_D_FAILED_BODY__", three_d_failed_body)
        .replace("__MARK_HINT__", mark_hint)
        .replace("__MARK_CONFIRM__", mark_confirm)
        .replace("__MARK_CANCEL__", mark_cancel)
        .replace("__MARK_ACCEPT__", mark_accept)
}
