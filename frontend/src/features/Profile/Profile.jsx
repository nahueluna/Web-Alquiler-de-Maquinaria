import {
  Box,
  Button,
  Divider,
  FormHelperText,
  Input,
  Snackbar,
  Typography,
} from "@mui/joy";
import { useFormik } from "formik";
import { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import * as Yup from "yup";
import UserContext from "../../context/UserContext";
import useAuth from "../utils/useAuth";

const phoneRegex = /^[\d+\-\s]{8,17}$/;

const Profile = () => {
  const { user } = useContext(UserContext);
  const { post } = useAuth();
  const navigate = useNavigate();
  const [loadingPasswordChange, setLoadingPasswordChange] = useState(false);
  const [userData, setUserData] = useState({
    nombre: "",
    correo: "",
    telefono: "",
    dni: "",
    nacimiento: "",
    esAdmin: false,
  });

  useEffect(() => {
    if (user) {
      setUserData({
        nombre: user.pub_user?.name || "",
        correo: user.pub_user?.email || "",
        telefono: user.user_info?.phone || "",
        dni: user.user_info?.id_card || "",
        nacimiento: user.user_info?.birthdate || "",
        rol: user?.pub_user?.role ?? 2,
      });
      console.log("USER COMPLETO", user);
    }
  }, [user]);

  const [editMode, setEditMode] = useState(false);

  // Snackbar telefono
  const [showSaveSnackbar, setShowSaveSnackbar] = useState(false);
  // Snackbar contraseña
  const [showChangePasswordSnackbar, setShowChangePasswordSnackbar] =
    useState(false);
  // Snackbar error al enviar el mail
  const [errorSnackbar, setErrorSnackbar] = useState({
    open: false,
    message: "",
  });

  const formik = useFormik({
    initialValues: {
      telefono: userData.telefono,
    },
    enableReinitialize: true,
    validateOnMount: true,
    validationSchema: Yup.object({
      telefono: Yup.string().test(
        "telefono-valido",
        "Teléfono inválido (solo números, +, -, espacios, 8-17 caracteres)",
        (value) => {
          if (!value) return true;
          return phoneRegex.test(value);
        }
      ),
    }),
    onSubmit: async (values, { setSubmitting }) => {
      try {
        console.log("Enviando teléfono:", values.telefono);
        const response = await post("/changephone", {
          phone: values.telefono,
        });

        if (response.status === 200) {
          setUserData((prev) => ({
            ...prev,
            telefono: values.telefono,
          }));
          window.localStorage.setItem("userPhone", values.telefono);
          setEditMode(false);
          setShowSaveSnackbar(true);
        }
      } catch (error) {
        if (error.response) {
          switch (error.response.status) {
            case 401:
              setErrorSnackbar({
                open: true,
                message: "Token inválido. Por favor inicia sesión de nuevo.",
              });
              // Aquí podrías agregar lógica para forzar logout si quieres
              break;
            case 500:
              setErrorSnackbar({
                open: true,
                message: "Error interno del servidor. Intenta más tarde.",
              });
              break;
            default:
              setErrorSnackbar({
                open: true,
                message: error.response.data.message || "Error desconocido.",
              });
          }
        } else {
          setErrorSnackbar({
            open: true,
            message: "No se pudo conectar con el servidor.",
          });
        }
      } finally {
        setSubmitting(false);
      }
    },
  });

  const handleChangePassword = async () => {
    try {
      setLoadingPasswordChange(true);
      const response = await post("/requestpswchange", {
        email: userData.correo,
      });

      if (response.status === 200) {
        setShowChangePasswordSnackbar(true); // Si funca...
        setLoadingPasswordChange(false);
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
      setLoadingPasswordChange(false);
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
      <Typography level="h3" fontWeight="lg" mb={2}>
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
      <Box sx={{ display: "flex", gap: 2, mb: 2, alignItems: "center" }}>
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
          <Typography>
            {userData.telefono ? userData.telefono : <i>No cargado</i>}
          </Typography>
        )}
      </Box>

      {/* DNI */}
      <Box sx={{ display: "flex", gap: 2, mb: 2, alignItems: "center" }}>
        <Typography level="body-sm" sx={{ minWidth: 100, fontWeight: "md" }}>
          DNI:
        </Typography>
        <Typography>
          {userData.dni ? userData.dni : <i>No cargado</i>}
        </Typography>
      </Box>

      {/* Fecha de nacimiento */}
      <Box sx={{ display: "flex", gap: 2, mb: 2, alignItems: "center" }}>
        <Typography level="body-sm" sx={{ minWidth: 100, fontWeight: "md" }}>
          Nacimiento:
        </Typography>
          <Typography>
            {userData.nacimiento ? (
              new Date(userData.nacimiento).toLocaleDateString("es-AR", {
                day: "2-digit",
                month: "2-digit",
                year: "numeric",
              })
            ) : (
              <i>No cargado</i>
            )}
          </Typography>
      </Box>

      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <Button
          variant="outlined"
          color="neutral"
          onClick={handleChangePassword}
          loading={loadingPasswordChange}
          disabled={loadingPasswordChange}
        >
          Cambiar contraseña
        </Button>

        {editMode ? (
          <Button
            color="primary"
            type="submit"
            onClick={formik.handleSubmit}
            disabled = {Boolean(formik.errors.telefono)}
          >
            Guardar cambios
          </Button>
        ) : (
          <Button
            variant="outlined"
            color="neutral"
            onClick={() => setEditMode(true)}
          >
            Editar teléfono
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
