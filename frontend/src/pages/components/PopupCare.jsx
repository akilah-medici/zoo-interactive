import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";

registerLocale("pt-BR", ptBR);

export default function ListPage() {
    const [cares, setCares] = useState([]);
    const [animalCares, setAnimalCares] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    const navigate = useNavigate();

    useEffect(() => {
        getCares();
    }, []);

    async function getAnimalCares() {
        try {
            setLoading(true);
            setError(null);
            const response = await fetch("http://localhost:3000/animal-cares/list");
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            setAnimalCares(data);
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }

    return (
        <div>
            {cares.map((care) => (
                <div key={care.cares_id}>
                    {care.type_of_care}
                </div>
                )
            )}
        </div>
    );
}
    
