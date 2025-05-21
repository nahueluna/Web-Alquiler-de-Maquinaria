import { Sheet } from "@mui/joy";
import { Divider } from "@mui/material";
import Filters from "./Filters";
import Results from "./Results";
import SortBy from "./SortBy";

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
      <Sheet sx={{ minWidth: 250, maxWidth: 350, flex: "0 0 20%" }}>
        <Filters />
      </Sheet>
      <Divider orientation="vertical" />
      <Sheet sx={{ flex: 1, minWidth: 0 }}>
        <Sheet
          sx={{
            display: "flex",
            justifyContent: "flex-end",
            px: 5,
            py: 1,
          }}
        >
          <SortBy />
        </Sheet>
        <Results />
      </Sheet>
    </Sheet>
  );
};

export default ExplorePage;
