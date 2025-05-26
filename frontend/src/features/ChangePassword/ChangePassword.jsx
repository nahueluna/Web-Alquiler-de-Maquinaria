import React, { useState } from "react";
import {
  Box,
  Button,
  Input,
  Typography,
  Sheet,
  Divider,
  Alert,
} from "@mui/joy";
import { useFormik } from "formik";
import * as Yup from "yup";

const ChangePassword = () => {
  const [success, setSuccess] = useState(false);

  const formik = useFormik({
    initialValues: {
      password: "",
      confirmPassword: "",
    },
    validationSchema: Yup.object({
      password: Yup.string()
        .min(8, "Mínimo 8 caracteres")
        .required("Requerido"),
      confirmPassword: Yup.string()
        .oneOf([Yup.ref("password"), null], "Las contraseñas no coinciden")
        .required("Requerido"),
    }),
    onSubmit: (values) => { // LLAMAR AL BACKEND ACÁ
      console.log("Nueva contraseña:", values.password);
      setSuccess(true);
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
      <Typography level="h3" fontWeight="lg" mb={2}>
        Cambiar contraseña
      </Typography>

      <Divider sx={{ mb: 2 }} />

      {success ? (
        <Alert color="success" variant="soft">
          La contraseña ha sido cambiada con éxito.
        </Alert>
      ) : (
        <form onSubmit={formik.handleSubmit}>
          <Box sx={{ mb: 2 }}>
            <Input
              type="password"
              name="password"
              placeholder="Nueva contraseña"
              value={formik.values.password}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
            />
            {formik.touched.password && formik.errors.password && (
              <Typography level="body-sm" color="danger">
                {formik.errors.password}
              </Typography>
            )}
          </Box>

          <Box sx={{ mb: 2 }}>
            <Input
              type="password"
              name="confirmPassword"
              placeholder="Repetir contraseña"
              value={formik.values.confirmPassword}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
            />
            {formik.touched.confirmPassword && formik.errors.confirmPassword && (
              <Typography level="body-sm" color="danger">
                {formik.errors.confirmPassword}
              </Typography>
            )}
          </Box>

          <Button type="submit" color="primary" fullWidth>
            Cambiar contraseña
          </Button>
        </form>
      )}
    </Box>
  );
};

export default ChangePassword;