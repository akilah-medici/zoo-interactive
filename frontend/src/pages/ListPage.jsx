import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";
import PopupCare from "./components/PopupCare";

registerLocale("pt-BR", ptBR);

export default function ListPage() {
    const [animals, setAnimals] = useState([]);
    const [filtered, setFiltered] = useState([]);
    const [search, setSearch] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [popupController, setPopupController] = useState(false);
    const [animalPopupID, setAnimalPopupID] = useState(null);
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
        
        if (!newAnimal.name || newAnimal.name.trim() === '') {
            setError('O campo Nome é obrigatório');
            return;
        }
        if (!newAnimal.specie || newAnimal.specie.trim() === '') {
            setError('O campo Espécie é obrigatório');
            return;
        }
        
        const payload = {
            name: newAnimal.name.trim(),
            specie: newAnimal.specie.trim(),
            habitat: newAnimal.habitat?.trim() || null,
            description: newAnimal.description?.trim() || null,
            country_of_origin: newAnimal.country_of_origin?.trim() || null,
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

            setAnimals(prev => [...prev, created]);
            setFiltered(prev => [...prev, created]);

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
        <div className="page-container" style={{padding:"2rem 1rem"}}>
            <div>
                <h1 className="header-primary">Lista de Animais</h1>
                <input
                    type="text"
                    className="search-bar"
                    placeholder="Pesquise um animal por nome ou espécie..."
                    value={search}
                    onChange={handleSearchChange}
                />
                <div style={{ display: "flex", gap: "2rem", alignItems: "flex-start" }}>

                    <div className="scroll-list" style={{flex:1, minWidth:"350px", maxWidth:"900px"}}>
                        <h3 style={{ position: "sticky", top: 0, background: "var(--color-secondary)", color: "#ffffff", zIndex: 2, padding: "0.5rem 0", margin:0, borderRadius:"6px" }}>Lista Completa</h3>
                        {loading && <p>Loading...</p>}
                        {error && <p style={{ color: "red" }}>Error: {error}</p>}
                        {filtered.map((animal, idx) => (
                            <div
                                key={animal.animal_id}
                                className="panel"
                                style={{ position: "relative", ...(idx === 0 ? { marginTop: "8px" } : {}) }}
                                onMouseEnter={() => setAnimalPopupID(animal.animal_id)}
                                onMouseLeave={() => setAnimalPopupID(null)}
                            >
                                <h3>{animal.name}</h3>
                                <p><strong>Espécie:</strong> {animal.specie}</p>
                                <p><strong>Habitat:</strong> {animal.habitat}</p>
                                <p><strong>Descrição:</strong> {animal.description}</p>
                                <p><strong>País de origem:</strong> {animal.country_of_origin}</p>
                                <p><strong>Data de nascimento:</strong> {new Date(animal.date_of_birth).toLocaleDateString("pt-BR")}</p>
                            </div>
                        ))}
                        {filtered.length === 0 && !loading && <p>Nenhum animal encontrado.</p>}
                        
                    </div>

                    <div style={{ flex: "0 0 360px" }}>
                        <h3>Adicionar Animal</h3>
                        <table>
                            <tbody>
                                <tr><td><input type="text" name="name" placeholder="Nome do animal..." value={newAnimal.name} onChange={handleChange}/></td></tr>
                                <tr><td><input type="text" name="specie" placeholder="Especie do animal..." value={newAnimal.specie} onChange={handleChange}/></td></tr>
                                <tr><td><input type="text" name="description" placeholder="Descrição do animal..." value={newAnimal.description} onChange={handleChange}/></td></tr>
                                <tr><td><input type="text" name="habitat" placeholder="Habitat do animal..." value={newAnimal.habitat} onChange={handleChange}/></td></tr>
                                <tr><td><input type="text" name="country_of_origin" placeholder="País de origem do animal..." value={newAnimal.country_of_origin} onChange={handleChange}/></td></tr>
                                <tr>
                                    <td>
                                        <DatePicker
                                            selected={newAnimal.date_of_birth}
                                            onChange={date => setNewAnimal({ ...newAnimal, date_of_birth: date })}
                                            dateFormat="dd/MM/yyyy"
                                            placeholderText="Dia/Mês/Ano"
                                            locale="pt-BR"
                                        />
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                        {animalPopupID !== null && (
                            <div style={{ marginTop: "1rem" }}>
                                <PopupCare id={animalPopupID} inline={true} />
                            </div>
                        )}
                    </div>
                </div>

                <div style={{ marginTop: "1rem", display: "flex", gap: "1rem" }}>
                    <button className="btn-outline" onClick={() => { navigate("/") }}>Página Principal</button>
                    <button className="btn-confirm" onClick={() => { addAnimal() }}>Adicionar</button>
                </div>
            </div>
        </div>
    );
}
    
