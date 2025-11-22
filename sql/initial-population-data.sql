-- USE zoo_db;
-- GO

-- -- Insert animals
-- INSERT INTO Animal (name, description, date_of_birth, specie, habitat, country_of_origin, animal_id)
-- VALUES
-- ('Lion', 'Large carnivorous feline', '2015-06-01', 'Panthera leo', 'Savannah', 'Kenya', 1),
-- ('Elephant', 'Largest land animal', '2010-09-15', 'Loxodonta africana', 'Grassland', 'Botswana', 2);

-- -- Insert cares
-- INSERT INTO Cares (type_of_care, description, frequency, cares_id)
-- VALUES
-- ('Feeding', 'Daily feeding routine', 'Daily', 1),
-- ('Medical Check', 'Monthly health check', 'Monthly', 2);

-- -- Insert animal care records
-- INSERT INTO Animal_Care_have (date_of_care, fk_Cares_cares_id, fk_Animal_animal_id, animal_care_id)
-- VALUES
-- ('2025-11-01', 1, 1, 1),
-- ('2025-11-02', 2, 2, 2);

-- INSERT INTO Animal (animal_id, name, description, date_of_birth, specie, habitat, country_of_origin)
-- VALUES (3, 'Giraffe', 'Tall herbivorous mammal', '2012-04-10', 'Giraffa camelopardalis', 'Savannah', 'South Africa');

-- GO

USE zoo_db;
GO

INSERT INTO Animal VALUES ('Pedro','Animal selvagem','2018-03-12','Leao','Savana','Africa',1);
INSERT INTO Animal VALUES ('Luna','Animal selvagem','2017-11-05','Pantera','Floresta','Brasil',2);
INSERT INTO Animal VALUES ('Rex','Animal selvagem','2020-01-22','Tigre','Floresta','India',3);
INSERT INTO Animal VALUES ('Milo','Animal selvagem','2019-07-19','Hiena','Savana','Africa',4);
INSERT INTO Animal VALUES ('Nina','Animal selvagem','2016-02-15','Elefante','Savana','Africa',5);
INSERT INTO Animal VALUES ('Bento','Animal selvagem','2018-12-01','Guepardo','Savana','Africa',6);
INSERT INTO Animal VALUES ('Zara','Animal selvagem','2021-05-10','Lobo','Montanhas','Canada',7);
INSERT INTO Animal VALUES ('Thor','Animal selvagem','2015-09-27','Urso','Floresta','Russia',8);
INSERT INTO Animal VALUES ('Kiara','Animal selvagem','2017-04-18','Leopardo','Floresta','India',9);
INSERT INTO Animal VALUES ('Simba','Animal selvagem','2020-06-23','Leao','Savana','Africa',10);
INSERT INTO Animal VALUES ('Akira','Animal selvagem','2019-12-09','Tigre','Floresta','China',11);
INSERT INTO Animal VALUES ('Bolt','Animal selvagem','2018-08-30','Guepardo','Savana','Africa',12);
INSERT INTO Animal VALUES ('Yumi','Animal selvagem','2014-05-14','Pantera','Floresta','Brasil',13);
INSERT INTO Animal VALUES ('Orion','Animal selvagem','2013-10-03','Lobo','Montanhas','EUA',14);
INSERT INTO Animal VALUES ('Atlas','Animal selvagem','2021-01-19','Urso','Floresta','Canada',15);
INSERT INTO Animal VALUES ('Loki','Animal selvagem','2018-07-25','Leopardo','Floresta','India',16);
INSERT INTO Animal VALUES ('Maya','Animal selvagem','2016-03-12','Elefante','Savana','Africa',17);
INSERT INTO Animal VALUES ('Togo','Animal selvagem','2019-09-09','Hiena','Savana','Africa',18);
INSERT INTO Animal VALUES ('Ruby','Animal selvagem','2022-02-28','Tartaruga','Praia','Brasil',19);
INSERT INTO Animal VALUES ('Max','Animal selvagem','2017-05-17','Leao','Savana','Africa',20);
INSERT INTO Animal VALUES ('Nala','Animal selvagem','2018-01-15','Pantera','Floresta','Brasil',21);
INSERT INTO Animal VALUES ('Zeus','Animal selvagem','2019-06-10','Urso','Floresta','Canada',22);
INSERT INTO Animal VALUES ('Koda','Animal selvagem','2017-02-11','Lobo','Montanhas','EUA',23);
INSERT INTO Animal VALUES ('Rafa','Animal selvagem','2020-11-12','Elefante','Savana','Africa',24);
INSERT INTO Animal VALUES ('Leo','Animal selvagem','2016-04-04','Leao','Savana','Africa',25);
INSERT INTO Animal VALUES ('Argo','Animal selvagem','2015-09-14','Hiena','Savana','Africa',26);
INSERT INTO Animal VALUES ('Suri','Animal selvagem','2018-03-27','Guepardo','Savana','Africa',27);
INSERT INTO Animal VALUES ('Tina','Animal selvagem','2019-07-08','Pantera','Floresta','Brasil',28);
INSERT INTO Animal VALUES ('Jade','Animal selvagem','2021-02-17','Leopardo','Floresta','India',29);
INSERT INTO Animal VALUES ('Ciro','Animal selvagem','2016-12-25','Tigre','Floresta','China',30);
INSERT INTO Animal VALUES ('Eros','Animal selvagem','2017-11-30','Leao','Savana','Africa',31);
INSERT INTO Animal VALUES ('Mika','Animal selvagem','2020-09-21','Pantera','Floresta','Brasil',32);
INSERT INTO Animal VALUES ('Ugo','Animal selvagem','2018-06-19','Lobo','Montanhas','Canada',33);
INSERT INTO Animal VALUES ('Bruno','Animal selvagem','2015-03-28','Urso','Floresta','Russia',34);
INSERT INTO Animal VALUES ('Lia','Animal selvagem','2017-08-11','Elefante','Savana','Africa',35);
INSERT INTO Animal VALUES ('Yara','Animal selvagem','2018-10-15','Tartaruga','Praia','Brasil',36);
INSERT INTO Animal VALUES ('Dante','Animal selvagem','2020-04-18','Guepardo','Savana','Africa',37);
INSERT INTO Animal VALUES ('Rico','Animal selvagem','2019-12-07','Hiena','Savana','Africa',38);
INSERT INTO Animal VALUES ('Tara','Animal selvagem','2022-03-13','Pantera','Floresta','Brasil',39);
INSERT INTO Animal VALUES ('Gus','Animal selvagem','2018-09-10','Leao','Savana','Africa',40);
INSERT INTO Animal VALUES ('Lola','Animal selvagem','2016-05-11','Pantera','Floresta','Brasil',41);
INSERT INTO Animal VALUES ('Ragnar','Animal selvagem','2019-01-29','Lobo','Montanhas','EUA',42);
INSERT INTO Animal VALUES ('Otto','Animal selvagem','2014-11-22','Urso','Floresta','Russia',43);
INSERT INTO Animal VALUES ('Salem','Animal selvagem','2017-09-03','Pantera','Floresta','Brasil',44);
INSERT INTO Animal VALUES ('Kai','Animal selvagem','2020-06-11','Tigre','Floresta','India',45);
INSERT INTO Animal VALUES ('Noah','Animal selvagem','2018-08-14','Elefante','Savana','Africa',46);
INSERT INTO Animal VALUES ('Pingo','Animal selvagem','2021-07-21','Tartaruga','Praia','Brasil',47);
INSERT INTO Animal VALUES ('Indra','Animal selvagem','2015-10-10','Leopardo','Floresta','India',48);
INSERT INTO Animal VALUES ('Hex','Animal selvagem','2019-04-19','Hiena','Savana','Africa',49);
INSERT INTO Animal VALUES ('Gaia','Animal selvagem','2017-03-07','Leoa','Savana','Africa',50);
INSERT INTO Animal VALUES ('Seth','Animal selvagem','2016-12-14','Pantera','Floresta','Brasil',51);
INSERT INTO Animal VALUES ('Nero','Animal selvagem','2020-02-02','Lobo','Montanhas','Canada',52);
INSERT INTO Animal VALUES ('Kali','Animal selvagem','2018-05-05','Urso','Floresta','Russia',53);
INSERT INTO Animal VALUES ('Arya','Animal selvagem','2019-12-19','Pantera','Floresta','Brasil',54);
INSERT INTO Animal VALUES ('Sol','Animal selvagem','2017-09-28','Elefante','Savana','Africa',55);
INSERT INTO Animal VALUES ('Valen','Animal selvagem','2021-01-09','Leopardo','Floresta','India',56);
INSERT INTO Animal VALUES ('Echo','Animal selvagem','2016-06-23','Lobo','Montanhas','EUA',57);
INSERT INTO Animal VALUES ('Fera','Animal selvagem','2020-08-16','Tigre','Floresta','China',58);
INSERT INTO Animal VALUES ('Shade','Animal selvagem','2018-11-05','Pantera','Floresta','Brasil',59);
INSERT INTO Animal VALUES ('Ghost','Animal selvagem','2015-01-13','Lobo','Montanhas','Canada',60);
INSERT INTO Animal VALUES ('Kora','Animal selvagem','2019-02-17','Elefante','Savana','Africa',61);
INSERT INTO Animal VALUES ('Aryon','Animal selvagem','2014-07-29','Urso','Floresta','Russia',62);
INSERT INTO Animal VALUES ('Tasha','Animal selvagem','2020-05-21','Pantera','Floresta','Brasil',63);
INSERT INTO Animal VALUES ('Ravi','Animal selvagem','2018-03-18','Tigre','Floresta','India',64);
INSERT INTO Animal VALUES ('Nero2','Animal selvagem','2017-11-30','Leao','Savana','Africa',65);
INSERT INTO Animal VALUES ('Lira','Animal selvagem','2021-06-08','Tartaruga','Praia','Brasil',66);
INSERT INTO Animal VALUES ('Uruk','Animal selvagem','2019-10-10','Hiena','Savana','Africa',67);
INSERT INTO Animal VALUES ('Miro','Animal selvagem','2016-09-09','Leopardo','Floresta','India',68);
INSERT INTO Animal VALUES ('Zion','Animal selvagem','2022-04-03','Pantera','Floresta','Brasil',69);
INSERT INTO Animal VALUES ('Rocco','Animal selvagem','2018-08-11','Urso','Floresta','Russia',70);
INSERT INTO Animal VALUES ('Sora','Animal selvagem','2017-12-22','Lobo','Montanhas','EUA',71);
INSERT INTO Animal VALUES ('Kira','Animal selvagem','2020-09-14','Guepardo','Savana','Africa',72);
INSERT INTO Animal VALUES ('Blitz','Animal selvagem','2018-04-20','Pantera','Floresta','Brasil',73);
INSERT INTO Animal VALUES ('Juno','Animal selvagem','2019-11-27','Leoa','Savana','Africa',74);
INSERT INTO Animal VALUES ('Flint','Animal selvagem','2016-01-01','Urso','Floresta','Canada',75);
INSERT INTO Animal VALUES ('Odin','Animal selvagem','2015-07-09','Lobo','Montanhas','Russia',76);
INSERT INTO Animal VALUES ('Milo2','Animal selvagem','2021-04-18','Tigre','Floresta','China',77);
INSERT INTO Animal VALUES ('Uma','Animal selvagem','2018-05-31','Pantera','Floresta','Brasil',78);
INSERT INTO Animal VALUES ('Rin','Animal selvagem','2020-12-05','Leao','Savana','Africa',79);
INSERT INTO Animal VALUES ('Zya','Animal selvagem','2017-10-18','Guepardo','Savana','Africa',80);
INSERT INTO Animal VALUES ('Puma','Animal selvagem','2022-01-14','Pantera','Floresta','Brasil',81);
INSERT INTO Animal VALUES ('Ryu','Animal selvagem','2016-06-16','Lobo','Montanhas','EUA',82);
INSERT INTO Animal VALUES ('Frost','Animal selvagem','2019-09-23','Urso','Floresta','Canada',83);
INSERT INTO Animal VALUES ('Mira','Animal selvagem','2020-03-12','Pantera','Floresta','Brasil',84);
INSERT INTO Animal VALUES ('Nox','Animal selvagem','2018-08-26','Tigre','Floresta','India',85);
INSERT INTO Animal VALUES ('Silas','Animal selvagem','2015-02-17','Leao','Savana','Africa',86);
INSERT INTO Animal VALUES ('Opal','Animal selvagem','2017-11-25','Leopardo','Floresta','India',87);
INSERT INTO Animal VALUES ('Rune','Animal selvagem','2021-07-07','Pantera','Floresta','Brasil',88);
INSERT INTO Animal VALUES ('Boris','Animal selvagem','2019-05-13','Urso','Floresta','Russia',89);
INSERT INTO Animal VALUES ('Kron','Animal selvagem','2016-03-21','Lobo','Montanhas','Canada',90);
INSERT INTO Animal VALUES ('Zephyr','Animal selvagem','2020-04-30','Pantera','Floresta','Brasil',91);
INSERT INTO Animal VALUES ('Sasha','Animal selvagem','2018-12-02','Tigre','Floresta','China',92);
INSERT INTO Animal VALUES ('Titan','Animal selvagem','2017-01-19','Leao','Savana','Africa',93);
INSERT INTO Animal VALUES ('Vera','Animal selvagem','2021-02-26','Tartaruga','Praia','Brasil',94);
INSERT INTO Animal VALUES ('Hati','Animal selvagem','2019-09-12','Lobo','Montanhas','EUA',95);
INSERT INTO Animal VALUES ('Fenrir','Animal selvagem','2016-11-11','Urso','Floresta','Russia',96);
INSERT INTO Animal VALUES ('Nyx','Animal selvagem','2020-08-03','Pantera','Floresta','Brasil',97);
INSERT INTO Animal VALUES ('Echo2','Animal selvagem','2017-05-05','Lobo','Montanhas','Canada',98);
INSERT INTO Animal VALUES ('Lotus','Animal selvagem','2019-10-29','Pantera','Floresta','Brasil',99);
INSERT INTO Animal VALUES ('Blanca','Animal selvagem','2018-03-30','Tigre','Floresta','India',100);

INSERT INTO Cares VALUES ('Alimentacao','Fornecimento de comida adequada','Diaria',1);
INSERT INTO Cares VALUES ('Exame Veterinario','Avaliação de saúde completa','Mensal',2);
INSERT INTO Cares VALUES ('Vacinacao','Aplicação de vacinas','Anual',3);
INSERT INTO Cares VALUES ('Treinamento','Treino comportamental','Semanal',4);
INSERT INTO Cares VALUES ('Higiene','Limpeza e banho','Semanal',5);
INSERT INTO Cares VALUES ('Enriquecimento','Atividades mentais','Diaria',6);
INSERT INTO Cares VALUES ('Pesagem','Controle de peso','Mensal',7);
INSERT INTO Cares VALUES ('Avaliacao Dentaria','Limpeza dental','Semestral',8);
INSERT INTO Cares VALUES ('Hidratacao','Monitoramento de água','Diaria',9);
INSERT INTO Cares VALUES ('Observacao','Monitoramento geral','Diaria',10);

INSERT INTO Animal_Care_have VALUES ('2024-01-01',1,1,1);
INSERT INTO Animal_Care_have VALUES ('2024-01-02',2,2,2);
INSERT INTO Animal_Care_have VALUES ('2024-01-03',3,3,3);
INSERT INTO Animal_Care_have VALUES ('2024-01-04',4,4,4);
INSERT INTO Animal_Care_have VALUES ('2024-01-05',5,5,5);
INSERT INTO Animal_Care_have VALUES ('2024-01-06',6,6,6);
INSERT INTO Animal_Care_have VALUES ('2024-01-07',7,7,7);
INSERT INTO Animal_Care_have VALUES ('2024-01-08',8,8,8);
INSERT INTO Animal_Care_have VALUES ('2024-01-09',9,9,9);
INSERT INTO Animal_Care_have VALUES ('2024-01-10',10,10,10);

INSERT INTO Animal_Care_have VALUES ('2024-01-11',1,11,11);
INSERT INTO Animal_Care_have VALUES ('2024-01-12',2,12,12);
INSERT INTO Animal_Care_have VALUES ('2024-01-13',3,13,13);
INSERT INTO Animal_Care_have VALUES ('2024-01-14',4,14,14);
INSERT INTO Animal_Care_have VALUES ('2024-01-15',5,15,15);
INSERT INTO Animal_Care_have VALUES ('2024-01-16',6,16,16);
INSERT INTO Animal_Care_have VALUES ('2024-01-17',7,17,17);
INSERT INTO Animal_Care_have VALUES ('2024-01-18',8,18,18);
INSERT INTO Animal_Care_have VALUES ('2024-01-19',9,19,19);
INSERT INTO Animal_Care_have VALUES ('2024-01-20',10,20,20);

INSERT INTO Animal_Care_have VALUES ('2024-01-21',1,21,21);
INSERT INTO Animal_Care_have VALUES ('2024-01-22',2,22,22);
INSERT INTO Animal_Care_have VALUES ('2024-01-23',3,23,23);
INSERT INTO Animal_Care_have VALUES ('2024-01-24',4,24,24);
INSERT INTO Animal_Care_have VALUES ('2024-01-25',5,25,25);
INSERT INTO Animal_Care_have VALUES ('2024-01-26',6,26,26);
INSERT INTO Animal_Care_have VALUES ('2024-01-27',7,27,27);
INSERT INTO Animal_Care_have VALUES ('2024-01-28',8,28,28);
INSERT INTO Animal_Care_have VALUES ('2024-01-29',9,29,29);
INSERT INTO Animal_Care_have VALUES ('2024-01-30',10,30,30);

INSERT INTO Animal_Care_have VALUES ('2024-01-31',1,31,31);
INSERT INTO Animal_Care_have VALUES ('2024-02-01',2,32,32);
INSERT INTO Animal_Care_have VALUES ('2024-02-02',3,33,33);
INSERT INTO Animal_Care_have VALUES ('2024-02-03',4,34,34);
INSERT INTO Animal_Care_have VALUES ('2024-02-04',5,35,35);
INSERT INTO Animal_Care_have VALUES ('2024-02-05',6,36,36);
INSERT INTO Animal_Care_have VALUES ('2024-02-06',7,37,37);
INSERT INTO Animal_Care_have VALUES ('2024-02-07',8,38,38);
INSERT INTO Animal_Care_have VALUES ('2024-02-08',9,39,39);
INSERT INTO Animal_Care_have VALUES ('2024-02-09',10,40,40);

INSERT INTO Animal_Care_have VALUES ('2024-02-10',1,41,41);
INSERT INTO Animal_Care_have VALUES ('2024-02-11',2,42,42);
INSERT INTO Animal_Care_have VALUES ('2024-02-12',3,43,43);
INSERT INTO Animal_Care_have VALUES ('2024-02-13',4,44,44);
INSERT INTO Animal_Care_have VALUES ('2024-02-14',5,45,45);
INSERT INTO Animal_Care_have VALUES ('2024-02-15',6,46,46);
INSERT INTO Animal_Care_have VALUES ('2024-02-16',7,47,47);
INSERT INTO Animal_Care_have VALUES ('2024-02-17',8,48,48);
INSERT INTO Animal_Care_have VALUES ('2024-02-18',9,49,49);
INSERT INTO Animal_Care_have VALUES ('2024-02-19',10,50,50);

INSERT INTO Animal_Care_have VALUES ('2024-02-20',1,51,51);
INSERT INTO Animal_Care_have VALUES ('2024-02-21',2,52,52);
INSERT INTO Animal_Care_have VALUES ('2024-02-22',3,53,53);
INSERT INTO Animal_Care_have VALUES ('2024-02-23',4,54,54);
INSERT INTO Animal_Care_have VALUES ('2024-02-24',5,55,55);
INSERT INTO Animal_Care_have VALUES ('2024-02-25',6,56,56);
INSERT INTO Animal_Care_have VALUES ('2024-02-26',7,57,57);
INSERT INTO Animal_Care_have VALUES ('2024-02-27',8,58,58);
INSERT INTO Animal_Care_have VALUES ('2024-02-28',9,59,59);
INSERT INTO Animal_Care_have VALUES ('2024-02-29',10,60,60);

INSERT INTO Animal_Care_have VALUES ('2024-03-01',1,61,61);
INSERT INTO Animal_Care_have VALUES ('2024-03-02',2,62,62);
INSERT INTO Animal_Care_have VALUES ('2024-03-03',3,63,63);
INSERT INTO Animal_Care_have VALUES ('2024-03-04',4,64,64);
INSERT INTO Animal_Care_have VALUES ('2024-03-05',5,65,65);
INSERT INTO Animal_Care_have VALUES ('2024-03-06',6,66,66);
INSERT INTO Animal_Care_have VALUES ('2024-03-07',7,67,67);
INSERT INTO Animal_Care_have VALUES ('2024-03-08',8,68,68);
INSERT INTO Animal_Care_have VALUES ('2024-03-09',9,69,69);
INSERT INTO Animal_Care_have VALUES ('2024-03-10',10,70,70);

INSERT INTO Animal_Care_have VALUES ('2024-03-11',1,71,71);
INSERT INTO Animal_Care_have VALUES ('2024-03-12',2,72,72);
INSERT INTO Animal_Care_have VALUES ('2024-03-13',3,73,73);
INSERT INTO Animal_Care_have VALUES ('2024-03-14',4,74,74);
INSERT INTO Animal_Care_have VALUES ('2024-03-15',5,75,75);
INSERT INTO Animal_Care_have VALUES ('2024-03-16',6,76,76);
INSERT INTO Animal_Care_have VALUES ('2024-03-17',7,77,77);
INSERT INTO Animal_Care_have VALUES ('2024-03-18',8,78,78);
INSERT INTO Animal_Care_have VALUES ('2024-03-19',9,79,79);
INSERT INTO Animal_Care_have VALUES ('2024-03-20',10,80,80);

INSERT INTO Animal_Care_have VALUES ('2024-03-21',1,81,81);
INSERT INTO Animal_Care_have VALUES ('2024-03-22',2,82,82);
INSERT INTO Animal_Care_have VALUES ('2024-03-23',3,83,83);
INSERT INTO Animal_Care_have VALUES ('2024-03-24',4,84,84);
INSERT INTO Animal_Care_have VALUES ('2024-03-25',5,85,85);
INSERT INTO Animal_Care_have VALUES ('2024-03-26',6,86,86);
INSERT INTO Animal_Care_have VALUES ('2024-03-27',7,87,87);
INSERT INTO Animal_Care_have VALUES ('2024-03-28',8,88,88);
INSERT INTO Animal_Care_have VALUES ('2024-03-29',9,89,89);
INSERT INTO Animal_Care_have VALUES ('2024-03-30',10,90,90);

INSERT INTO Animal_Care_have VALUES ('2024-03-31',1,91,91);
INSERT INTO Animal_Care_have VALUES ('2024-04-01',2,92,92);
INSERT INTO Animal_Care_have VALUES ('2024-04-02',3,93,93);
INSERT INTO Animal_Care_have VALUES ('2024-04-03',4,94,94);
INSERT INTO Animal_Care_have VALUES ('2024-04-04',5,95,95);
INSERT INTO Animal_Care_have VALUES ('2024-04-05',6,96,96);
INSERT INTO Animal_Care_have VALUES ('2024-04-06',7,97,97);
INSERT INTO Animal_Care_have VALUES ('2024-04-07',8,98,98);
INSERT INTO Animal_Care_have VALUES ('2024-04-08',9,99,99);
INSERT INTO Animal_Care_have VALUES ('2024-04-09',10,100,100);
