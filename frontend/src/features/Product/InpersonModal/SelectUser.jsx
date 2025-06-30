import React from "react";
import Input from "@mui/joy/Input";
import {
  Button,
  FormControl,
  FormHelperText,
  FormLabel,
  Stack,
  Typography,
  Box,
} from "@mui/joy";
import useAuth from "../../utils/useAuth";
import { useEffect } from "react";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import InfoOutlined from "@mui/icons-material/InfoOutlined";

const SelectUser = ({ setUserId, setDisable }) => {
  const { post } = useAuth();
  const [email, setEmail] = React.useState("");
  const [helperText, setHelperText] = React.useState("");
  const [loading, setLoading] = React.useState(false);
  const [validEmail, setValidEmail] = React.useState(null);

  useEffect(() => {
    setDisable(true);
    return () => setDisable(false);
  }, []);

  async function verifyClient(email) {
    setLoading(true);
    try {
      const response = await post("/staff/rental/verifyclient", {
        email: email,
      });
      if (response.data) {
        setUserId(response.data.user_id);
        setValidEmail(true);
        setDisable(false);
      }
    } catch (error) {
      setValidEmail(false);
      let errorMessage =
        "Hubo un error al verificar el correo, intentalo mas tarde.";
      switch (error.response?.status) {
        case 404:
          errorMessage =
            "El correo electronico no esta registrado o no pertenece a un cliente.";
          break;
        case 403:
          errorMessage =
            "No tenes permisos para realizar esta accion o tu sesion se vencio.";
          break;
      }
      setHelperText(errorMessage);
    } finally {
      setLoading(false);
    }
  }

  return (
    <>
      <Box
        sx={{
          p: 3,
          display: "flex",
          alignItems: "center",
          flexDirection: "column",
          justifyContent: "center",
        }}
      >
        <FormControl error={!!helperText && !validEmail}>
          <FormLabel>Correo electronico del cliente</FormLabel>
          <Stack direction="row" spacing={1}>
            <Input
              placeholder="cliente@ejemplo.com"
              sx={{ width: "400px" }}
              type="email"
              value={email}
              onChange={(e) => {
                setEmail(e.target.value);
                setValidEmail(null);
                setHelperText("");
              }}
              endDecorator={
                validEmail ? <CheckCircleIcon color="success" /> : null
              }
              disabled={loading || validEmail}
            />
            <Button
              color="danger"
              disabled={
                !email ||
                !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email) ||
                validEmail
              }
              loading={loading}
              onClick={() => verifyClient(email)}
            >
              Verificar
            </Button>
          </Stack>
          {helperText && (
            <FormHelperText>
              <InfoOutlined />
              {helperText}
            </FormHelperText>
          )}
        </FormControl>
        {email && validEmail && (
          <Typography sx={{ mt: 2, width: "90%" }}>
            Vas a continuar con el registro de alquiler para el cliente de
            correo
            <strong> {email}</strong>.
          </Typography>
        )}
      </Box>
    </>
  );
};

export default SelectUser;
