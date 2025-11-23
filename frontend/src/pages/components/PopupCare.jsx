import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";

import './PopupCare.css';

export default function PopupCare({id, inline = false}) {
    const [cares, setCares] = useState([]);
    const [animalCares, setAnimalCares] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    const navigate = useNavigate();

    useEffect(() => {
        getAnimalCares(id);
    }, []);

    async function getAnimalCares(id) {
        try {
            setLoading(true);
            setError(null);
            const response = await fetch(`http://localhost:3000/animal-cares/by-animal/by-id/${id}`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            setAnimalCares(data);
            getCaresByID(data.fk_cares_cares_id);
            console.log(data);
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }

    async function getCaresByID(id) {
        try {
            setLoading(true);
            setError(null);
            const response = await fetch(`http://localhost:3000/cares/by-id/${id}`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            setCares(data);
            console.log(data);
        } catch (err) {
            setError(err.message);
            setCares(["Nenhum cuidado necessário"])
        } finally {
            setLoading(false);
        }
    }

    return (
        <div id="background" className="popup-care" style={{position:inline?"static":"fixed", top:inline?"auto":"20px", right:inline?"auto":"20px", zIndex:inline?1:1000, minWidth:"260px", maxWidth:"340px"}}>   
            {cares && cares.type_of_care
            ? (
                <>
                    <h2>Cuidados</h2>
                    <div>Tipo do cuidado: {cares.type_of_care}</div>
                    <div>Descrição: {cares.description}</div>
                    <div>Frequência: {cares.frequency}</div>
                </>
                )
            : (
                <div>Nenhum cuidado necessário</div>
                )
            }
        </div>
    );
}
    
