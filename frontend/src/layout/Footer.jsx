import React from "react";
import { Link, Sheet, Typography, Box, Stack, Divider } from "@mui/joy";
import Logo from "../assets/LogoCompleto.png";

const Footer = () => {
  return (
    <Sheet
      sx={{
        padding: { xs: 2, md: 4 },
        backgroundColor: "#dfe1e3",
        display: "flex",
        flexDirection: "column",
        gap: 2,
        boxShadow: "0 -1px 20px rgba(0, 0, 0, 0.1)",
      }}
    >
      <Box
        sx={{
          display: "flex",
          flexDirection: "row",
          justifyContent: { xs: "flex-start", md: "space-around" },
          alignItems: { xs: "flex-start", md: "center" },
          gap: 4,
        }}
      >
        {/* Logo y descripción */}
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            alignItems: "flex-start",
            gap: 1,
          }}
        >
          <a href="/">
            <img
              src={Logo}
              alt="Bob el Alquilador"
              style={{
                width: "100%",
                maxWidth: 300,
                minWidth: 120,
                height: "auto",
                ...(window.innerWidth < 600 ? { maxWidth: 160 } : {}),
              }}
            />
          </a>
        </Box>

        {/* Navegación */}
        <Stack direction="column" spacing={1} sx={{ minWidth: 120 }}>
          <Link href="/" sx={{ color: "#222" }}>
            Inicio
          </Link>
          <Link href="/explore" sx={{ color: "#222" }}>
            Catálogo
          </Link>
          <Link sx={{ color: "#222" }}>Sobre Nosotros</Link>
          <Link sx={{ color: "#222" }}>Contacto</Link>
        </Stack>

        {/* Contacto y redes */}
        <Stack direction="column" spacing={1} sx={{ minWidth: 180 }}>
          <Typography level="title-sm" sx={{ mb: 0.5 }}>
            Contacto
          </Typography>
          <Link href="mailto:soporte@alquilador.com">
            soporte@alquilador.com
          </Link>
          <Link href="tel:+888888888">0221 482-3629</Link>
          <Box sx={{ mt: 1, display: "flex", gap: 1 }}>
            <Link
              href="https://facebook.com"
              target="_blank"
              aria-label="Facebook"
            >
              <img
                src="https://cdn.jsdelivr.net/gh/simple-icons/simple-icons/icons/facebook.svg"
                alt="Facebook"
                width={22}
                height={22}
                style={{ filter: "grayscale(1)" }}
              />
            </Link>
            <Link
              href="https://instagram.com"
              target="_blank"
              aria-label="Instagram"
            >
              <img
                src="https://cdn.jsdelivr.net/gh/simple-icons/simple-icons/icons/instagram.svg"
                alt="Instagram"
                width={22}
                height={22}
                style={{ filter: "grayscale(1)" }}
              />
            </Link>
            <Link
              href="https://wa.me/888888888"
              target="_blank"
              aria-label="WhatsApp"
            >
              <img
                src="https://cdn.jsdelivr.net/gh/simple-icons/simple-icons/icons/whatsapp.svg"
                alt="WhatsApp"
                width={22}
                height={22}
                style={{ filter: "grayscale(1)" }}
              />
            </Link>
          </Box>
        </Stack>
      </Box>

      <Divider flexItem sx={{ my: 2, display: { xs: "block", md: "block" } }} />
      <Typography
        level="body-xs"
        sx={{
          color: "#888",
          textAlign: "center",
          mt: 1,
          width: "100%",
        }}
      >
        © {new Date().getFullYear()} Bob el Alquilador. Todos los derechos
        reservados.
      </Typography>
    </Sheet>
  );
};

export default Footer;
