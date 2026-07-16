-- Seed demo data around central Jakarta (Sudirman/Thamrin area)
-- Run: make seed

INSERT INTO reports (category, lat, lng, note, device_hash, upvote_count)
VALUES
  ('crime',    -6.2143, 106.8440, 'Rawan copet di dekat halte TransJakarta malam hari',  'demo-device-1', 5),
  ('crime',    -6.2200, 106.8280, 'Begal motor sering terjadi jam 23.00-01.00',          'demo-device-2', 8),
  ('lighting', -6.2050, 106.8520, 'Lampu jalan mati sudah 2 minggu',                    'demo-device-3', 3),
  ('lighting', -6.2300, 106.8350, 'Gang gelap, hindari malam hari',                     'demo-device-1', 2),
  ('accident', -6.1900, 106.8230, 'Tikungan tanpa rambu, sering kecelakaan motor',      'demo-device-4', 6),
  ('accident', -6.2400, 106.8460, 'Lubang besar di tengah jalan, berbahaya malam',      'demo-device-2', 4),
  ('crime',    -6.2600, 106.8100, 'Area sepi, sering ada modus pura-pura butuh bantuan','demo-device-5', 7),
  ('lighting', -6.1750, 106.8650, 'Jembatan penyeberangan tidak ada penerangan',        'demo-device-3', 1),
  ('crime',    -6.2450, 106.8580, 'Rawan jambret di traffic light merah malam hari',    'demo-device-4', 9),
  ('other',    -6.2100, 106.8700, 'Jalur banjir kalau hujan deras, licin dan berbahaya','demo-device-1', 2)
ON CONFLICT DO NOTHING;
