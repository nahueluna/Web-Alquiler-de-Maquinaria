import { Button, Input, Link, Sheet } from "@mui/joy";
import SoloLogo from "../assets/SoloLogo.png";

const Navbar = () => {
  return (
    <Sheet
      sx={{
        padding: 2,
        backgroundColor: "white",
        display: "flex",
        justifyContent: "space-between",
        height: "50px",
        alignItems: "center",
        boxShadow: "0px 2px 4px rgba(0, 0, 0, 0.1)",
        position: "fixed",
        top: 0,
        left: 0,
        right: 0,
        zIndex: 1000,
      }}
    >
      <Link href="/">
        <img width={"50px"} src={SoloLogo} alt="" />
      </Link>
      <Input variant="outlined" placeholder="Buscar maquinaria..."></Input>
      <Button
        color="danger"
        onClick={function () {}}
        size="sm"
        variant="outlined"
      >
        Registrarse
      </Button>
      <Button
        color="danger"
        onClick={function () {}}
        size="sm"
        variant="outlined"
      >
        Iniciar sesion
      </Button>
    </Sheet>
  );
};

export default Navbar;
