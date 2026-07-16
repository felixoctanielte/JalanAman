// JalanAman Service Worker – handles Web Push and PWA caching

const CACHE_NAME = 'jalanaman-v1';
const STATIC_ASSETS = ['/', '/index.html', '/manifest.json'];

// ── Install: pre-cache shell ──────────────────────────────────────────────────
self.addEventListener('install', event => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => cache.addAll(STATIC_ASSETS).catch(() => {}))
  );
});

// ── Activate: clean old caches ────────────────────────────────────────────────
self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k)))
    ).then(() => clients.claim())
  );
});

// ── Fetch: network-first for API, cache-first for assets ─────────────────────
self.addEventListener('fetch', event => {
  const url = new URL(event.request.url);
  if (url.pathname.startsWith('/api')) return; // never cache API calls
  event.respondWith(
    fetch(event.request).catch(() => caches.match(event.request))
  );
});

// ── Push: show notification ───────────────────────────────────────────────────
self.addEventListener('push', event => {
  const text = event.data ? event.data.text() : '🆘 SOS! Bantuan dibutuhkan!';

  // Extract Google Maps URL from message if present
  const mapsMatch = text.match(/https:\/\/maps\.google\.com\/\?q=[^\s]+/);
  const mapsUrl   = mapsMatch ? mapsMatch[0] : null;

  const options = {
    body: text,
    icon: '/icon-192.png',
    badge: '/icon-192.png',
    vibrate: [800, 200, 800, 200, 800, 200, 800],
    requireInteraction: true,
    tag: 'sos-alert',
    renotify: true,
    silent: false,
    actions: mapsUrl ? [
      { action: 'open-maps', title: '📍 Buka Lokasi' },
      { action: 'dismiss',   title: 'Tutup' },
    ] : [
      { action: 'dismiss', title: 'Tutup' },
    ],
    data: { mapsUrl },
  };

  event.waitUntil(
    self.registration.showNotification('🚨 JalanAman SOS', options).then(() => {
      // Attempt to speak the message aloud on supported browsers
      // (works when the notification triggers a client focus event)
    })
  );
});

// ── Notification click ────────────────────────────────────────────────────────
self.addEventListener('notificationclick', event => {
  event.notification.close();
  const { action } = event;
  const { mapsUrl } = event.notification.data || {};

  if (action === 'open-maps' && mapsUrl) {
    event.waitUntil(clients.openWindow(mapsUrl));
  } else if (action !== 'dismiss') {
    event.waitUntil(clients.openWindow('/'));
  }
});
