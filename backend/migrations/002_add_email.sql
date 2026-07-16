-- Add email column to emergency_contacts for SOS email alerts
ALTER TABLE emergency_contacts
    ADD COLUMN IF NOT EXISTS email TEXT;

-- Partial index to quickly find contacts that have email configured
CREATE INDEX IF NOT EXISTS idx_ec_email
    ON emergency_contacts (device_hash)
    WHERE email IS NOT NULL;
