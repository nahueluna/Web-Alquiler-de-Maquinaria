import {
  Alert,
  Box,
  Button,
  FormControl,
  FormHelperText,
  FormLabel,
  Input,
  Typography,
} from "@mui/joy";
import { useFormik } from "formik";
import { useEffect, useState } from "react";
import * as yup from "yup";
import useAuth from "../utils/useAuth";

const validationSchema = yup.object({
  email: yup
    .string()
    .email("Correo electrónico inválido")
    .test("is-valid-domain", "Correo electrónico inválido", (value) => {
      if (!value) return false;
      return /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/.test(value);
    })
    .required("El correo es obligatorio"),
});

function RecoverPassword() {
  const [mensajeEnviado, setMensajeEnviado] = useState(false);
  const [loading, setLoading] = useState(false);
  const [errorSnackbar, setErrorSnackbar] = useState({
    open: false,
    message: "",
  });
  const { post } = useAuth();

  useEffect(() => {
    document.title = "Recuperar contraseña";
  }, []);

  const formik = useFormik({
    initialValues: {
      email: "",
    },
    validationSchema,
    onSubmit: async (values) => {
      setLoading(true);
      try {
        const response = await post("/requestpswchange", {
          email: values.email,
        });
        setMensajeEnviado(true);
      } catch (error) {
        if (error.response) {
          if (error.response.status === 400) {
            setMensajeEnviado(true);
          } else {
            setErrorSnackbar({
              open: true,
              message: error.response.data.message || "Error en el servidor.",
            });
          }
        } else {
          setErrorSnackbar({
            open: true,
            message: "Error. No se pudo contactar al servidor.",
          });
        }
      } finally {
        setLoading(false);
      }
    },
  });

  return (
    <Box
      sx={{
        maxWidth: 400,
        mx: "auto",
        mt: 4,
        mb: 4,
        p: 4,
        borderRadius: "lg",
        boxShadow: "sm",
        backgroundColor: "background.surface",
        border: "1px solid",
        borderColor: "neutral.outlinedBorder",
      }}
    >
      {!mensajeEnviado && (
        <Typography
          level="h4"
          component="h2"
          gutterBottom
          sx={{ mb: 4, fontSize: "2rem" }}
        >
          Recuperar contraseña
        </Typography>
      )}

      {mensajeEnviado ? (
        <Typography sx={{ fontSize: "1.1rem" }}>
          Si el correo ingresado está registrado, te enviaremos un email para
          cambiar tu contraseña.
        </Typography>
      ) : (
        <form onSubmit={formik.handleSubmit}>
          <FormControl
            sx={{ mb: 4 }}
            error={formik.touched.email && Boolean(formik.errors.email)}
          >
            <FormLabel sx={{ fontSize: "1.1rem" }}>Email</FormLabel>
            <Input
              name="email"
              type="email"
              placeholder="tucorreo@ejemplo.com"
              value={formik.values.email}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              disabled={loading}
              sx={{ fontSize: "1.1rem", py: 1 }}
            />
            {formik.touched.email && formik.errors.email && (
              <FormHelperText>{formik.errors.email}</FormHelperText>
            )}
          </FormControl>

          <Box sx={{ display: "flex", justifyContent: "center" }}>
            <Button
              type="submit"
              color="danger"
              sx={{ mt: 1, fontSize: "1.1rem", py: 1.5, width: "fit-content" }}
              disabled={loading}
            >
              {loading ? "Enviando..." : "Enviar"}
            </Button>
          </Box>
        </form>
      )}
      {errorSnackbar.open && (
        <Alert
          color="danger"
          variant="soft"
          sx={{ mt: 2 }}
          onClose={() => setErrorSnackbar({ open: false, message: "" })}
        >
          {errorSnackbar.message}
        </Alert>
      )}
    </Box>
  );
}

export default RecoverPassword;
