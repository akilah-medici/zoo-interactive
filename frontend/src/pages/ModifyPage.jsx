import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";
import PopupCare from "./components/PopupCare";

registerLocale("pt-BR", ptBR);

export default function ModifyPage() {
    const [animals, setAnimals] = useState([]);
    const [filtered, setFiltered] = useState([]);
    const [search, setSearch] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
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
                        <h3><input type="checkbox" />{animal.name}</h3>
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
                <button onClick={() => {}}>Modificar</button>
                <button onClick={() => {}}>Exlcuir</button>
            </div>
            
            </div>
        </div>
    );
}
    
