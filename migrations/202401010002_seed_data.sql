INSERT INTO files (id, filename, size_bytes, uploaded_at, processed)
VALUES
    (gen_random_uuid(), 'example.txt', 1024, NOW() - INTERVAL '1 day', FALSE);