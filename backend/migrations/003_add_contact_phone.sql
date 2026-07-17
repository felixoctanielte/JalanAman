-- Add WhatsApp/phone number support for SOS contacts
ALTER TABLE emergency_contacts
    ADD COLUMN IF NOT EXISTS phone TEXT;

CREATE INDEX IF NOT EXISTS idx_ec_phone
    ON emergency_contacts (device_hash)
    WHERE phone IS NOT NULL;
