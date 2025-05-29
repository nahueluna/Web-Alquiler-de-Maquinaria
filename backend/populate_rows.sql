INSERT INTO users (email, name, surname, psw_hash, salt, role) VALUES
('alice@example.com', 'Alice', 'Johnson', 'a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0', 'abc123def456ghi7', 0),
('bob@example.com', 'Bob', 'Smith', 'b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1', 'bcd234efg567hij8', 1),
('carol@example.com', 'Carol', 'White', 'c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2', 'cde345fgh678ijk9', 0),
('dave@example.com', 'Dave', 'Brown', 'd4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3', 'def456ghi789jkl0', 2),
('emma@example.com', 'Emma', 'Davis', 'e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4', 'efg567hij890klm1', 0),
('frank@example.com', 'Frank', 'Miller', 'f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5', 'fgh678ijk901lmn2', 1),
('grace@example.com', 'Grace', 'Wilson', '67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f', 'ghi789jkl012mno3', 0),
('hank@example.com', 'Hank', 'Moore', '7890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f6', 'hij890klm123nop4', 2),
('ivy@example.com', 'Ivy', 'Taylor', '890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f67', 'ijk901lmn234opq5', 2),
('login@example.com', 'Ivy', 'Taylor', '9c819791b519290f435a8fcf896b2125274350636b220106f8629f6eedb3ea7d', 'PC8cXiOnFZO8Radu', 2),
('admin@example.com', 'admin', 'admin', '4e8822dcafcb5611e1554f6054969e25e81228751211a28c94f82dd79f77f5fe', '1234123412341234', 0),
('refresh@example.com', 'refresh', 'refresh', '69aa8cddc392e17d03008aa250cae723ee7402524007124e8190aaec5c7311e0', '1234123412341234', 2),
('pswchange@example.com', 'psw', 'change', 'change', 'psw', 0),
('logout@example.com', 'log', 'out', '0df0d73a572d94bf79da609a4b37a4dd7a308c9debf51197a2e1eaa5b616a8a1', '1111111111111111', 2),
('admin2@example.com', 'admin', '2', '9c82435c63a8d9c0afee889440e9b5c75c0b72cc717230a1dab6f0db35a7e2eb', '1212121212121212', 0);

INSERT INTO user_info (id, birthdate, id_card, phone) VALUES
(2, '1985-05-22', 'ID234567', '555-2345'),
(6, '1982-07-11', 'ID678901', '555-5678'),
(4, '1988-12-03', 'ID456789', NULL),
(8, '1989-02-14', 'ID890123', NULL),
(9, '1993-06-30', 'ID901234', '555-7890'),
(10, '1993-06-30', '888888', '555-7890'),
(12, '1993-06-30', '123123', '555-7890'),
(14, '1993-06-30', '123122', '555-7890');

INSERT INTO change_psw_codes (id, code) VALUES
(13, 'change_psw_code');

-- Insert sample data into the machinery_models table
INSERT INTO machinery_models (name, brand, model, year, policy, description, price) VALUES
('Excavadora hidráulica', 'Caterpillar', 'CAT320D', 2020, 'Cancelación con 48h de anticipación', 'Excavadora para trabajos pesados', 150000.00),
('Retroexcavadora', 'John Deere', '310SL', 2019, 'Sin devolución', 'Ideal para zonas urbanas', 95000.00),
('Cargadora frontal', 'Komatsu', 'WA270', 2021, 'Cancelación sin costo 24h antes', 'Cargadora con gran potencia', 120000.00),
('Miniexcavadora', 'Bobcat', 'E35', 2022, 'Cancelación parcial con retención del 20%', 'Compacta para espacios reducidos', 75000.00),
('Grúa torre', 'Liebherr', 'EC-B', 2023, 'No reembolsable', 'Grúa de gran altura', 200000.00);

-- Insert sample data into the locations table
INSERT INTO locations (latitude, longitude, street, number, city) VALUES
(-34.603722, -58.381592, 'Av. Corrientes', '1234', 'Buenos Aires'),
(-34.920495, -57.953566, 'Calle 50', '678', 'La Plata'),
(-32.944242, -60.650538, 'Av. Pellegrini', '2345', 'Rosario'),
(-31.420083, -64.188776, 'Bv. San Juan', '789', 'Córdoba');

-- Insert sample data into the machinery_units table
INSERT INTO machinery_units (serial_number, status, assigned_at, model_id, location_id) VALUES
-- Modelo 1 (Caterpillar)
('CAT-001', 'available', NOW() - INTERVAL '100 days', 1, 1),
('CAT-002', 'rented', NOW() - INTERVAL '20 days', 1, 2),
('CAT-003', 'maintenance', NOW() - INTERVAL '5 days', 1, 3),

-- Modelo 2 (John Deere)
('JD-001', 'available', NOW() - INTERVAL '50 days', 2, 1),
('JD-002', 'rented', NOW() - INTERVAL '10 days', 2, 4),
('JD-003', 'maintenance', NOW() - INTERVAL '15 days', 2, 2),

-- Modelo 3 (Komatsu)
('KM-001', 'available', NOW() - INTERVAL '60 days', 3, 3),
('KM-002', 'rented', NOW() - INTERVAL '30 days', 3, 4),
('KM-003', 'available', NOW() - INTERVAL '5 days', 3, 1),

-- Modelo 4 (Bobcat)
('BC-001', 'available', NOW() - INTERVAL '2 days', 4, 2),
('BC-002', 'rented', NOW() - INTERVAL '40 days', 4, 3),
('BC-003', 'maintenance', NOW() - INTERVAL '7 days', 4, 4),

-- Modelo 5 (Liebherr)
('LH-001', 'available', NOW() - INTERVAL '3 days', 5, 1),
('LH-002', 'rented', NOW() - INTERVAL '25 days', 5, 2),
('LH-003', 'available', NOW() - INTERVAL '12 days', 5, 3);

-- Insert sample data into the machinery_location_history table
INSERT INTO machinery_location_history (unit_id, location_id, assigned_at, unassigned_at) VALUES
-- CAT-001 antes estaba en locación 2
(1, 2, NOW() - INTERVAL '180 days', NOW() - INTERVAL '100 days'),

-- JD-003 estuvo en Rosario antes
(6, 3, NOW() - INTERVAL '100 days', NOW() - INTERVAL '15 days'),

-- KM-002 estuvo en Buenos Aires antes
(8, 1, NOW() - INTERVAL '90 days', NOW() - INTERVAL '30 days'),

-- BC-003 estaba en La Plata
(12, 2, NOW() - INTERVAL '50 days', NOW() - INTERVAL '7 days');

INSERT INTO categories (name) VALUES
('construccion pesada'),
('obras urbanas'),
('movimiento de tierra'),
('elevacion'),
('compactacion');

-- Modelo 1: Excavadora Caterpillar → Construcción pesada
INSERT INTO machinery_categories (model_id, category_id) VALUES
(1, 1);

-- Modelo 2: Retroexcavadora John Deere → Obras urbanas, Movimiento de tierra, Compactación
INSERT INTO machinery_categories (model_id, category_id) VALUES
(2, 2),
(2, 3),
(2, 5);

-- Modelo 3: Cargadora frontal Komatsu → Movimiento de tierra, Compactación
INSERT INTO machinery_categories (model_id, category_id) VALUES
(3, 3),
(3, 5);

-- Modelo 4: Miniexcavadora Bobcat → Obras urbanas, Movimiento de tierra
INSERT INTO machinery_categories (model_id, category_id) VALUES
(4, 2),
(4, 3);

-- Modelo 5: Grúa torre Liebherr → Construcción pesada, Elevación
INSERT INTO machinery_categories (model_id, category_id) VALUES
(5, 1),
(5, 4);

-- Insert sample data into the rentals table
-- completed | 01/12/2024 - 10/12/2024 | Máquina ID 2 (CAT-002 - Caterpillar) | Usuario ID 2
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (2, 2, NOW() - INTERVAL '179 days', NOW() - INTERVAL '170 days', 5000.00, 'completed');

-- completed | 01/01/2025 - 10/01/2025 | Máquina ID 5 (JD-002 - John Deere) | Usuario ID 6
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (6, 5, NOW() - INTERVAL '148 days', NOW() - INTERVAL '139 days', 4300.00, 'completed');

-- active | 25/05/2025 - 05/06/2025 | Máquina ID 6 (JD-003 - John Deere) | Usuario ID 10
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (10, 6, NOW() - INTERVAL '3 days', NOW() + INTERVAL '8 days', 4100.00, 'active');

-- active | 28/05/2025 - 10/06/2025 | Máquina ID 9 (KM-002 - Komatsu) | Usuario ID 14
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (14, 9, NOW(), NOW() + INTERVAL '13 days', 3950.00, 'active');

-- pending_payment | 15/06/2025 - 25/06/2025 | Máquina ID 3 (CAT-003 - Caterpillar) | Usuario ID 4
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (4, 3, NOW() + INTERVAL '18 days', NOW() + INTERVAL '28 days', 6200.00, 'pending_payment');

-- pending_payment | 20/06/2025 - 27/06/2025 | Máquina ID 7 (KM-003 - Komatsu) | Usuario ID 11
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (11, 7, NOW() + INTERVAL '23 days', NOW() + INTERVAL '30 days', 4000.00, 'pending_payment');

-- pending_payment | 22/06/2025 - 29/06/2025 | Máquina ID 8 (BC-001 - Bobcat) | Usuario ID 12
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (12, 8, NOW() + INTERVAL '15 days', NOW() + INTERVAL '29 days', 4300.00, 'pending_payment');

-- cancelled | 01/06/2025 - 10/06/2025 | Máquina ID 2 (CAT-002 - Caterpillar) | Usuario ID 8
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (8, 2, NOW() + INTERVAL '4 days', NOW() + INTERVAL '13 days', 5500.00, 'cancelled');

-- cancelled | 01/06/2025 - 08/06/2025 | Máquina ID 3 (CAT-003 - Caterpillar) | Usuario ID 9
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (9, 3, NOW() + INTERVAL '4 days', NOW() + INTERVAL '11 days', 5800.00, 'cancelled');

-- failed | 03/06/2025 - 10/06/2025 | Máquina ID 1 (CAT-001 - Caterpillar) | Usuario ID 13
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (13, 1, NOW() + INTERVAL '6 days', NOW() + INTERVAL '13 days', 4600.00, 'failed');


INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (1, 10, NOW() + INTERVAL '8 days', NOW() + INTERVAL '18 days', 6000.00, 'active');

INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
VALUES (2, 10, NOW() + INTERVAL '28 days', NOW() + INTERVAL '38 days', 6000.00, 'pending_payment');