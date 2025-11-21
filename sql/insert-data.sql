USE zoo_db;
GO

-- Insert animals
INSERT INTO Animal (name, description, date_of_birth, specie, habitat, country_of_origin, animal_id)
VALUES
('Lion', 'Large carnivorous feline', '2015-06-01', 'Panthera leo', 'Savannah', 'Kenya', 1),
('Elephant', 'Largest land animal', '2010-09-15', 'Loxodonta africana', 'Grassland', 'Botswana', 2);

-- Insert cares
INSERT INTO Cares (type_of_care, description, fresquency, cares_id)
VALUES
('Feeding', 'Daily feeding routine', 'Daily', 1),
('Medical Check', 'Monthly health check', 'Monthly', 2);

-- Insert animal care records
INSERT INTO Animal_Care_have (date_of_care, fk_Cares_cares_id, fk_Animal_animal_id)
VALUES
('2025-11-01', 1, 1),
('2025-11-02', 2, 2);

GO