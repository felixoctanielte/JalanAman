# JalanAman

**Garuda Hacks 7.0 — Track: Safety & Resilience**

> "Google Maps, but it knows which route is safe."

A mobile PWA + web dashboard built on community-sourced safety reports, helping pedestrians and public-transit users in Indonesia know how safe a given route is — with a one-tap SOS button that sounds a loud local alarm and notifies emergency contacts by WhatsApp and email.

No login required. Identity is a local `device_hash`, generated and stored on-device — there is no account system and no profile screen.

---

## MVP Features

| Feature | Description |
|---|---|
| 🗺 **Safety map** | Community-submitted pins (poor lighting / crime-prone / accident-prone / other) shown within a radius around the user |
| ⚠️ **Quick report** | One tap → pick a category → submit with automatic geolocation, no account needed |
| 📊 **Route score** | Enter a destination → overlays an **Aman (safe) / Waspada (caution) / Hindari (avoid)** score on top of Google Directions results; picks the best-scoring route among the alternatives Directions already returns |
| 🆘 **SOS button** | Tap → 3-second cancellable countdown → loud local alarm + vibration (instant, works offline) + automatic email to emergency contacts + one-tap pre-filled WhatsApp message |
| 🏘 **Web dashboard** | Read-only heatmap of reports per area, for RT/RW (neighborhood watch), campus security, or local police |

### Known limitations (disclosed by design)

- **Local alarm is the only truly real-time, guaranteed channel** — it doesn't depend on network access or on anyone else's device/settings.
- **Email** to emergency contacts is sent automatically from the backend (no tap required), but whether the recipient notices it immediately depends on their phone's mail-app notification settings. It is not a guaranteed real-time channel.
- **WhatsApp** messages are pre-filled with the location link, but WhatsApp does not allow fully silent, unattended sending — the sender still has to tap "Send" once per contact. This is a WhatsApp platform restriction, not a bug.
- We do **not** attempt to bypass a recipient's silent/Do-Not-Disturb mode. That kind of OS-level "critical alert" requires a special entitlement from Apple (restricted, approval-only) and is not something a third-party app can obtain for a hackathon project. Framing this honestly is intentional.
- Emergency contacts do **not** need to install anything or grant any permission ahead of time — they only need a valid WhatsApp number or email address.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust (Axum 0.7 + Tokio) |
| Frontend | Dioxus 0.5 (WASM PWA) + Trunk |
| Database | PostgreSQL 16 + `earthdistance` extension |
| Cache | Redis 7 (optional — used for report cooldown / SOS rate-limit TTLs) |
| Maps | Google Maps JavaScript API (map + heatmap) + Directions API (route + polyline) |
| SOS – Email | Resend / SendGrid (free-tier REST API), called from the backend |
| SOS – WhatsApp | `wa.me` deep link, built client-side — no library or paid API needed |
| Container | Docker & Docker Compose |
| Reverse proxy | nginx |

**Scope note:** Dioxus targets **Web (WASM) as an installable PWA**, not native iOS/Android builds — this avoids the immature native mobile toolchain in Dioxus and keeps the 30-hour build loop fast. "Add to Home Screen" covers the "feels like a mobile app" requirement without that risk. REST is prioritized over gRPC/tonic for the same reason: keep the critical path simple.

---

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Tools
cargo install trunk sqlx-cli cargo-watch

# Docker & Docker Compose (already installed)
docker --version
```

---

## Setup & Running

### 1. Clone & environment

```bash
git clone https://github.com/YOUR_USERNAME/JalanAman
cd JalanAman
cp .env.example .env
# Edit .env: fill in GOOGLE_MAPS_API_KEY and EMAIL_API_KEY (Resend or SendGrid)
```

`.env.example`:
```
DATABASE_URL=postgres://jalanaman:jalanaman@localhost:5432/jalanaman
REDIS_URL=redis://localhost:6379
GOOGLE_MAPS_API_KEY=
EMAIL_API_KEY=
EMAIL_FROM=alerts@jalanaman.app
```

### 2. Start infrastructure (DB + Redis)

```bash
docker compose up postgres redis -d
# Wait until healthy, then:
cd backend && sqlx migrate run
```

### 3. Dev mode

```bash
# Terminal 1 – backend
cd backend && cargo watch -x run

# Terminal 2 – frontend
cd frontend && trunk serve
# Open http://localhost:3000
```

### 4. Full Docker stack

```bash
docker compose up -d --build
# Open http://localhost
```

### 5. Seed demo data

```bash
make seed
```

---

## REST API

```
GET  /health
GET  /api/config                          → returns Maps API key for the frontend

POST /api/reports                         → create a new report
GET  /api/reports?lat=&lng=&radius=       → reports within radius (meters)
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

POST /api/route-score                     → compute route safety score
     body: { waypoints: [{lat, lng}] }
     returns the best-scoring route plus up to 2 alternatives, each with a score

POST /api/sos/trigger                     → trigger SOS
     body: { device_hash, lat, lng, timestamp }
     returns: { sos_id, maps_link, email_contacts_notified: [...], whatsapp_contacts: [...] }
POST /api/sos/resolve                     → mark an SOS as resolved ("I'm safe")
     body: { sos_id }

POST /api/contacts                        → add an emergency contact
     body: { device_hash, name, channel: "whatsapp" | "email", value }
GET  /api/contacts?device_hash=           → list emergency contacts for a device
DELETE /api/contacts/:id                  → remove an emergency contact
```

There is intentionally **no** invite/subscribe flow — earlier drafts of this project used Web Push (VAPID) subscriptions, which required the emergency contact to open the PWA and grant notification permission ahead of time. That was dropped in favor of the simpler email/WhatsApp model above, since it needs zero setup on the contact's side.

---

## Anti-Spam / Anti-Abuse Design

- **Report cooldown**: 1 report per device per 10 minutes at the same location (Redis TTL)
- **Auto-hide**: reports with ≥3 downvotes are automatically hidden
- **Report decay**: report weight decreases linearly over 30 days
- **SOS rate limit**: max 1 trigger per minute per device (Redis TTL, plus client-side debounce)
- **Contact ownership**: emergency contacts can only be added through the app by the owning `device_hash` — there is no public endpoint that lets anyone register a WhatsApp number or email as someone else's contact
- **Privacy**: no real names are stored beyond what the user types for a contact's display name; no login means no centrally stored personal profile; SOS location is only ever sent to contacts the user explicitly added themselves

---

## Design System (summary)

Two color modes, dark as default (primary use case is walking at night), light as an optional toggle. Full token table and screen-by-screen mockups live in the project design doc — key points for implementation:

- One color = one meaning everywhere: red = danger/SOS/avoid, amber = caution, green = safe. Never reused for anything else.
- Bottom navigation has **3 tabs only**: Map, Route, Contacts. No Profile tab — there's no login to have a profile of.
- The SOS button is **not** a tab. It's a persistent floating button, visible on every screen, bottom-center, so it's always in the same place during an emergency.
- Category and score labels use plain everyday words ("Gelap", "Rawan", "Kecelakaan" / "Aman", "Waspada", "Hindari") — no percentages or jargon in the UI.

---

## Project Structure

```
JalanAman/
├── backend/                  # Rust/Axum REST API
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── error.rs
│   │   ├── routes.rs
│   │   ├── models/            # report.rs, emergency_contact.rs
│   │   ├── services/          # email.rs (Resend/SendGrid client), whatsapp.rs (wa.me link builder)
│   │   └── handlers/          # reports.rs, route_score.rs, sos.rs, contacts.rs, health.rs
│   └── migrations/
│       └── 001_init.sql
├── frontend/                  # Dioxus WASM PWA
│   ├── src/
│   │   ├── main.rs
│   │   ├── api.rs
│   │   ├── js.rs               # wasm_bindgen bindings to JS helpers
│   │   ├── types.rs
│   │   ├── components/         # map.rs, report_form.rs, sos_button.rs, sos_countdown.rs, route_score.rs
│   │   └── pages/              # map_page.rs, route_page.rs, contacts_page.rs, dashboard.rs
│   ├── public/
│   │   ├── sw.js                # Service Worker (offline caching / PWA installability only — no push)
│   │   └── manifest.json
│   └── index.html               # Entry point + Google Maps script + JS helpers (alarm, vibration)
├── nginx/nginx.conf
├── docker-compose.yml
├── Makefile
└── scripts/
    └── seed_demo.sql
```

---

## Database Schema (summary)

```sql
CREATE TYPE report_category AS ENUM ('lighting', 'crime', 'accident', 'other');
CREATE TYPE contact_channel AS ENUM ('whatsapp', 'email');

CREATE TABLE reports (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  category report_category NOT NULL,
  lat DOUBLE PRECISION NOT NULL,
  lng DOUBLE PRECISION NOT NULL,
  note VARCHAR(100),
  device_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  upvote_count INT NOT NULL DEFAULT 0,
  downvote_count INT NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active'
);

CREATE TABLE emergency_contacts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  device_hash TEXT NOT NULL,
  name TEXT NOT NULL,
  channel contact_channel NOT NULL,
  value TEXT NOT NULL, -- WhatsApp number (62xxx format) or email address
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Just two tables by design — no `users`/`roles`/`push_subscriptions` tables. Radius queries (map view, route scoring) use the `earthdistance`+`cube` extension rather than full PostGIS, since simple radius lookups are all this MVP needs.

---

## AI Usage Disclosure (Garuda Hacks)

As required by Garuda Hacks 7.0 rules, here is what AI (Claude) assisted with:

- **Initial project planning**: theme/track selection, problem scoping, feature prioritization for a 30-hour build window
- **Infrastructure scaffolding** (project structure, `Cargo.toml`, `docker-compose.yml`, initial migration SQL)
- **Axum handler boilerplate** (CRUD structure and error handling patterns)
- **Dioxus component boilerplate** (RSX structure, prop types)
- **Design system and screen mockups** (color tokens, navigation layout, SOS flow)
- **Service Worker and JS helper scaffolding** (Web Audio API, Vibration API, Geolocation)

All core business logic — the route scoring algorithm, the anti-spam/anti-abuse mechanisms, and the SOS trigger flow — was designed by the team and implemented/verified by hand.

---

## Team

1. Farrel Nayaka
2. Felix Octaniel T
3. Aryabell Boston T
4. Ingatius Rayden I.R

---

## License

MIT License – see [LICENSE](LICENSE)
