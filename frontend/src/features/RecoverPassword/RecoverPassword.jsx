import React, { useEffect, useState } from "react";
import {
  Box,
  Typography,
  Input,
  Button,
  FormControl,
  FormLabel,
  FormHelperText,
} from "@mui/joy";
import { useFormik } from "formik";
import * as yup from "yup";

const validationSchema = yup.object({
  email: yup
    .string()
    .email("Ingrese un correo electrónico válido")
    .required("El correo es obligatorio"),
});

function RecoverPassword() {
  const [mensajeEnviado, setMensajeEnviado] = useState(false);
  const [loading, setLoading] = useState(false);

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
        await fetch("http://localhost:3000/api/password-reset", { // Como no hay backend todavia, dejamos un fetch que no hace nada
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ email: values.email }),
        });
      } catch (error) {
        // No hacemos un carajo porque no hay que chequear si existe el mail, VLLC.
      } finally {
        setLoading(false);
        setMensajeEnviado(true);
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
        borderColor: "neutral.outlinedBorder"
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
            <FormControl sx={{ mb: 4 }} error={formik.touched.email && Boolean(formik.errors.email)}>
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

                        <Box sx={{ display: "flex", justifyContent: "center"}}>
              <Button
                type="submit"
                sx={{ mt: 1, fontSize: "1.1rem", py: 1.5, width: "fit-content" }}
                disabled={loading}
              >
                {loading ? "Enviando..." : "Enviar"}
              </Button>
            </Box>
          </form>
        )}
      </Box>
  );
}

export default RecoverPassword; // teso puto