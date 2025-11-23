import { useEffect, useState } from "react";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";

registerLocale("pt-BR", ptBR);

export default function ModifyAnimalPopup({ animal, onSave, onCancel, animalIndex, totalAnimals }) {
    const [formData, setFormData] = useState({
        name: animal.name,
        specie: animal.specie,
        habitat: animal.habitat || "",
        description: animal.description || "",
        country_of_origin: animal.country_of_origin || "",
        date_of_birth: animal.date_of_birth ? new Date(animal.date_of_birth) : null
    });

    const [enableCareManagement, setEnableCareManagement] = useState(false);
    const [cares, setCares] = useState([]);
    const [selectedCareId, setSelectedCareId] = useState("");
    const [createNewCare, setCreateNewCare] = useState(false);
    const [newCare, setNewCare] = useState({
        type_of_care: "",
        description: "",
        frequency: ""
    });
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    useEffect(() => {
        if (enableCareManagement) {
            fetchCares();
        }
    }, [enableCareManagement]);

    async function fetchCares() {
        try {
            setLoading(true);
            const response = await fetch("http://localhost:3000/cares/list");
            if (!response.ok) throw new Error("Falha ao carregar cuidados");
            const data = await response.json();
            setCares(data);
        } catch (e) {
            setError(e.message);
        } finally {
            setLoading(false);
        }
    }

    function handleChange(e) {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    }

    function handleNewCareChange(e) {
        setNewCare({ ...newCare, [e.target.name]: e.target.value });
    }

    async function handleSave() {
        try {
            setError(null);
            
            // Validate required fields
            if (!formData.name || formData.name.trim() === '') {
                setError('O campo Nome é obrigatório');
                return;
            }
            if (!formData.specie || formData.specie.trim() === '') {
                setError('O campo Espécie é obrigatório');
                return;
            }
            
            // Prepare modified animal with updated data
            const modifiedAnimal = {
                animal_id: animal.animal_id,
                name: formData.name.trim(),
                specie: formData.specie.trim(),
                habitat: formData.habitat?.trim() || null,
                description: formData.description?.trim() || null,
                country_of_origin: formData.country_of_origin?.trim() || null,
                date_of_birth: formData.date_of_birth
            };

            let careData = null;

            // Handle care creation or selection
            if (enableCareManagement) {
                if (createNewCare) {
                    // Validate care fields
                    if (!newCare.type_of_care || newCare.type_of_care.trim() === '') {
                        setError('O campo Tipo de Cuidado é obrigatório');
                        return;
                    }
                    if (!newCare.frequency || newCare.frequency.trim() === '') {
                        setError('O campo Frequência é obrigatório');
                        return;
                    }
                    
                    // Prepare new care data
                    careData = {
                        isNewCare: true,
                        type_of_care: newCare.type_of_care.trim(),
                        description: newCare.description?.trim() || null,
                        frequency: newCare.frequency.trim(),
                        date_of_care: new Date()
                    };
                } else if (selectedCareId) {
                    // Use existing care
                    careData = {
                        isNewCare: false,
                        careId: parseInt(selectedCareId),
                        date_of_care: new Date()
                    };
                } else {
                    // Care management enabled but no care selected
                    setError('Por favor, selecione um cuidado existente ou crie um novo');
                    return;
                }
            }

            onSave(modifiedAnimal, careData);
        } catch (e) {
            setError(e.message);
        }
    }

    return (
        <div style={{
            position: "fixed",
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            background: "rgba(0,0,0,0.5)",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            zIndex: 1000
        }}>
            <div className="modal-surface" style={{maxWidth:"600px", maxHeight:"80vh", overflow:"auto", width:"90%"}}>
                <h2>Modificar Animal ({animalIndex} de {totalAnimals})</h2>
                <p><strong>Animal:</strong> {animal.name}</p>
                
                {error && <p style={{ color: "red" }}>{error}</p>}

                <div style={{ marginBottom: "1rem" }}>
                    <label>Nome:</label>
                    <input
                        type="text"
                        name="name"
                        value={formData.name}
                        onChange={handleChange}
                        style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
                    />
                </div>

                <div style={{ marginBottom: "1rem" }}>
                    <label>Espécie:</label>
                    <input
                        type="text"
                        name="specie"
                        value={formData.specie}
                        onChange={handleChange}
                        style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
                    />
                </div>

                <div style={{ marginBottom: "1rem" }}>
                    <label>Habitat:</label>
                    <input
                        type="text"
                        name="habitat"
                        value={formData.habitat}
                        onChange={handleChange}
                        style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
                    />
                </div>

                <div style={{ marginBottom: "1rem" }}>
                    <label>Descrição:</label>
                    <textarea
                        name="description"
                        value={formData.description}
                        onChange={handleChange}
                        style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem", minHeight: "60px" }}
                    />
                </div>

                <div style={{ marginBottom: "1rem" }}>
                    <label>País de Origem:</label>
                    <input
                        type="text"
                        name="country_of_origin"
                        value={formData.country_of_origin}
                        onChange={handleChange}
                        style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
                    />
                </div>

                <div style={{ marginBottom: "1rem" }}>
                    <label>Data de Nascimento:</label>
                    <div>
                        <DatePicker
                            selected={formData.date_of_birth}
                            onChange={date => setFormData({ ...formData, date_of_birth: date })}
                            dateFormat="dd/MM/yyyy"
                            locale="pt-BR"
                            style={{ width: "100%", padding: "0.5rem" }}
                        />
                    </div>
                </div>

                <div style={{ marginBottom: "1rem", borderTop: "1px solid #ccc", paddingTop: "1rem" }}>
                    <label>
                        <input
                            type="checkbox"
                            checked={enableCareManagement}
                            onChange={(e) => setEnableCareManagement(e.target.checked)}
                        />
                        {" "}Adicionar/Gerenciar Cuidado
                    </label>
                </div>

                {enableCareManagement && (
                    <div style={{ marginLeft: "1.5rem", padding: "1rem", background: "#f5f5f5", borderRadius: "4px" }}>
                        {loading && <p>Carregando cuidados...</p>}
                        
                        <div style={{ marginBottom: "1rem" }}>
                            <label>
                                <input
                                    type="radio"
                                    checked={!createNewCare}
                                    onChange={() => setCreateNewCare(false)}
                                />
                                {" "}Selecionar Cuidado Existente
                            </label>
                        </div>

                        {!createNewCare && (
                            <div style={{ marginBottom: "1rem", marginLeft: "1.5rem" }}>
                                <select
                                    value={selectedCareId}
                                    onChange={(e) => setSelectedCareId(e.target.value)}
                                    style={{ width: "100%", padding: "0.5rem" }}
                                >
                                    <option value="">-- Selecione um cuidado --</option>
                                    {cares.map(care => (
                                        <option key={care.cares_id} value={care.cares_id}>
                                            {care.type_of_care} - {care.frequency}
                                        </option>
                                    ))}
                                </select>
                            </div>
                        )}

                        <div style={{ marginBottom: "1rem" }}>
                            <label>
                                <input
                                    type="radio"
                                    checked={createNewCare}
                                    onChange={() => setCreateNewCare(true)}
                                />
                                {" "}Criar Novo Cuidado
                            </label>
                        </div>

                        {createNewCare && (
                            <div style={{ marginLeft: "1.5rem" }}>
                                <div style={{ marginBottom: "0.5rem" }}>
                                    <input
                                        type="text"
                                        name="type_of_care"
                                        placeholder="Tipo de cuidado..."
                                        value={newCare.type_of_care}
                                        onChange={handleNewCareChange}
                                        style={{ width: "100%", padding: "0.5rem" }}
                                    />
                                </div>
                                <div style={{ marginBottom: "0.5rem" }}>
                                    <input
                                        type="text"
                                        name="description"
                                        placeholder="Descrição..."
                                        value={newCare.description}
                                        onChange={handleNewCareChange}
                                        style={{ width: "100%", padding: "0.5rem" }}
                                    />
                                </div>
                                <div style={{ marginBottom: "0.5rem" }}>
                                    <input
                                        type="text"
                                        name="frequency"
                                        placeholder="Frequência (ex: Diária, Semanal)..."
                                        value={newCare.frequency}
                                        onChange={handleNewCareChange}
                                        style={{ width: "100%", padding: "0.5rem" }}
                                    />
                                </div>
                            </div>
                        )}
                    </div>
                )}

                <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem", justifyContent: "flex-end" }}>
                    <button className="btn-cancel" onClick={onCancel}>Cancelar</button>
                    <button className="btn-confirm" onClick={handleSave}>Salvar</button>
                </div>
            </div>
        </div>
    );
}
