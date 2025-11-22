USE zoo_db;
GO
CREATE TABLE Animal (
    name VARCHAR(250),
    description TEXT,
    date_of_birth DATE,
    specie VARCHAR(250),
    habitat VARCHAR(250),
    country_of_origin VARCHAR(250),
    is_active BIT,
    animal_id INT PRIMARY KEY
)
CREATE TABLE Cares (
    type_of_care VARCHAR(250),
    description TEXT,
    frequency VARCHAR(250),
    cares_id INT PRIMARY KEY
)
CREATE TABLE Animal_Care_have (
    date_of_care DATE,
    fk_Cares_cares_id INT,
    fk_Animal_animal_id INT,
    animal_care_id INT PRIMARY KEY
)
ALTER TABLE Animal_Care_have ADD CONSTRAINT FK_Animal_Care_have_1
    FOREIGN KEY (fk_Cares_cares_id)
    REFERENCES Cares (cares_id)
 
ALTER TABLE Animal_Care_have ADD CONSTRAINT FK_Animal_Care_have_2
    FOREIGN KEY (fk_Animal_animal_id)
    REFERENCES Animal (animal_id)
GO