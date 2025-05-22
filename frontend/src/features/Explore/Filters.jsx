import { KeyboardArrowRight } from "@mui/icons-material";
import { Divider, Typography } from "@mui/joy";
import IconButton from "@mui/joy/Button";
import FormControl from "@mui/joy/FormControl";
import Input from "@mui/joy/Input";
import Link from "@mui/joy/Link";
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
      <Typography level="body-lg" sx={{ fontWeight: "lg" }}>
        Categorias
      </Typography>
      {categorias.map((categoria) => (
        <Typography key={categoria} level="body-md" color="">
          <Link color="neutral">{categoria}</Link>
        </Typography>
      ))}
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
