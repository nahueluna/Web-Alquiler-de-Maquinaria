import { Box, Button, Input, Link, Sheet } from "@mui/joy";
import { useFormik } from "formik";
import { useContext, useEffect, useState } from "react";
import { Link as RouterLink, useLocation, useNavigate } from "react-router-dom";
import * as Yup from "yup";
import SoloLogo from "../../assets/SoloLogo.png";
import UserContext from "../../context/UserContext";
import Nav from "./Nav";
import UserDrop from "./UserDrop";

const Navbar = () => {
  const [hideNav, setHideNav] = useState(false);
  const { user } = useContext(UserContext);
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    function scrollNavbar() {
      const { scrollY: scroll } = window;

      setHideNav(scroll > 100);
    }

    window.addEventListener("scroll", scrollNavbar);

    return () => window.removeEventListener("scroll", scrollNavbar);
    d;
  }, []);

  const params = new URLSearchParams(location.search);
  const initialSearch = params.get("search") || "";

  // Configuración de Formik y Yup para el buscador
  const formik = useFormik({
    initialValues: { search: initialSearch },
    validationSchema: Yup.string({
      search: Yup.string().required("Por favor ingresa un término de búsqueda"),
    }),
    onSubmit: (values) => {
      const termino = values.search.trim();
      const params = new URLSearchParams(location.search);

      if (termino) {
        params.set("search", termino);
      } else {
        params.delete("search");
      }
      navigate(`/explore?${params.toString()}`);
    },
    enableReinitialize: true,
  });

  return (
    <>
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
            <img width={"50px"} src={SoloLogo} alt="Logo" />
          </Link>

          {/* Formulario de búsqueda */}
          <form
            onSubmit={formik.handleSubmit}
            style={{ display: "flex", gap: 8 }}
          >
            <Input
              id="search"
              name="search"
              variant="outlined"
              placeholder="Buscar maquinaria..."
              size="sm"
              sx={{
                width: {
                  xs: "50%",
                  sm: "90%",
                },
              }}
              value={formik.values.search}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              //error={formik.touched.search && Boolean(formik.errors.search)}
              aria-describedby="search-error-text"
            />
            <Button type="submit" color="danger" variant="solid" size="sm">
              Buscar
            </Button>
          </form>
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
          {user !== null ? (
            <UserDrop />
          ) : (
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
          )}
        </Sheet>
      </Box>
    </>
  );
};

export default Navbar;
