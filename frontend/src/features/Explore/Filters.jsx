import { KeyboardArrowRight } from "@mui/icons-material";
import { Divider, Typography } from "@mui/joy";
import Button from "@mui/joy/Button";
import FormControl from "@mui/joy/FormControl";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";

const categorias = [
  "Agricultura",
  "Construccion",
  "Educacion",
  "Energia",
  "Finanzas",
  "Salud",
  "Tecnologia",
  "Transporte",
];

const Filters = () => {
  return (
    <Sheet sx={{ p: 2 }}>
      <Typography level="body-md" sx={{ fontWeight: "lg" }}>
        Categorias
      </Typography>
      {categorias.map((categoria) => (
        <Typography key={categoria} level="body-xs">
          {categoria}
        </Typography>
      ))}
      <Divider sx={{ my: 2 }} />
      <Typography level="body-md" sx={{ fontWeight: "lg" }}>
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
        <FormControl sx={{ minWidth: 40, maxWidth: 80, width: "100%" }}>
          <Input placeholder="Minimo" size="sm" />
        </FormControl>
        <FormControl sx={{ minWidth: 40, maxWidth: 80, width: "100%" }}>
          <Input placeholder="Maximo" size="sm" />
        </FormControl>
        <Button type="submit" variant="solid" color="danger" size="sm">
          <KeyboardArrowRight />
        </Button>
      </form>
    </Sheet>
  );
};

export default Filters;
