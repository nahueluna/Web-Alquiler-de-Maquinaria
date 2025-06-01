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
import axios from "axios";
import { useFormik } from "formik";
import { useContext, useState } from "react";
import * as Yup from "yup";
import UserContext from "../../context/UserContext";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

function AddEmployee({ setRegisterForm }) {
  const [openSnackbar, setOpenSnackbar] = useState(false);
  const { user } = useContext(UserContext);
  const accessToken = user?.access || null;
  const [errorSnackbar, setErrorSnackbar] = useState({
    open: false,
    message: "",
  });

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
          access: user?.access || "",
        };
        console.log("Payload enviado:", JSON.stringify(payload, null, 2));
        const response = await axios.post(
          `${BACKEND_URL}/registeremployee`,
          payload,
          {
            withCredentials: true,
          }
        );

        if (response.status === 200) {
          setOpenSnackbar(true);
          resetForm();
          setRegisterForm(false);
        }
      } catch (error) {
        console.error("Error axios:", error);
        let errorMsg = "Error desconocido.";
        if (error.response) {
          switch (error.response.status) {
            case 400:
              errorMsg = "Datos inválidos. Revisa el formulario.";
              break;
            case 401:
              errorMsg = "Token inválido. Por favor inicia sesión de nuevo.";
              break;
            case 403:
              errorMsg = "No tienes permisos para registrar empleados.";
              break;
            case 409:
              errorMsg = "El email ya está registrado.";
              break;
            case 500:
              errorMsg =
                "Error interno del servidor (posible DNI ya registrado).";
              break;
            default:
              errorMsg = error.response.data.message || errorMsg;
          }
        } else {
          errorMsg = "No se pudo conectar con el servidor.";
        }
        setErrorSnackbar({ open: true, message: errorMsg });
      } finally {
        setSubmitting(false);
      }
    },
  });

  const handleCloseSnackbar = (event, reason) => {
    if (reason === "clickaway") return;
    setOpenSnackbar(false);
  };

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

      <Snackbar
        open={openSnackbar}
        autoHideDuration={3000}
        onClose={handleCloseSnackbar}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
        sx={{
          backgroundColor: "transparent",
          boxShadow: "none",
          padding: 0,
        }}
      >
        <Alert color="success" variant="soft" onClose={handleCloseSnackbar}>
          El empleado ha sido registrado correctamente.
        </Alert>
      </Snackbar>
      <Snackbar
        open={errorSnackbar.open}
        onClose={() => setErrorSnackbar({ open: false, message: "" })}
        message={errorSnackbar.message}
        color={
          errorSnackbar.message.includes("correctamente") ? "success" : "danger"
        }
        variant="soft"
        autoHideDuration={3000}
        sx={{
          position: "fixed",
          bottom: 16,
          left: "70%",
          transform: "translateX(-50%)",
          zIndex: 9999,
        }}
      >
        {errorSnackbar.message}
      </Snackbar>
    </>
  );
}

export default AddEmployee;
