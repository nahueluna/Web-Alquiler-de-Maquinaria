import Grid from "@mui/joy/Grid";
import Typography from "@mui/joy/Typography";
import axios from "axios";
import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import MachineCard from "./MachineCard";
import MachineCardSkeleton from "./MachineCardSkeleton";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

const Results = () => {
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchParams] = useSearchParams();
  const nav = useNavigate();
  const query = searchParams.get("q")?.toLowerCase() || "";

  useEffect(() => {
    async function fetchResults(query) {
      try {
        const {
          data: { items },
        } = await axios.get(
          `${BACKEND_URL}/explore${query ? `?search=${query}` : ""}`
        );
        setResults(items);
        setLoading(false);
      } catch (error) {
        console.error(error);
      }
    }

    fetchResults(query); // Fetch results whenever the query changes to update them
  }, [query]);

  return (
    <Grid container spacing={2} sx={{ px: 5, pb: 2 }}>
      {loading ? (
        new Array(8).fill("").map((_, i) => (
          <Grid xs={6} sm={6} md={4} lg={3} key={i}>
            <MachineCardSkeleton />
          </Grid>
        ))
      ) : results.length > 0 ? (
        results.map((machine) => (
          <Grid xs={6} sm={6} md={4} lg={3} key={machine.id}>
            <MachineCard
              imageUrl={machine.imageUrl}
              model={machine.model}
              category={machine.category}
              price={machine.price}
              onClick={() => nav(`/explore/${machine.id}`)}
            />
          </Grid>
        ))
      ) : (
        <Typography level="body-md" sx={{ px: 5, py: 3 }}>
          No se encontraron resultados
        </Typography>
      )}
    </Grid>
  );
};

export default Results;
