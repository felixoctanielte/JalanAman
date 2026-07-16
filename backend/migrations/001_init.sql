-- Enable geo distance functions
CREATE EXTENSION IF NOT EXISTS cube;
CREATE EXTENSION IF NOT EXISTS earthdistance;

-- ── Report category enum ──────────────────────────────────────────────────────
DO $$ BEGIN
    CREATE TYPE report_category AS ENUM ('lighting', 'crime', 'accident', 'other');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- ── Reports ───────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reports (
    id            UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    category      report_category NOT NULL,
    lat           DOUBLE PRECISION NOT NULL,
    lng           DOUBLE PRECISION NOT NULL,
    note          VARCHAR(100),
    device_hash   TEXT          NOT NULL,
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    upvote_count  INT           NOT NULL DEFAULT 0,
    downvote_count INT          NOT NULL DEFAULT 0,
    status        TEXT          NOT NULL DEFAULT 'active' -- active | hidden
);

-- Spatial index for radius queries using earthdistance
CREATE INDEX IF NOT EXISTS idx_reports_ll ON reports
    USING GIST (ll_to_earth(lat, lng));

CREATE INDEX IF NOT EXISTS idx_reports_created_at ON reports (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_device_hash ON reports (device_hash);

-- ── Emergency contacts ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS emergency_contacts (
    id                   UUID  PRIMARY KEY DEFAULT gen_random_uuid(),
    device_hash          TEXT  NOT NULL,           -- owner's device hash
    name                 TEXT  NOT NULL,
    contact_device_hash  TEXT,                      -- filled once contact opens invite
    push_endpoint        TEXT,                      -- Web Push subscription endpoint
    push_p256dh          TEXT,                      -- ECDH public key (base64url)
    push_auth            TEXT,                      -- Auth secret (base64url)
    invite_token         TEXT  UNIQUE,              -- UUID token inside the invite URL
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ec_device_hash  ON emergency_contacts (device_hash);
CREATE INDEX IF NOT EXISTS idx_ec_invite_token ON emergency_contacts (invite_token);
