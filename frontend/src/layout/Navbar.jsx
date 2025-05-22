import SearchIcon from "@mui/icons-material/Search";
import { Button, FormControl, IconButton, Input, Link, Sheet } from "@mui/joy";
import { useNavigate } from "react-router-dom";
import SoloLogo from "../assets/SoloLogo.png";

const Navbar = () => {
  const navigate = useNavigate();
  return (
    <div>
      <Sheet
        sx={{
          padding: 2,
          backgroundColor: "white",
          display: "flex",
          justifyContent: "space-between",
          height: "50px",
          alignItems: "center",
          boxShadow: "0px 1px 0px rgba(0, 0, 0, 0.1)",
          position: "sticky",
          top: 0,
          left: 0,
          right: 0,
          zIndex: 1000,
        }}
      >
        <Link href="/">
          <img width={"50px"} src={SoloLogo} alt="" />
        </Link>
        <FormControl sx={{ width: "100%", maxWidth: 400 }}>
          <Input
            type="text"
            variant="outlined"
            placeholder="Buscar maquinaria..."
            endDecorator={
              <IconButton
                color="neutral"
                variant="soft"
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
                <SearchIcon />
              </IconButton>
            }
          ></Input>
        </FormControl>
        <Button
          color="danger"
          onClick={() => navigate("/explore")}
          size="sm"
          variant="solid"
        >
          Catalogo
        </Button>
        <Button
          color="danger"
          onClick={() => navigate("/register")}
          size="sm"
          variant="outlined"
        >
          Registrarse
        </Button>
        <Button
          color="danger"
          onClick={() => navigate("/login")}
          size="sm"
          variant="outlined"
        >
          Iniciar sesion
        </Button>
      </Sheet>
    </div>
  );
};

export default Navbar;
