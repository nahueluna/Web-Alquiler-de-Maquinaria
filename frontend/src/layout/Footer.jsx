import React from "react";
import { Link, Sheet, Typography } from "@mui/joy/";
import Logo from "../assets/LogoCompleto.png";

const Footer = () => {
  return (
    <Sheet
      sx={{
        padding: 2,
        backgroundColor: "#dfe1e3",
        display: "flex",
        justifyContent: "space-between",
        boxShadow: "0 -1px 20px rgba(0, 0, 0, 0.1)",
      }}
    >
      <div>
        <a href="/">
          <img width={"200px"} src={Logo} alt="" />
        </a>
      </div>
      <Sheet sx={{ backgroundColor: "transparent" }}>
        <Typography level="title-lg">Contacto</Typography>

        <Typography level="body-sm">
          <Link href="mailto:soporte@alquilador.com" variant="plain">
            soporte@alquilador.com
          </Link>
        </Typography>
        <Typography level="body-sm">
          <Link href="tel:+888888888" variant="plain">
            +888888888
          </Link>
        </Typography>
      </Sheet>
    </Sheet>
  );
};

export default Footer;
