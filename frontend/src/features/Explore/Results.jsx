import Grid from "@mui/joy/Grid";
import Typography from "@mui/joy/Typography";
import { useNavigate } from "react-router-dom";
import MachineCard from "./MachineCard";
import MachineCardSkeleton from "./MachineCardSkeleton";

const Results = ({ results, loading }) => {
  const nav = useNavigate();

  return (
    <Grid container spacing={2} sx={{ px: 5, pb: 2 }}>
      {loading ? (
        new Array(8).fill("").map((_, i) => (
          <Grid xs={6} sm={6} md={4} lg={3} key={i}>
            <MachineCardSkeleton />
          </Grid>
        ))
      ) : Array.isArray(results) && results.length > 0 ? (
        results.map((machine) => (
          <Grid xs={6} sm={6} md={4} lg={3} key={machine.id}>
            <MachineCard
              imageUrl={machine.main_image}
              model={machine.model}
              model_id={machine.id}
              categories={machine.categories}
              price={machine.price}
              name={machine.name}
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
