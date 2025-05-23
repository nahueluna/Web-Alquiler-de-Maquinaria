import { Box, Button, Input, Link, Sheet } from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import SoloLogo from "../assets/SoloLogo.png";
import { useEffect, useState } from "react";
import Nav from "./Nav";

const Navbar = () => {
  const [hideNav, setHideNav] = useState(false);

  useEffect(() => {
    function scrollNavbar() {
      const { scrollY: scroll } = window;
      console.log(scroll);

      setHideNav(scroll > 100);
    }

    window.addEventListener("scroll", scrollNavbar);

    return () => window.removeEventListener("scroll", scrollNavbar);
  }, []);

  return (
    <Box
      sx={{
        position: "sticky",
        top: 0,
        left: 0,
        right: 0,
        zIndex: 999,
      }}
    >
      {/* Top */}
      <Sheet
        sx={{
          padding: 2,
          paddingY: hideNav ? 0 : "",
          height: hideNav ? 0 : "50px",
          // height: "50px",
          backgroundColor: "white",
          display: "flex",
          justifyContent: "space-between",
          transition: "all 250ms ease-out",
          alignItems: "center",
        }}
      >
        <Link component={RouterLink} to={"/"}>
          <img width={"50px"} src={SoloLogo} alt="" />
        </Link>
        <Input
          sx={{
            width: {
              xs: "50%",
              sm: "30%",
            },
          }}
          variant="outlined"
          placeholder="Buscar maquinaria..."
        ></Input>
      </Sheet>
      {/* Bottom */}
      <Sheet
        sx={{
          padding: 2,
          backgroundColor: "white",
          display: "flex",
          justifyContent: "space-between",
          height: "50px",
          alignItems: "center",
          boxShadow: "0px 3px 4px rgba(0, 0, 0, 0.1)",
          zIndex: 1000,
        }}
      >
        <Box>
          <Nav />
        </Box>
        <Box
          sx={{
            display: "flex",
            gap: 2,
          }}
        >
          <Link component={RouterLink} to="/register" underline="none">
            <Button
              color="danger"
              onClick={function () {}}
              size="sm"
              variant="solid"
            >
              Registrarse
            </Button>
          </Link>

          <Link component={RouterLink} to="/login" underline="none">
            <Button
              color="danger"
              onClick={function () {}}
              size="sm"
              variant="outlined"
            >
              Iniciar sesion
            </Button>
          </Link>
        </Box>
      </Sheet>
    </Box>
  );
};

export default Navbar;
