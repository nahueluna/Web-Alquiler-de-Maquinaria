INSERT INTO users (email, name, surname, birthdate, id_card, phone, psw_hash, salt, role) VALUES
('alice@example.com', 'Alice', 'Johnson', '1990-01-15', 'ID123456', '555-1234', 'a1b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0', 'abc123def456ghi7', 0),
('bob@example.com', 'Bob', 'Smith', '1985-05-22', 'ID234567', '555-2345', 'b2c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1', 'bcd234efg567hij8', 1),
('carol@example.com', 'Carol', 'White', '1992-09-10', 'ID345678', '555-3456', 'c3d4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2', 'cde345fgh678ijk9', 0),
('dave@example.com', 'Dave', 'Brown', '1988-12-03', 'ID456789', NULL, 'd4e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3', 'def456ghi789jkl0', 2),
('emma@example.com', 'Emma', 'Davis', '1995-03-25', 'ID567890', '555-4567', 'e5f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4', 'efg567hij890klm1', 0),
('frank@example.com', 'Frank', 'Miller', '1982-07-11', 'ID678901', '555-5678', 'f67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5', 'fgh678ijk901lmn2', 1),
('grace@example.com', 'Grace', 'Wilson', '1991-11-19', 'ID789012', '555-6789', '67890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f', 'ghi789jkl012mno3', 0),
('hank@example.com', 'Hank', 'Moore', '1989-02-14', 'ID890123', NULL, '7890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f6', 'hij890klm123nop4', 2),
('ivy@example.com', 'Ivy', 'Taylor', '1993-06-30', 'ID901234', '555-7890', '890123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f67', 'ijk901lmn234opq5', 0),
('login@example.com', 'Ivy', 'Taylor', '1993-06-30', '888888', '555-7890', '9c819791b519290f435a8fcf896b2125274350636b220106f8629f6eedb3ea7d', 'PC8cXiOnFZO8Radu', 2),
('jack@example.com', 'Jack', 'Anderson', '1987-08-08', 'ID012345', '555-8901', '90123456789abcdef0123456789abcdef0123456789abcdef0a1b2c3d4e5f678', 'jkl012mno345pqr6', 1);

INSERT INTO machines (serial_number, name, brand, model, year, policy, description, price) VALUES
('SN1001', 'Hydraulic Press A1', 'Bosch', 'HP-A1', 2020, 'Standard warranty: 2 years', 'Industrial hydraulic press for metal shaping.', 12500.00),
('SN1002', 'Laser Cutter X5', 'Epilog', 'LC-X5', 2021, 'Extended warranty: 3 years', 'High-precision CO2 laser cutter for wood and acrylic.', 18999.50),
('SN1003', '3D Printer Max', 'Ultimaker', 'S5', 2022, 'Standard warranty: 1 year', 'FDM 3D printer with dual extrusion.', 7499.99),
('SN1004', 'CNC Milling Machine', 'Haas', 'VF-2', 2019, 'Standard warranty: 2 years', 'Vertical machining center for metal parts.', 22000.00),
('SN1005', 'Lathe Pro 3000', 'Jet', 'LP-3000', 2018, 'Standard warranty: 2 years', 'Precision lathe for metal and wood turning.', 9700.00),
('SN1006', 'Welding Station ZX', 'Lincoln Electric', 'WS-ZX', 2023, 'Extended warranty: 4 years', 'Advanced TIG/MIG welding station.', 4200.00),
('SN1007', 'Drill Press HeavyDuty', 'DeWalt', 'DP-HD', 2020, 'Standard warranty: 2 years', 'Floor drill press for industrial drilling.', 3100.00),
('SN1008', 'Plasma Cutter Turbo', 'Hypertherm', 'PC-Turbo', 2021, 'Standard warranty: 2 years', 'Portable plasma cutter for steel plates.', 6500.00),
('SN1009', 'Industrial Robot Arm', 'Fanuc', 'M-20iA', 2022, 'Extended warranty: 5 years', '6-axis robotic arm for assembly and welding.', 36000.00),
('SN1010', 'Compressor XTR', 'Atlas Copco', 'XTR-500', 2017, 'Standard warranty: 2 years', 'High-performance air compressor for workshops.', 8700.00);
