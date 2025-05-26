import Grid from "@mui/joy/Grid";
import MachineCard from "./MachineCard";
import { useSearchParams } from "react-router-dom";
import Typography from "@mui/joy/Typography";
import axios from "axios";
import { useEffect, useState } from "react";
import MachineCardSkeleton from "./MachineCardSkeleton";

const Results = () => {
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchParams] = useSearchParams();
  const query = searchParams.get("q")?.toLowerCase() || "";

  useEffect(() => {
    async function fetchResults(query) {
      try {
        const {
          data: { items },
        } = await axios.get(
          `http://localhost:8000/explore${query ? `?search=${query}` : ""}`
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
              onClick={() => console.log(machine.model)}
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
