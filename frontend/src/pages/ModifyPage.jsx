import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";
import PopupCare from "./components/PopupCare";
import ModifyAnimalPopup from "./components/ModifyAnimalPopup";

registerLocale("pt-BR", ptBR);

export default function ModifyPage() {
    const [animals, setAnimals] = useState([]);
    const [filtered, setFiltered] = useState([]);
    const [search, setSearch] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [selectedAnimals, setSelectedAnimals] = useState([]);
    const [currentModifyIndex, setCurrentModifyIndex] = useState(null);
    // tirar depois
    const [addAnimalDate, setAddAnimalDate] = useState(null);
    const [newAnimal, setNewAnimal] = useState({
        name: "",
        specie: "",
        habitat: "",
        description: "",
        country_of_origin: "",
        date_of_birth: null
    });

    const navigate = useNavigate();

    useEffect(() => {
        setFiltered(
                animals.filter((animal) => {
                    const name = animal.name.toLowerCase();
                    const specie = animal.specie ? animal.specie.toLowerCase() : "";
                    const searchText = search.toLowerCase();
                    return name.includes(searchText) || specie.includes(searchText);
                })
            );
    }, [search, animals]);

    useEffect(() => {
        getAnimals();
    }, []);

    async function getAnimals() {
        try {
            setLoading(true);
            setError(null);
            const response = await fetch("http://localhost:3000/animals/list");
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            setAnimals(data);
            setFiltered(data);
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }

    async function addAnimal() {
        setError(null);
        const payload = {
            name: newAnimal.name,
            specie: newAnimal.specie,
            habitat: newAnimal.habitat || null,
            description: newAnimal.description || null,
            country_of_origin: newAnimal.country_of_origin || null,
            date_of_birth: newAnimal.date_of_birth
                ? newAnimal.date_of_birth.toISOString().slice(0, 10)
                : null,
        };
        try {
            const response = await fetch('http://localhost:3000/animals/add', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload),
            });
            if (!response.ok) {
                throw new Error(`Failed to create animal: ${response.status}`);
            }
            const created = await response.json();
            // Update lists optimistically
            setAnimals(prev => [...prev, created]);
            setFiltered(prev => [...prev, created]);
            // Reset form
            setNewAnimal({
                name: '',
                specie: '',
                habitat: '',
                description: '',
                country_of_origin: '',
                date_of_birth: null,
            });
        } catch (e) {
            setError(e.message);
        }
        navigate("/");
    }

    function handleSearchChange(e) {
        setSearch(e.target.value);
    }

    function handleChange(e) {
        setNewAnimal({ ...newAnimal, [e.target.name]: e.target.value });
    }

    function toggleAnimalSelection(animalId) {
        setSelectedAnimals(prev => 
            prev.includes(animalId) 
                ? prev.filter(id => id !== animalId)
                : [...prev, animalId]
        );
    }

    async function deleteSelectedAnimals() {
        if (selectedAnimals.length === 0) {
            setError("Nenhum animal selecionado para excluir");
            return;
        }

        try {
            setError(null);
            const deletePromises = selectedAnimals.map(id =>
                fetch(`http://localhost:3000/animals/deactivate/${id}`, {
                    method: 'POST'
                })
            );

            const results = await Promise.all(deletePromises);
            const failedDeletes = results.filter(r => !r.ok);

            if (failedDeletes.length > 0) {
                throw new Error(`Falha ao excluir ${failedDeletes.length} animal(is)`);
            }

            // Remove deactivated animals from state
            setAnimals(prev => prev.filter(a => !selectedAnimals.includes(a.animal_id)));
            setFiltered(prev => prev.filter(a => !selectedAnimals.includes(a.animal_id)));
            setSelectedAnimals([]);
        } catch (e) {
            setError(e.message);
        }
    }

    function startModifyWorkflow() {
        if (selectedAnimals.length === 0) {
            setError("Nenhum animal selecionado para modificar");
            return;
        }
        setError(null);
        setCurrentModifyIndex(0);
    }

    async function handleModifySave(modifiedAnimal, careData) {
        try {
            setError(null);
            
            // Update animal
            const animalResponse = await fetch(`http://localhost:3000/animals/update/${modifiedAnimal.animal_id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name: modifiedAnimal.name,
                    specie: modifiedAnimal.specie,
                    habitat: modifiedAnimal.habitat || null,
                    description: modifiedAnimal.description || null,
                    country_of_origin: modifiedAnimal.country_of_origin || null,
                    date_of_birth: modifiedAnimal.date_of_birth 
                        ? new Date(modifiedAnimal.date_of_birth).toISOString().slice(0, 10)
                        : null
                })
            });

            if (!animalResponse.ok) {
                throw new Error('Falha ao atualizar animal');
            }

            const updatedAnimal = await animalResponse.json();

            // Handle care if provided
            if (careData) {
                let careId = careData.careId;

                // Create new care if needed
                if (careData.isNewCare) {
                    const careResponse = await fetch('http://localhost:3000/cares/add', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            type_of_care: careData.type_of_care,
                            description: careData.description,
                            frequency: careData.frequency
                        })
                    });

                    if (!careResponse.ok) {
                        throw new Error('Falha ao criar cuidado');
                    }

                    const createdCare = await careResponse.json();
                    careId = createdCare.cares_id;
                }

                // Create animal-care relationship
                const relationResponse = await fetch('http://localhost:3000/animal-cares/add', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        fk_animal_animal_id: modifiedAnimal.animal_id,
                        fk_cares_cares_id: careId,
                        date_of_care: careData.date_of_care 
                            ? new Date(careData.date_of_care).toISOString().slice(0, 10)
                            : null
                    })
                });

                if (!relationResponse.ok) {
                    throw new Error('Falha ao associar cuidado ao animal');
                }
            }

            // Update local state
            setAnimals(prev => prev.map(a => 
                a.animal_id === updatedAnimal.animal_id ? updatedAnimal : a
            ));
            setFiltered(prev => prev.map(a => 
                a.animal_id === updatedAnimal.animal_id ? updatedAnimal : a
            ));

            // Move to next animal or finish
            if (currentModifyIndex < selectedAnimals.length - 1) {
                setCurrentModifyIndex(currentModifyIndex + 1);
            } else {
                // Workflow complete
                setCurrentModifyIndex(null);
                setSelectedAnimals([]);
            }
        } catch (e) {
            setError(e.message);
        }
    }

    function handleModifyCancel() {
        // Skip to next animal or finish
        if (currentModifyIndex < selectedAnimals.length - 1) {
            setCurrentModifyIndex(currentModifyIndex + 1);
        } else {
            setCurrentModifyIndex(null);
            setSelectedAnimals([]);
        }
    }


    return (
        <div style={{display: "flex",
                    justifyContent: "center",
                    alignItems: "center",
                    width: "100vw",
                }}>
            <div >
                <h1>Modificar Animais</h1>
            <input
                type="text"
                placeholder="Pesquise um animal por nome ou espécie..."
                value={search}
                onChange={handleSearchChange}
                style={{ width: "100%", padding: "0.5rem", marginBottom: "1rem"}}
            />
            <div style={{ flex: 1, maxHeight: "600px", overflow: "auto", minWidth: "350px" }}>
                <h3 style={{ position: "sticky", top: 0, background: "#242424", color: "#ffffff", zIndex: 2, padding: "0.5rem 0" }}>Lista de Animais</h3>
                {loading && <p>Loading...</p>}
                {error && <p style={{ color: "red" }}>Error: {error}</p>}
                {filtered.map((animal) => (
                    <div
                        key={animal.animal_id}
                        style={{ 
                            border: "1px solid #ccc", 
                            marginBottom: "1rem", 
                            padding: "1rem", 
                            position: "relative",
                        }}
                    >
                        <h3>
                            <input 
                                type="checkbox" 
                                checked={selectedAnimals.includes(animal.animal_id)}
                                onChange={() => toggleAnimalSelection(animal.animal_id)}
                            />
                            {animal.name}
                        </h3>
                        <p>
                            <strong>Espécie:</strong> {animal.specie}
                        </p>
                        <p>
                            <strong>Habitat:</strong> {animal.habitat}
                        </p>
                        <p>
                            <strong>Descrição:</strong> {animal.description}
                        </p>
                        <p>
                            <strong>País de origem:</strong> {animal.country_of_origin}
                        </p>
                        <p>
                            <strong>Data de nascimento:</strong> {new Date(animal.date_of_birth).toLocaleDateString("pt-BR")}
                        </p>
                    </div>
                ))}
                {filtered.length === 0 && !loading && <p>Nenhum animal encontrado.</p>}
            </div>
            <div style={{padding: "2rem"}}>
                <button onClick={() => {navigate("/")}}>Páginal Principal</button>
                <button onClick={startModifyWorkflow}>Modificar ({selectedAnimals.length})</button>
                <button onClick={deleteSelectedAnimals}>Excluir ({selectedAnimals.length})</button>
            </div>
            
            {currentModifyIndex !== null && (
                <ModifyAnimalPopup
                    animal={animals.find(a => a.animal_id === selectedAnimals[currentModifyIndex])}
                    animalIndex={currentModifyIndex + 1}
                    totalAnimals={selectedAnimals.length}
                    onSave={handleModifySave}
                    onCancel={handleModifyCancel}
                />
            )}
            
            </div>
        </div>
    );
}
    
