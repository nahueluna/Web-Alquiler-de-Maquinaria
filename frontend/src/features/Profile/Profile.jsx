import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Box,
  Button,
  Input,
  Typography,
  Sheet,
  Divider,
} from "@mui/joy";

const Profile = () => {
  const navigate = useNavigate();

  const [userData, setUserData] = useState({
    nombre: "Valentino Amato Roberts",
    correo: "elmisilpato@gmail.com",
    telefono: "+54 1234-5678",
  });

  const [editMode, setEditMode] = useState(false);
  const [formData, setFormData] = useState(userData);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSave = () => {
    setUserData(formData);
    setEditMode(false);
  };

  const handleChangePassword = () => {
    navigate("/recover-password");
  };

  return (
    <Sheet
      sx={{
        maxWidth: 600,
        margin: "auto",
        marginTop: 4,
        padding: 4,
        borderRadius: "lg",
        boxShadow: "sm",
        backgroundColor: "background.surface",
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
          <Input
            name="telefono"
            value={formData.telefono}
            onChange={handleChange}
            sx={{ flex: 1 }}
          />
        ) : (
          <Typography>{userData.telefono}</Typography>
        )}
      </Box>

      <Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <Button variant="outlined" color="neutral" onClick={handleChangePassword}>
          Cambiar contraseña
        </Button>

        {editMode ? (
          <Button color="primary" onClick={handleSave}>
            Guardar cambios
          </Button>
        ) : (
          <Button variant="outlined" color="neutral" onClick={() => setEditMode(true)}>
            Editar
          </Button>
        )}
      </Box>
    </Sheet>
  );
};

export default Profile;
