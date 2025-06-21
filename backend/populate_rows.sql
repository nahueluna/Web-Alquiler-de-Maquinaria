INSERT INTO users (email, name, surname, psw_hash, salt, role, status) VALUES
('alice@example.com', 'Alice', 'Johnson', 'a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0', 'abc123def456ghi7', 0, 'active'),
('bob@example.com', 'Bob', 'Smith', 'b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1', 'bcd234efg567hij8', 1, 'active'),
('carol@example.com', 'Carol', 'White', 'c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2', 'cde345fgh678ijk9', 0, 'active'),
('dave@example.com', 'Dave', 'Brown', 'd4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3', 'def456ghi789jkl0', 2, 'active'),
('emma@example.com', 'Emma', 'Davis', 'e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4', 'efg567hij890klm1', 0, 'active'),
('frank@example.com', 'Frank', 'Miller', 'f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5', 'fgh678ijk901lmn2', 1, 'active'),
('grace@example.com', 'Grace', 'Wilson', '67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f', 'ghi789jkl012mno3', 0, 'active'),
('hank@example.com', 'Hank', 'Moore', '7890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f6', 'hij890klm123nop4', 2, 'active'),
('ivy@example.com', 'Ivy', 'Taylor', '890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f67', 'ijk901lmn234opq5', 2, 'active'),
('login@example.com', 'Ivy', 'Taylor', '9c819791b519290f435a8fcf896b2125274350636b220106f8629f6eedb3ea7d', 'PC8cXiOnFZO8Radu', 2, 'active'),
('admin@example.com', 'admin', 'admin', '4e8822dcafcb5611e1554f6054969e25e81228751211a28c94f82dd79f77f5fe', '1234123412341234', 0, 'active'),
('refresh@example.com', 'refresh', 'refresh', '69aa8cddc392e17d03008aa250cae723ee7402524007124e8190aaec5c7311e0', '1234123412341234', 2, 'active'),
('pswchange@example.com', 'psw', 'change', 'change', 'psw', 0, 'active'),
('logout@example.com', 'log', 'out', '0df0d73a572d94bf79da609a4b37a4dd7a308c9debf51197a2e1eaa5b616a8a1', '1111111111111111', 2, 'active'),
('admin2@example.com', 'admin', '2', '9c82435c63a8d9c0afee889440e9b5c75c0b72cc717230a1dab6f0db35a7e2eb', '1212121212121212', 0, 'active'),
('check_change_psw_code@example.com', 'jamie', 'hi', '2c82435c63a8d9c0afee889440e9b5c75c0b72cc717230a1dab6f0db35a7e2eb', '1212121212121212', 0, 'active'),
('client1@example.com', 'user17', 'one', 'nopasswordforyou', '123', 2, 'active'),
('loadreturn@example.com', 'user18', 'one', 'nopasswordforyoueither', '123', 0, 'active'),
('newquestion@example.com', 'user19', 'u19', 'nopasswordforyoueither', '123', 2, 'active'),
('newanswer@example.com', 'user20', 'u20', 'nopasswordforyoueither', '123', 0, 'active'),
('user@example.com', 'user21', 'u21', '4e8822dcafcb5611e1554f6054969e25e81228751211a28c94f82dd79f77f5fe', '1234123412341234', 2, 'active'),
('employee1@example.com', 'user22', 'u22', '4e8822dcafcb5611e1554f6054969e25e81228751211a28c94f82dd79f77f5fe', '1234123412341234', 1, 'active'),
('employee2@example.com', 'user23', 'u23', '4e8822dcafcb5611e1554f6054969e25e81228751211a28c94f82dd79f77f5fe', '1234123412341234', 1, 'active');

INSERT INTO user_info (id, birthdate, id_card, phone) VALUES
(2, '1985-05-22', 'ID234567', '555-2345'),
(6, '1982-07-11', 'ID678901', '555-5678'),
(4, '1988-12-03', 'ID456789', NULL),
(8, '1989-02-14', 'ID890123', NULL),
(9, '1993-06-30', 'ID901234', '555-7890'),
(10, '1993-06-30', '888888', '555-7890'),
(12, '1993-06-30', '123123', '555-7890'),
(14, '1993-06-30', '123122', '555-7890'),
(19, '1993-06-30', '123134', '555-7890'),
(21, '1993-06-30', '123534', '555-7890'),
(22, '1993-06-30', '12123534', '555-7890'),
(23, '1993-06-30', '1asdad23534', '555-7890');

INSERT INTO change_psw_codes (id, code) VALUES
(13, 'change_psw_code'),
(16, 'change_psw_code2');

-- Insert sample data into the machinery_models table
INSERT INTO machinery_models (name, brand, model, year, policy, description, price, image) VALUES
('Excavadora hidráulica', 'Caterpillar', 'CAT320D', 2020, 'No se realizan reembolsos por cancelaciones.', 'Excavadora para trabajos pesados', 150000.00, 'imagecode'),
('Retroexcavadora', 'John Deere', '310SL', 2019, 'No se realizan reembolsos por cancelaciones.', 'Ideal para zonas urbanas', 95000.00, 'imagecode'),
('Cargadora frontal', 'Komatsu', 'WA270', 2021, 'Se aplica un reembolso parcial según el tiempo de aviso previo.', 'Cargadora con gran potencia', 120000.00, 'imagecode'),
('Miniexcavadora', 'Bobcat', 'E35', 2022, 'Reembolso total disponible si se cancela con suficiente antelación.', 'Compacta para espacios reducidos', 75000.00, 'imagecode'),
('Grúa torre', 'Liebherr', 'EC-B', 2023, 'Reembolso total disponible si se cancela con suficiente antelación.', 'Grúa de gran altura', 200000.00, 'imagecode'),
('testloadreturn', 'model6', 'EC-B', 2023, 'Reembolso total disponible si se cancela con suficiente antelación.', 'Grúa de gran altura', 200000.00, 'imagecode');

-- Insert sample data into the extra images table
INSERT INTO model_extra_images (name, id) VALUES 
('imagecode1', 1),
('imagecode2', 1),
('imagecode3', 2),
('imagecode4', 4),
('imagecode5', 6);

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
('CAT-002', 'available', NOW() - INTERVAL '20 days', 1, 2),
('CAT-003', 'maintenance', NOW() - INTERVAL '5 days', 1, 1),

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
('LH-003', 'available', NOW() - INTERVAL '12 days', 5, 3),

-- Modelo 6 (test load return)
('CAT-0016', 'rented', NOW() - INTERVAL '5 days', 6, 1),
('CAT-0017', 'rented', NOW() - INTERVAL '5 days', 6, 1),
('CAT-0018', 'available', NOW() - INTERVAL '5 days', 6, 1);

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
INSERT INTO rentals (
    user_id, machine_id, start_date, end_date, total_price, status,
    retirement_employee_id, return_employee_id, retirement_date, return_date, payment_id, created_at
)
VALUES (
    4, 2, NOW() - INTERVAL '179 days', NOW() - INTERVAL '170 days', 1000.00, 'completed',
    2, 6, NOW() - INTERVAL '179 days', NOW() - INTERVAL '170 days', 'PAY_00001', DATE_TRUNC('year', NOW())
),
(
    8, 5, '2025-1-3', '2025-1-12', 1000.00, 'completed',
    6, 2, '2025-1-3', '2025-1-12', 'PAY_00002', DATE_TRUNC('year', NOW())
),
(
    17, 16,'2025-1-3', '2025-1-12', 1000.00, 'completed',
    2, 6, '2025-1-3', '2025-1-12', 'PAY_00001', DATE_TRUNC('year', NOW())
),
(
    17, 17, NOW() - INTERVAL '179 days', NOW() - INTERVAL '170 days', 1000.00, 'completed',
    2, 6, NOW() - INTERVAL '179 days', NOW() - INTERVAL '170 days', 'PAY_00001', DATE_TRUNC('year', NOW())
);

INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status,
    retirement_employee_id, retirement_date, payment_id, created_at) VALUES
(4, 6, NOW() - INTERVAL '3 days', NOW() + INTERVAL '8 days', 1000.00, 'active', 2, NOW() - INTERVAL '3 days', 'PAY_00003', DATE_TRUNC('year', NOW()));

INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status, created_at) VALUES
(4, 9, NOW(), NOW() + INTERVAL '13 days', 1000.00, 'active', DATE_TRUNC('year', NOW())),

(9, 3, NOW() + INTERVAL '18 days', NOW() + INTERVAL '28 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),

(9, 7, NOW() + INTERVAL '23 days', NOW() + INTERVAL '30 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),

(10, 8, NOW() + INTERVAL '15 days', NOW() + INTERVAL '29 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),

(10, 2, NOW() + INTERVAL '4 days', NOW() + INTERVAL '13 days', 1000.00, 'cancelled', DATE_TRUNC('year', NOW())),

(10, 3, NOW() + INTERVAL '4 days', NOW() + INTERVAL '11 days', 1000.00, 'active', DATE_TRUNC('year', NOW())),

(10, 1, NOW() + INTERVAL '6 days', NOW() + INTERVAL '13 days', 1000.00, 'failed', DATE_TRUNC('year', NOW())),

(4, 10, NOW() + INTERVAL '8 days', NOW() + INTERVAL '18 days', 1000.00, 'active', DATE_TRUNC('year', NOW())),

(9, 10, NOW() + INTERVAL '28 days', NOW() + INTERVAL '38 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),

-- To test cancel rental
(9, 15, NOW() + INTERVAL '23 days', NOW() + INTERVAL '30 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),
(9, 15, NOW() + INTERVAL '40 days', NOW() + INTERVAL '47 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW())),


-- To test get staff rentals - late rent
(9, 15, NOW() - INTERVAL '10 days', NOW() - INTERVAL '3 days', 1000.00, 'active', DATE_TRUNC('year', NOW())),

(4, 2, NOW() - INTERVAL '10 days', NOW() + INTERVAL '3 days', 1000.00, 'active', DATE_TRUNC('year', NOW())),

(4, 2, NOW() - INTERVAL '10 days', NOW() + INTERVAL '3 days', 1000.00, 'completed', DATE_TRUNC('year', NOW())),

(4, 2, NOW() - INTERVAL '10 days', NOW(), 1000.00, 'active', DATE_TRUNC('year', NOW())),

-- To test cancel rental start date in the past
(9, 15, NOW() - INTERVAL '40 days', NOW() + INTERVAL '2 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW()));

INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status,
    retirement_employee_id, retirement_date, payment_id, created_at) VALUES
(4, 6, NOW() - INTERVAL '3 days', NOW() + INTERVAL '8 days', 1000.00, 'active', 2, NOW() - INTERVAL '3 days', 'PAY_00003', DATE_TRUNC('year', NOW()));

-- To test validate dates
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status, created_at) VALUES
(10, 18, NOW() + INTERVAL '5 days', NOW() + INTERVAL '12 days', 1000.00, 'pending_payment', DATE_TRUNC('year', NOW()));


INSERT INTO questions (user_id, model_id, content, created_at) VALUES
(19, 1, 'pregunta 1', NOW()),
(19, 1, 'pregunta 2', NOW()),
(19, 1, 'pregunta 3', NOW()),
(19, 1, 'pregunta 4', NOW() + INTERVAL '1 seconds'),
(19, 1, 'pregunta 5', NOW() + INTERVAL '1 seconds');

INSERT INTO answers (question_id, user_id, content) VALUES
(2, 20, 'r1 a p2'),
(4, 20, 'r2 a p4');

INSERT INTO question_votes (question_id, user_id) VALUES
(5, 1),
(5, 2),
(3, 3),
(3, 4),
(1, 21);

-- Id unit 4 must not have any history events 
INSERT INTO unit_history_events (unit_id, description, previous_status, new_status) VALUES
(1, 'Avería en el sector delantero', 'available', 'maintenance'),
(1, 'Refacciones completadas', 'maintenance', 'available'),
(3, 'Cambio de estado a disponible', 'maintenance', 'available'),
(6, 'Cambio de aceite', 'maintenance', 'available'),
(6, NULL, 'available', 'maintenance');

-- Stats by month test
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, created_at, status) VALUES
-- FEB Current year
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,2,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,2,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,2,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,2,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,2,1,0,0,0),'active'),
-- DEC Current year
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,12,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,12,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,12,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,12,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,12,1,0,0,0),'active'),
-- MAR 2024
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,3,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,3,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,3,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,3,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,3,1,0,0,0),'active'),
-- APR 2024
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,4,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,4,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,4,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,4,1,0,0,0),'active'),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,4,1,0,0,0),'active');

-- Stats by employee test
INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, created_at, status, rental_employee_id) VALUES
-- AUG Current year user22
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',22),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',22),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',22),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',22),
-- AUG 2024 user22
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,8,1,0,0,0),'active',22),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,8,1,0,0,0),'active',22),
-- AUG Current year user23
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',23),
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(EXTRACT(YEAR FROM NOW())::INT,8,1,0,0,0),'active',23),
-- AUG 2024 user23
(10,18,NOW(),NOW(),1000.00,MAKE_TIMESTAMP(2024,8,1,0,0,0),'active',23);
