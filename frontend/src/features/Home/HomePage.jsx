import { Typography, Sheet, Box, Link, Button, Stack } from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import VideoSucursalNY from "../../assets/Sucursal_NY.mp4";

function Home() {
  return (
    <Sheet
      sx={{
        minHeight: "100vh",
        width: "100%",
        position: "relative",
        overflow: "hidden",
        bgcolor: "background.body",
        display: "flex",
        alignItems: "center",
      }}
    >
      <video
        src={VideoSucursalNY}
        autoPlay
        loop
        muted
        playsInline
        controls={false}
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100vw",
          height: "100vh",
          objectFit: "cover",
          zIndex: 0,
        }}
      />
      {/* Overlay oscuro */}
      <Box
        sx={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100vw",
          height: "100vh",
          bgcolor: "rgba(0,0,0,0.45)",
          zIndex: 1,
        }}
      />

      <Stack
        sx={{
          position: "relative",
          zIndex: 2,
          width: { sm: "100%", md: "90%", lg: "80%", xl: "70%" },
          display: "flex",
          flexDirection: "column",
          alignItems: "flex-start", // Alinea a la izquierda
          justifyContent: "center",
          color: "#fff",
          px: { xs: 2, sm: 6 },
          minHeight: "100vh",
        }}
        spacing={4}
      >
        <Typography
          level="h1"
          sx={{
            fontSize: { xs: "2.8rem", sm: "4rem", md: "5rem" },
            fontWeight: 800,
            mb: 2,
            lineHeight: 1.1,
            textAlign: "left",
            color: "rgba(255, 255, 255, 0.9)",
            letterSpacing: "-2px",
          }}
        >
          Tu obra no puede esperar. Nosotros tampoco.
        </Typography>
        <Typography
          level="body-lg"
          sx={{
            fontSize: { xs: "1.2rem", sm: "1.5rem" },
            mb: 3,
            maxWidth: 800,
            color: "rgba(255, 255, 255, 0.9)",
            textAlign: "left",
          }}
        >
          Te ofrecemos un servicio de alquiler ágil, con maquinaria siempre
          lista para trabajar. Operás mejor, rendís más, ganás tiempo.
        </Typography>
        <Button
          size="lg"
          sx={{
            zIndex: 5,

            bgcolor: "rgba(222, 63, 63, 0.9)",
            color: "rgba(255, 255, 255, 0.9)",
            fontWeight: 700,
            fontSize: "1.6rem",
            px: 5,
            py: 2,
            borderRadius: 8,
            boxShadow: "0 2px 8px rgba(0,0,0,0.15)",
            "&:hover": { bgcolor: "rgba(174, 55, 55, 0.9)" },
            mb: 2,
          }}
          component={RouterLink}
          to="/explore"
        >
          Alquilá ahora
        </Button>
      </Stack>
    </Sheet>
  );
}

export default Home;
