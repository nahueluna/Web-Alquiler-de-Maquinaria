import React, { useState } from "react";
import { useFormik } from "formik";
import axios from "axios";
import * as Yup from "yup";
import {
  Box,
  Button,
  Input,
  Typography,
  Divider,
  Snackbar,
  FormHelperText,
} from "@mui/joy";

const phoneRegex = /^[\d+\-\s]{8,17}$/;

const Profile = () => {
  const [userData, setUserData] = useState({
    nombre: "Valentino Amato Roberts",
    correo: "mrdjangus@gmail.com",
    telefono: "+54 1234-5678",
  });

  const [editMode, setEditMode] = useState(false);

  // Snackbar telefono
  const [showSaveSnackbar, setShowSaveSnackbar] = useState(false);
  // Snackbar contraseña
  const [showChangePasswordSnackbar, setShowChangePasswordSnackbar] = useState(false);
  // Snackbar error al enviar el mail
  const [errorSnackbar, setErrorSnackbar] = useState({ open: false, message: "" });

  const formik = useFormik({
    initialValues: {
      telefono: userData.telefono,
    },
    enableReinitialize: true,
    validateOnMount: true,
    validationSchema: Yup.object({
      telefono: Yup.string()
        .required("Teléfono es obligatorio")
        .matches(
          phoneRegex,
          "Teléfono inválido (solo números, +, -, espacios, 8-17 caracteres)"
        ),
    }),
    onSubmit: (values) => {
      setUserData((prev) => ({
        ...prev,
        telefono: values.telefono,
      }));
      setEditMode(false);
      setShowSaveSnackbar(true);
    },
  });

const handleChangePassword = async () => {
  try {
    const response = await axios.post(
      "http://localhost:8000/requestpswchange",
      { email: userData.correo },
      { withCredentials: true } // Para las cookies (?)
    );

    if (response.status === 200) {
      setShowChangePasswordSnackbar(true); // Si funca...
    }
  } catch (error) {
    if (error.response) {
      // ERROR 1
      setErrorSnackbar({ open: true, message: error.response.data.message });
    } else {
      // ERROR 2
      console.error("Error al conectar con el backend:", error);
      alert("Error de red. No se pudo contactar al servidor.");
      }
    }
  };

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
      <Typography level="h4" fontWeight="lg" mb={2}>
        Mis datos
      </Typography>

      <Divider sx={{ mb: 2 }} />

      {/* Nombre (no se puede cambiar) */}
      <Box sx={{ display: "flex", gap: 2, mb: 2, alignItems: "center" }}>
        <Typography level="body-sm" sx={{ minWidth: 100, fontWeight: "md" }}>
          Nombre:
        </Typography>
        <Typography>{userData.nombre}</Typography>
      </Box>

      {/* Correo (no se puede cambiar) */}
      <Box sx={{ display: "flex", gap: 2, mb: 2, alignItems: "center" }}>
        <Typography level="body-sm" sx={{ minWidth: 100, fontWeight: "md" }}>
          Correo:
        </Typography>
        <Typography>{userData.correo}</Typography>
      </Box>

      {/* Teléfono (editable) */}
      <Box sx={{ display: "flex", gap: 2, mb: 3, alignItems: "center" }}>
        <Typography level="body-sm" sx={{ minWidth: 100, fontWeight: "md" }}>
          Teléfono:
        </Typography>
        {editMode ? (
          <form onSubmit={formik.handleSubmit} style={{ flex: 1 }}>
            <Input
              name="telefono"
              value={formik.values.telefono}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.telefono && Boolean(formik.errors.telefono)}
              autoFocus
              sx={{ width: "100%" }}
            />
            {formik.touched.telefono && formik.errors.telefono && (
              <FormHelperText sx={{ color: "error.500" }}>
                {formik.errors.telefono}
              </FormHelperText>
            )}
          </form>
        ) : (
          <Typography>{userData.telefono}</Typography>
        )}
      </Box>

      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <Button variant="outlined" color="neutral" onClick={handleChangePassword}>
          Cambiar contraseña
        </Button>

        {editMode ? (
          <Button
            color="primary"
            onClick={formik.handleSubmit}
            disabled={formik.touched.telefono && Boolean(formik.errors.telefono)}
          >
            Guardar cambios
          </Button>
        ) : (
          <Button variant="outlined" color="neutral" onClick={() => setEditMode(true)}>
            Editar
          </Button>
        )}
      </Box>

      {/* Snackbar para guardar teléfono */}
      <Snackbar
        open={showSaveSnackbar}
        onClose={() => setShowSaveSnackbar(false)}
        autoHideDuration={3000}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
        variant="soft"
        color="success"
      >
        Teléfono guardado correctamente.
      </Snackbar>

      {/* Snackbar para cambio de contraseña */}
      <Snackbar
        open={showChangePasswordSnackbar}
        onClose={() => setShowChangePasswordSnackbar(false)}
        autoHideDuration={3000}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
        variant="soft"
        color="success"
      >
        Se le ha enviado un correo para cambiar su contraseña.
      </Snackbar>
      <Snackbar
        open={errorSnackbar.open}
        onClose={() => setErrorSnackbar({ ...errorSnackbar, open: false })}
        autoHideDuration={4000}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
        variant="soft"
        color="danger"
      >
        {errorSnackbar.message}
      </Snackbar>
    </Box>
  );
};

export default Profile;