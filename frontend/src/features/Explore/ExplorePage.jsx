import { Sheet } from "@mui/joy";
import Results from "./Results";

const ExplorePage = () => {
  return (
    <Sheet
      sx={{
        backgroundColor: "blue",
        width: "80%",
        minHeight: "50%",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Sheet>Filtros</Sheet>
      <Sheet>
        <Results />
      </Sheet>
    </Sheet>
  );
};

export default ExplorePage;
