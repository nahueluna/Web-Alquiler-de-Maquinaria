import { KeyboardArrowRight } from "@mui/icons-material";
import CheckIcon from "@mui/icons-material/Check";
import { Divider, Typography } from "@mui/joy";
import Box from "@mui/joy/Box";
import IconButton from "@mui/joy/Button";
import Checkbox from "@mui/joy/Checkbox";
import Chip from "@mui/joy/Chip";
import FormControl from "@mui/joy/FormControl";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";
import ResultsResumee from "./ResultsResumee"; // Assuming this is a custom component for displaying results summary

const categorias = [
  "Tecnología",
  "Hogar",
  "Ropa",
  "Juguetes",
  "Libros",
  "Deportes",
  "Salud",
  "Belleza",
  "Alimentos",
  "Automotriz",
];

const Filters = ({
  results,
  currentFilters,
  setSelectedFilters,
  selectedFilters,
}) => {
  return (
    <Sheet sx={{ p: 2 }}>
      <ResultsResumee
        totalItems={results.total_items}
        currentFilters={currentFilters}
      />
      <Typography level="body-lg" sx={{ fontWeight: "lg", mb: 1 }}>
        Categorias
      </Typography>
      <Box
        role="group"
        aria-labelledby="fav-movie"
        sx={{ display: "flex", flexWrap: "wrap", gap: 1 }}
      >
        {categorias.map((name) => {
          const checked = selectedFilters.categories.includes(name);
          return (
            <Chip
              key={name}
              variant="outlined"
              color={checked ? "primary" : "neutral"}
              startDecorator={
                checked && (
                  <CheckIcon sx={{ zIndex: 1, pointerEvents: "none" }} />
                )
              }
            >
              <Checkbox
                variant="outlined"
                color={checked ? "black" : "neutral"}
                disableIcon
                overlay
                label={name}
                checked={checked}
                onChange={(event) => {
                  setSelectedFilters((prev) => ({
                    ...prev,
                    categories: !event.target.checked
                      ? prev.categories.filter((n) => n !== name)
                      : [...prev.categories, name],
                  }));
                }}
              />
            </Chip>
          );
        })}
      </Box>
      <Divider sx={{ my: 2 }} />
      <Typography level="body-lg" sx={{ fontWeight: "lg" }}>
        Precio
      </Typography>
      <form
        style={{
          display: "flex",
          gap: 2,
          alignItems: "center",
          width: "auto",
          marginTop: 8,
        }}
      >
        <FormControl sx={{ minWidth: 40, maxWidth: 100, width: "100%" }}>
          <Input placeholder="Minimo" size="sm" />
        </FormControl>
        -
        <FormControl sx={{ minWidth: 40, maxWidth: 100, width: "100%" }}>
          <Input placeholder="Maximo" size="sm" />
        </FormControl>
        <IconButton
          type="submit"
          variant="solid"
          color="danger"
          size="sm"
          sx={{
            borderRadius: "50%",
            width: 32,
            height: 32,
            minWidth: 32,
            minHeight: 32,
            p: 0,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <KeyboardArrowRight />
        </IconButton>
      </form>
    </Sheet>
  );
};

export default Filters;
