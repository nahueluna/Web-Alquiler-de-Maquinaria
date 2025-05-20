import ErrorOutline from "@mui/icons-material/ErrorOutline";
import MailIcon from "@mui/icons-material/Mail";
import {
  Button,
  Checkbox,
  Divider,
  FormControl,
  FormHelperText,
  Input,
  Link,
  Sheet,
  Stack,
  Typography,
} from "@mui/joy";
import { useFormik } from "formik";
import React from "react";
import { Link as RouterLink } from "react-router-dom";

import * as yup from "yup";

const today = new Date();
const yyyy = today.getFullYear();
const mm = String(today.getMonth() + 1).padStart(2, "0");
const dd = String(today.getDate()).padStart(2, "0");
const todayStr = `${yyyy}-${mm}-${dd}`;
console.log(todayStr);

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
      "Ingrese un correo electronico valido"
    )
    .required("El correo es obligatorio"),
  telefono: yup.string(), // Opcional
  terminos: yup
    .boolean()
    .oneOf([true], "Debe aceptar los términos y condiciones"),
});

const RegisterPage = () => {
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
    onSubmit: (values) => {
      alert(JSON.stringify(values, null, 2));
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
              error={formik.touched.apellido && Boolean(formik.errors.apellido)}
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
                endDecorator={errorIcon(formik.touched.dni, formik.errors.dni)}
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
              color="success"
              size="lg"
              sx={{ width: "50%" }}
              type="submit"
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
  );
};

export default RegisterPage;
