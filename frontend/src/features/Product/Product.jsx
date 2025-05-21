import { Box, Button, Divider, Sheet, Table, Typography } from "@mui/joy";
import Img from "../../assets/sierra.png";

const maquina = {
  name: 'SIERRA CIRCULAR 9" BOSCH 220V HD 2100 W GKS 235',
  desc: "GKS 235 tiene dos miras para aumentar la precisión de corte, la base GKS 235 se puede inclinar hasta 48°, también tiene un sistema de eliminación de virutas optimizado.",
  images: ["../../assets/SoloLogo.png"],
};

function Product() {
  return (
    <Sheet
      sx={{
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* TODO: add modal */}

      {/* Product info */}
      <Sheet
        sx={{
          display: "flex",
          flexDirection: {
            sm: "column",
            lg: "row",
          },
          alignItems: "center",
          justifyContent: "center",
          pt: 20,
        }}
      >
        {/* TODO: multiple images */}
        <Sheet>
          <img width={500} src={Img} alt="" />
        </Sheet>

        <Sheet>
          <Typography level="h2" maxWidth={500}>
            {maquina.name}
          </Typography>
          <Typography textColor={"neutral.500"} level="body-md" maxWidth={500}>
            {maquina.desc}
          </Typography>
          <Typography my={5} level="h3">
            $15.200 x dia
          </Typography>
          <Button sx={{ width: "100%" }} size="lg" color="danger">
            Alquilar
          </Button>
        </Sheet>
      </Sheet>

      <Divider sx={{ mt: 20, mb: 5 }} />

      <Sheet
        sx={{
          alignSelf: "center",
        }}
      >
        <Typography level="h4">Caracteristicas generales</Typography>
        <Table
          sx={{
            maxWidth: "500px",
          }}
          stripe={"odd"}
          borderAxis="none"
        >
          <tbody>
            <tr>
              <td>Marca</td>
              <td>Bosch</td>
            </tr>
            <tr>
              <td>Modelo</td>
              <td>GKS 235</td>
            </tr>
            <tr>
              <td>Anio</td>
              <td>2020</td>
            </tr>
            <tr>
              <td>Numero de serie</td>
              <td>23520201</td>
            </tr>
          </tbody>
        </Table>
      </Sheet>

      <Divider sx={{ my: 5 }} />

      {/* Otros productos */}
      <Sheet
        sx={{
          py: 2,
        }}
      >
        <Typography level="h3">Otros productos</Typography>
        {/* TODO: Use/make a slider for this */}
        <Sheet
          sx={{
            display: "flex",
            gap: 2,
          }}
        >
          {/* TODO: Add product cards */}
          {new Array(6).fill("").map((_, i) => (
            <Box
              key={i}
              sx={{
                width: "200px",
                aspectRatio: "1/1",
                backgroundColor: "red",
              }}
            ></Box>
          ))}
        </Sheet>
      </Sheet>
    </Sheet>
  );
}

export default Product;
