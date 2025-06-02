import {
  Alert,
  Box,
  Button,
  FormHelperText,
  FormLabel,
  Input,
  Snackbar,
  Typography,
} from "@mui/joy";
import Stack from "@mui/joy/Stack";
import { useFormik } from "formik";
import { useState } from "react";
import * as Yup from "yup";
import useAuth from "../utils/useAuth";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";

function AddEmployee({
  setRegisterForm,
  setOpenSnack,
  setStatus,
  handleEmployeeAdded,
}) {
  const { post } = useAuth();

  const formik = useFormik({
    initialValues: {
      email: "",
      nombre: "",
      apellido: "",
      fechaNacimiento: "",
      dni: "",
      telefono: "",
    },
    validationSchema: Yup.object({
      email: Yup.string()
        .matches(/^[^\s@]+@[^\s@]+\.[a-zA-Z]{2,}$/, "Email inválido")
        .required("Email es obligatorio"),

      nombre: Yup.string()
        .matches(
          /^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]{2,}$/,
          "Nombre solo puede contener letras y espacios"
        )
        .required("Nombre es obligatorio"),

      apellido: Yup.string()
        .matches(
          /^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]{2,}$/,
          "Apellido solo puede contener letras y espacios"
        )
        .required("Apellido es obligatorio"),

      fechaNacimiento: Yup.date()
        .required("Fecha de nacimiento es obligatoria")
        .typeError("Fecha inválida (formato YYYY-MM-DD)")
        .max(new Date(), "La fecha no puede ser futura")
        .test("mayor-edad", "Debe de ser mayor de 18 años", function (value) {
          if (!value) return false;
          const hoy = new Date();
          const fecha18 = new Date(
            hoy.getFullYear() - 18,
            hoy.getMonth(),
            hoy.getDate()
          );
          return value <= fecha18;
        }),

      dni: Yup.string()
        .matches(/^\d{7,8}$/, "DNI debe tener entre 7 y 8 números")
        .required("DNI es obligatorio"),

      telefono: Yup.string()
        .matches(/^\d{8,17}$/, "Teléfono debe tener entre 8 y 17 dígitos")
        .notRequired(),
    }),
    onSubmit: async (values, { setSubmitting, resetForm }) => {
      try {
        const fechaSplit = values.fechaNacimiento.split("-");
        const fechaFormateada = `${fechaSplit[2]}-${fechaSplit[1]}-${fechaSplit[0]}`;

        const payload = {
          email: values.email,
          name: values.nombre,
          surname: values.apellido,
          birthdate: fechaFormateada,
          id_card: values.dni,
          phone: values.telefono || null,
        };
        console.log("Payload enviado:", JSON.stringify(payload, null, 2));
        const response = await post("/registeremployee", payload);
        setStatus({
          isError: false,
          message: "Empleado registrado exitosamente.",
        });
        handleEmployeeAdded();
        setOpenSnack(true);
        resetForm();
        setRegisterForm(false);
      } catch (error) {
        console.error("Error axios:", error);
        let errorMsg = "Ocurrio un error con el servidor. Intentalo más tarde.";
        if (error.response) {
          switch (error.response.status) {
            case 400:
              errorMsg = "Datos inválidos. Revisa el formulario.";
              break;
            case 401:
              errorMsg = "Expiro tu sesion. Por favor inicia sesión de nuevo.";
              break;
            case 403:
              errorMsg = "No tenes permisos para registrar empleados.";
              break;
            case 409:
              errorMsg = "El email o DNI ingresados ya estan registrados.";
              break;
            case 500:
              errorMsg =
                "Error interno del servidor (posible DNI ya registrado).";
              break;
            default:
              errorMsg = errorMsg;
          }
        } else {
          errorMsg = "No se pudo conectar con el servidor.";
        }
        setStatus({ isError: true, message: errorMsg });
        setOpenSnack(true);
      } finally {
        setSubmitting(false);
      }
    },
  });

  const renderInput = (label, name, type = "text", required = false) => {
    const showError = formik.touched[name] && Boolean(formik.errors[name]);
    return (
      <Box>
        <FormLabel htmlFor={name}>
          {label} {required && "*"}
        </FormLabel>
        <Input
          id={name}
          name={name}
          type={type}
          value={formik.values[name]}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={showError}
        />
        {showError && (
          <FormHelperText color="danger">{formik.errors[name]}</FormHelperText>
        )}
      </Box>
    );
  };

  return (
    <>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        sx={{
          maxWidth: 600,
          mx: "auto",
          mt: 4,
          mb: 4,
          display: "flex",
          flexDirection: "column",
          gap: 2,
          p: 3,
          borderRadius: "sm",
          backgroundColor: "background.surface",
          border: "1px solid",
          borderColor: "neutral.outlinedBorder",
        }}
        noValidate
      >
        <Typography level="h3" component="h3" textAlign="center" mb={1}>
          Registrar empleado
        </Typography>

        {renderInput("Email", "email", "email", true, { size: "sm" })}
        {renderInput("Nombre", "nombre", "text", true, { size: "sm" })}
        {renderInput("Apellido", "apellido", "text", true, { size: "sm" })}
        {renderInput("Fecha de nacimiento", "fechaNacimiento", "date", true, {
          size: "sm",
        })}
        {renderInput("DNI", "dni", "text", true, { size: "sm" })}
        {renderInput("Teléfono (opcional)", "telefono", "text", false, {
          size: "sm",
        })}

        <Stack direction="row" justifyContent="flex-start" spacing={1}>
          <Button
            type="submit"
            size="sm"
            color="danger"
            variant="solid"
            disabled={formik.isSubmitting}
          >
            Registrar empleado
          </Button>
          <Button
            size="sm"
            variant="plain"
            onClick={() => {
              formik.resetForm();
              setRegisterForm(false);
            }}
          >
            Cancelar
          </Button>
        </Stack>
      </Box>
    </>
  );
}

export default AddEmployee;
