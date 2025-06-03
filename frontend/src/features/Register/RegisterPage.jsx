import {
  default as ErrorOutline,
  default as ErrorOutlineIcon,
} from "@mui/icons-material/ErrorOutline";
import MailIcon from "@mui/icons-material/Mail";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";
import {
  Button,
  Checkbox,
  Divider,
  FormControl,
  FormHelperText,
  Input,
  Link,
  Sheet,
  Snackbar,
  Stack,
  Typography,
} from "@mui/joy";
import { useFormik } from "formik";
import React, { useState } from "react";
import { Link as RouterLink } from "react-router-dom";

import { useNavigate } from "react-router-dom";
import * as yup from "yup";
import useAuth from "../utils/useAuth";

const today = new Date();
const yyyy = today.getFullYear();
const mm = String(today.getMonth() + 1).padStart(2, "0");
const dd = String(today.getDate()).padStart(2, "0");
const todayStr = `${yyyy}-${mm}-${dd}`;

const validationSchema = yup.object({
  nombre: yup
    .string()
    .max(20, "El nombre no puede tener más de 20 caracteres")
    .matches(
      /^[A-Za-zÁÉÍÓÚáéíóúÑñ\s]+$/,
      "El nombre solo puede contener letras y espacios"
    )
    .required("El nombre es obligatorio"),
  apellido: yup
    .string()
    .max(20, "El apellido no puede tener más de 20 caracteres")
    .matches(
      /^[A-Za-zÁÉÍÓÚáéíóúÑñ\s]+$/,
      "El apellido solo puede contener letras y espacios"
    )
    .required("El apellido es obligatorio"),
  dni: yup
    .string()
    .matches(/^\d+$/, "El DNI solo debe contener números")
    .required("El DNI es obligatorio"),
  fecha: yup
    .date()
    .test("mayor-18", "Debes ser mayor de 18 años", (value) => {
      if (!value) return false;
      const diff = today.getFullYear() - value.getFullYear();
      if (diff > 18) return true;
      if (diff === 18) {
        if (
          today.getMonth() > value.getMonth() ||
          (today.getMonth() === value.getMonth() &&
            today.getDate() >= value.getDate())
        ) {
          return true;
        }
      }
      return false;
    })
    .required("La fecha es obligatoria"),
  email: yup
    .string()
    .matches(
      /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
      "Ingrese un correo electronico válido"
    )
    .required("El correo es obligatorio"),
  telefono: yup.string(), // Opcional
  terminos: yup
    .boolean()
    .oneOf([true], "Debe aceptar los términos y condiciones"),
});

const RegisterPage = () => {
  const [loading, setLoading] = useState(false);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const { post } = useAuth();
  const nav = useNavigate();

  const formik = useFormik({
    initialValues: {
      nombre: "",
      apellido: "",
      dni: "",
      fecha: "",
      email: "",
      telefono: "",
      terminos: false,
    },
    validationSchema,
    onSubmit: async (values) => {
      setLoading(true);
      const {
        nombre: name,
        apellido: surname,
        email,
        dni: id_card,
        fecha,
        telefono: phone,
      } = values;

      try {
        const { data } = await post("/signup", {
          name,
          surname,
          email,
          id_card,
          birth_date: fecha.split("-").reverse().join("-"),
          phone,
        });
        setStatus({ isError: false, message: "Usuario creado exitosamente." });
        setOpenSnack(true);
        setTimeout(() => {
          nav("/login");
        }, 2000);
      } catch (error) {
        console.error(error);
        let errorMsg;
        switch (error.response?.status) {
          case 409:
            errorMsg = "Ya existe un usuario con esa informacion";
            break;
          case 403:
            errorMsg = "El usuario es menor de edad";
            break;
          default:
            errorMsg = "Error al crear el usuario. Intente nuevamente.";
        }
        setStatus({
          isError: true,
          message: errorMsg,
        });
        setOpenSnack(true);
      } finally {
        setLoading(false);
      }
    },
  });

  // Helper para mostrar el icono de error y el tooltip
  const errorIcon = (touched, error) =>
    touched && error ? (
      <ErrorOutline color="danger" fontSize="small" titleAccess={error} />
    ) : (
      <span style={{ width: 21, display: "inline-block" }} />
    );

  return (
    <>
      <Snackbar
        variant="soft"
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
        autoHideDuration={3000}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
        startDecorator={
          status.isError ? (
            <ErrorOutlineIcon />
          ) : (
            <PlaylistAddCheckCircleRoundedIcon />
          )
        }
      >
        {status.message}
      </Snackbar>
      <Sheet
        variant="outlined"
        sx={{
          p: 4,
          borderRadius: "md",
          width: 600,
          mx: "auto",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <Typography level="h2" mb={2}>
          Crea una cuenta
        </Typography>
        <Divider />
        <form onSubmit={formik.handleSubmit}>
          <Stack spacing={1.5} sx={{ pt: 2 }}>
            <Stack
              direction="row"
              spacing={1}
              sx={{ justifyContent: "center", alignItems: "center" }}
            >
              <FormControl
                error={formik.touched.nombre && Boolean(formik.errors.nombre)}
              >
                <Input
                  placeholder="Nombre"
                  name="nombre"
                  value={formik.values.nombre}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  endDecorator={errorIcon(
                    formik.touched.nombre,
                    formik.errors.nombre
                  )}
                  title={
                    formik.touched.nombre && formik.errors.nombre
                      ? formik.errors.nombre
                      : ""
                  }
                />
              </FormControl>
              <FormControl
                error={
                  formik.touched.apellido && Boolean(formik.errors.apellido)
                }
              >
                <Input
                  placeholder="Apellido"
                  name="apellido"
                  value={formik.values.apellido}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  endDecorator={errorIcon(
                    formik.touched.apellido,
                    formik.errors.apellido
                  )}
                  title={
                    formik.touched.apellido && formik.errors.apellido
                      ? formik.errors.apellido
                      : ""
                  }
                />
              </FormControl>
            </Stack>
            <Stack
              direction="row"
              spacing={1}
              sx={{ justifyContent: "space-between", alignItems: "center" }}
            >
              <FormControl
                error={formik.touched.dni && Boolean(formik.errors.dni)}
              >
                <Input
                  placeholder="DNI"
                  name="dni"
                  value={formik.values.dni}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  endDecorator={errorIcon(
                    formik.touched.dni,
                    formik.errors.dni
                  )}
                  title={
                    formik.touched.dni && formik.errors.dni
                      ? formik.errors.dni
                      : ""
                  }
                />
              </FormControl>
              <FormControl
                sx={{ flex: 1 }}
                error={formik.touched.fecha && Boolean(formik.errors.fecha)}
              >
                <Input
                  type="date"
                  name="fecha"
                  value={formik.values.fecha}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  fullWidth
                  slotProps={{
                    input: {
                      min: "1900-01-01",
                      max: todayStr.toString(),
                    },
                  }}
                  endDecorator={errorIcon(
                    formik.touched.fecha,
                    formik.errors.fecha
                  )}
                  title={
                    formik.touched.fecha && formik.errors.fecha
                      ? formik.errors.fecha
                      : ""
                  }
                />
              </FormControl>
            </Stack>
            <FormControl
              error={formik.touched.email && Boolean(formik.errors.email)}
            >
              <Input
                startDecorator={<MailIcon />}
                type="email"
                placeholder="Correo electronico"
                name="email"
                value={formik.values.email}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
                endDecorator={errorIcon(
                  formik.touched.email,
                  formik.errors.email
                )}
                title={
                  formik.touched.email && formik.errors.email
                    ? formik.errors.email
                    : ""
                }
              />
            </FormControl>
            <FormControl>
              <Input
                type="tel"
                placeholder="Numero de celular (opcional)"
                name="telefono"
                value={formik.values.telefono}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
              />
            </FormControl>
            <FormControl
              size="sm"
              sx={{ width: 400, alignSelf: "flex-start" }}
              error={formik.touched.terminos && Boolean(formik.errors.terminos)}
            >
              <Checkbox
                name="terminos"
                checked={formik.values.terminos}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
                label={
                  <React.Fragment>
                    He leido y acepto los terminos y condiciones.
                  </React.Fragment>
                }
              />
              {formik.touched.terminos && formik.errors.terminos && (
                <FormHelperText>{formik.errors.terminos}</FormHelperText>
              )}
              <FormHelperText>
                <Typography level="body-sm">
                  Lea nuestros{" "}
                  <Link component={RouterLink} to={"/terms"} level="body-sm">
                    terminos y condiciones
                  </Link>
                  .
                </Typography>
              </FormHelperText>
            </FormControl>
            <Divider />
            <Stack spacing={2} sx={{ alignItems: "center" }}>
              <Button
                color="danger"
                size="lg"
                sx={{ width: "50%" }}
                type="submit"
                loading={loading}
                disabled={!(formik.isValid && formik.dirty)}
              >
                Registrarse
              </Button>
              <Link component={RouterLink} to={"/login"} level="body-sm">
                Ya tengo una cuenta
              </Link>
            </Stack>
          </Stack>
        </form>
      </Sheet>
    </>
  );
};

export default RegisterPage;
