import { Sheet, Stack } from "@mui/joy";

const ExplorePage = () => {
  return (
    <Sheet
      sx={{
        width: "80%",
        minHeight: "100%",
        display: "flex",
        flexDirection: "column",
        boxShadow: "md",
      }}
    >
      <Stack direction="row" spacing={2} sx={{ p: 2 }}>
        <div>Filtro</div>
        <div>Orden</div>
      </Stack>
    </Sheet>
  );
};

export default ExplorePage;
