import { Sheet } from "@mui/joy";
import { Divider } from "@mui/material";
import Results from "./Results";

const ExplorePage = () => {
  return (
    <Sheet
      sx={{
        minWidth: "80%",
        maxWidth: "995px",
        minHeight: "100%",
        backgroundColor: "white",
        display: "flex",
        flexDirection: "row",
        boxShadow: "0 1px 4px rgba(0, 0, 0, 0.1)",
      }}
    >
      <Sheet sx={{ width: "15%", minWidth: "150px" }}>Filtros</Sheet>
      <Divider orientation="vertical" />
      <Sheet sx={{ width: "85%" }}>
        <Sheet>Orden</Sheet>
        <Results />
      </Sheet>
    </Sheet>
  );
};

export default ExplorePage;
