import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Dashboard() {
  const [animals, setAnimals] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  useEffect(() => {
    getAnimals();
  }, []);

  async function getAnimals() {
    try {
      setLoading(true);
      setError(null);
      
      const response = await fetch('http://localhost:3000/animals/list');
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const data = await response.json();
      console.log(data);
      setAnimals(data);
    } catch (err) {
      console.error("Error fetching animals:", err);
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={styles.container}>
      <h1 style={styles.title}>Zoo Dashboard</h1>

      {loading ? (
        <p>Loading animals...</p>
      ) : error ? (
        <div style={styles.errorBox}>
          <p style={styles.errorText}>❌ Error: {error}</p>
          <button style={styles.button} onClick={fetchAnimals}>
            Retry
          </button>
        </div>
      ) : (
        <>
          <div style={styles.statsContainer}>
            <div style={styles.card}>
              <h3>Total Animals</h3>
              <p style={styles.number}>{animals.length}</p>
            </div>

            <div style={styles.card}>
              <h3>Endangered Species</h3>
              <p style={styles.number}>
                {animals.filter(a => a.endangered).length}
              </p>
            </div>

            <div style={styles.card}>
              <h3>Unique Habitats</h3>
              <p style={styles.number}>
                {new Set(animals.map(a => a.habitat).filter(Boolean)).size}
              </p>
            </div>
          </div>

          <h2 style={styles.sectionTitle}>Animals List</h2>
          
          <div style={styles.animalsGrid}>
            {animals.map((animal) => (
              <div key={animal.animal_id} style={styles.animalCard}>
                <h3 style={styles.animalName}>{animal.name}</h3>
                <p><strong>Espécie:</strong> {animal.specie}</p>
                <p><strong>Habitat:</strong> {animal.habitat || 'Unknown'}</p>
                <p>
                  <strong>Status:</strong>{' '}
                  <span style={{
                    color: animal.endangered ? '#d32f2f' : '#388e3c',
                    fontWeight: 'bold'
                  }}>
                    {animal.endangered ? '⚠️ Endangered' : '✓ Safe'}
                  </span>
                </p>
                <p style={styles.dateText}>
                  Birth: {animal.date_of_birth ? new Date(animal.date_of_birth).toLocaleDateString("pt-BR") : "Unknown"}
                </p>
              </div>
            ))}
          </div>

          {animals.length === 0 && (
            <p style={styles.emptyText}>No animals found in the database.</p>
          )}
        </>
      )}
      <button onClick={() => navigate("/list")} >Lista de Animais</button>
      <button onClick={() => navigate("/modify")} >Modificar Animais</button>
    </div>
  );
}

const styles = {
  container: {
    padding: "40px",
    fontFamily: "Arial",
    maxWidth: "1200px",
    margin: "0 auto",
  },
  title: {
    marginBottom: "25px",
    color: "#333",
  },
  sectionTitle: {
    marginTop: "40px",
    marginBottom: "20px",
    color: "#555",
  },
  statsContainer: {
    display: "flex",
    gap: "20px",
    marginBottom: "30px",
    flexWrap: "wrap",
  },
  card: {
    background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    padding: "20px",
    borderRadius: "10px",
    width: "220px",
    color: "white",
    boxShadow: "0 4px 6px rgba(0,0,0,0.1)",
  },
  number: {
    fontSize: "32px",
    fontWeight: "bold",
    margin: "10px 0",
  },
  animalsGrid: {
    display: "grid",
    gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))",
    gap: "20px",
    marginTop: "20px",
  },
  animalCard: {
    background: "#fff",
    border: "2px solid #e0e0e0",
    padding: "20px",
    borderRadius: "12px",
    boxShadow: "0 2px 8px rgba(0,0,0,0.1)",
    transition: "transform 0.2s, box-shadow 0.2s",
    cursor: "pointer",
    color: "#000000",
  },
  animalName: {
    color: "#2196F3",
    marginBottom: "10px",
    fontSize: "20px",
  },
  dateText: {
    fontSize: "12px",
    color: "#888",
    marginTop: "10px",
  },
  errorBox: {
    background: "#ffebee",
    border: "2px solid #ef5350",
    borderRadius: "8px",
    padding: "20px",
    marginTop: "20px",
  },
  errorText: {
    color: "#c62828",
    fontSize: "16px",
  },
  emptyText: {
    textAlign: "center",
    color: "#999",
    fontSize: "18px",
    marginTop: "40px",
  },
  button: {
    padding: "12px 20px",
    border: "none",
    borderRadius: "8px",
    cursor: "pointer",
    background: "#4CAF50",
    color: "white",
    fontSize: "16px",
    marginTop: "10px",
  },
};
