import React, { useState, useEffect } from "react";
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
import axios from "axios";
import { useParams, useNavigate } from "react-router-dom";


const ChangePassword = () => {
  const { code } = useParams();
  const navigate = useNavigate();

  const [success, setSuccess] = useState(false);
  const [codeValid, setCodeValid] = useState(false);
  const [checkingCode, setCheckingCode] = useState(true);
  const [errorMsg, setErrorMsg] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!code) {
      navigate("/home", { replace: true });
      return;
    }

    const checkCode = async () => {
      try {
        const { data } = await axios.post(
          "http://localhost:8000/checkchangepswcode",
          { code },
          { withCredentials: true }
        );
        setCodeValid(data.valid);
        setErrorMsg(data.valid ? null : "Código inválido o expirado.");
      } catch (error) {
        setCodeValid(false);
        setErrorMsg("Error al verificar el código.");
      } finally {
        setCheckingCode(false);
      }
    };

    checkCode();
  }, [code, navigate]);

  // SI ES INVÁLIDO
  useEffect(() => {
    if (!checkingCode && !codeValid) {
      navigate("/home", { replace: true });
    }
  }, [checkingCode, codeValid, navigate]);

  const formik = useFormik({
    initialValues: {
      password: "",
      confirmPassword: "",
    },
    validationSchema: Yup.object({
      password: Yup.string().min(8, "Mínimo 8 caracteres").required("Requerido"),
      confirmPassword: Yup.string()
        .oneOf([Yup.ref("password"), null], "Las contraseñas no coinciden")
        .required("Requerido"),
    }),
    onSubmit: async (values) => {
      try {
        setLoading(true);
        const { data } = await axios.post(
          "http://localhost:8000/changepsw",
          {
            code,
            new_password: values.password,
          },
          { withCredentials: true }
        );
        setSuccess(true);
        setErrorMsg(null);
      } catch (error) {
        const mensaje = error.response?.data?.mensaje || "Error";
        setErrorMsg(mensaje);
        setSuccess(false);
      } finally {
        setLoading(false);
      }
    },
  });

  // PARA QUE NO PARPADEE
  if (checkingCode) return null;
  if (!codeValid) return null;

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

          {errorMsg && (
            <Alert color="danger" variant="soft" sx={{ mb: 2 }}>
              {errorMsg}
            </Alert>
          )}

          <Button type="submit" color="primary" fullWidth>
            Cambiar contraseña
          </Button>
        </form>
      )}
    </Box>
  );
};

export default ChangePassword;