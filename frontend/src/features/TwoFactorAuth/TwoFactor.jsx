import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";
import Shield from "@mui/icons-material/Shield";
import { Box, Button, FormLabel, Link, Sheet, Typography } from "@mui/joy";
import Snackbar from "@mui/joy/Snackbar";
import { useContext, useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import UserContext from "../../context/UserContext";
import InputGroup from "./InputGroup";

const cooldownTime = 60;
let interval, timeout;

function TwoFactor() {
  const { login, user, setUser } = useContext(UserContext);
  const { state } = useLocation();
  const [code, setCode] = useState(new Array(6).fill(""));
  const [isComplete, setIsComplete] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [cooldown, setCooldown] = useState(cooldownTime);
  const [newCodeDisable, setNewCodeDisable] = useState(false);

  const refs = useRef([]);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const nav = useNavigate();

  // Redirect if the user is logged in or if there is no state from the login page
  useEffect(() => {
    if (user !== null || !state) {
      nav("/");
    }
  }, [user, state]);

  useEffect(() => {
    if (code.every((x) => x !== "")) {
      setIsComplete(true);
    } else {
      setIsComplete(false);
    }
  }, [code]);

  // Focus the first input when mounted
  useEffect(() => {
    refs.current[0].focus();

    return () => {
      clearInterval(interval);
      clearTimeout(timeout);
    };
  }, []);

  function editInputs(disabled) {
    refs.current.forEach((x) => {
      if (disabled) x.blur();
      x.disabled = disabled;
    });
    if (!disabled) refs.current[refs.current.length - 1].focus();
  }

  function handleChange(e, i) {
    const { value } = e.target;

    if (value.match(/^\d{2}$/) && value.length > 1 && i + 2 <= code.length) {
      setCode((prev) => {
        prev[i + 1] = value.split("")[1];
        return [...prev];
      });
      refs.current[i + 2 === code.length ? code.length - 1 : i + 2]?.focus();
      return;
    }

    // If the value is 1 or 2 digits (typing normally)
    if (value.match(/^\d{1}$/)) {
      // If the input already has a value, set the next one (2 digits)
      setCode((prev) => {
        prev[i] = value;
        return [...prev];
      });
      refs.current[i + 1]?.focus(); // Focus the next input
    } else if (value.match(/\d{1}/) && value.length === code.length) {
      // If the value is larger than 1 digit (pasting a code)
      setCode(value.split(""));
      refs.current[refs.current.length - 1].focus(); // Focus the last input
    }
  }

  function handleKeyDown(e, i) {
    const { code, keyCode } = e;

    // keyCode 8 is backspace on mobile
    if ((code === "Backspace" || keyCode === 8) && i >= 0) {
      refs.current[i - 1]?.focus(); // Focus the previous input
      setCode((prev) => {
        prev[i] = "";
        return [...prev];
      });
    }
  }

  async function handleVerify() {
    setIsLoading(true);
    editInputs(true); // disable inputs
    try {
      const user = await login({ ...state }, parseInt(code.join("")));

      setStatus({
        isError: false,
        message: "Inicio de sesion exitoso, redirigiendo en 5 segundos...",
      });
      setOpenSnack(true);
      setIsComplete(false); // keep the button disabled but not loading

      setTimeout(() => {
        setUser(user);
      }, 5000);
    } catch (error) {
      console.error(error);
      switch (error.response.status) {
        case 400:
          setStatus({
            isError: true,
            message: "Ocurrió un error al verificar el código. Inténtalo de nuevo.",
          });
          break;
        default:
          setStatus({
            isError: true,
            message:
              "Ocurrió un error al verificar el código. Inténtalo de nuevo.",
          });
      }
      setOpenSnack(true);
      editInputs(false); // enable the inputs
    } finally {
      setIsLoading(false);
    }
  }

  async function handleNewCodeRequest() {
    setNewCodeDisable(true);
    interval = setInterval(() => {
      setCooldown((prev) => prev - 1);
      console.log("INTERVALO");
    }, 1000);
    timeout = setTimeout(() => {
      clearInterval(interval);
      setNewCodeDisable(false);
      setCooldown(cooldownTime);
    }, cooldownTime * 1000);
    try {
      const response = await login({ ...state });
      setOpenSnack(true);
      setStatus({
        isError: false,
        message: "Se envio un nuevo codigo a tu correo.",
      });
    } catch (error) {
      clearTimeout(timeout);
      clearInterval(interval);
      setNewCodeDisable(false);
      setOpenSnack(true);
      setStatus({
        isError: true,
        message:
          "Ocurrió un error al enviar el nuevo código. Intentalo mas tarde",
      });
    }
  }

  return (
    <>
      <Snackbar
        variant="soft"
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
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
        sx={{
          display: "grid",
          placeItems: "center",
          minHeight: "100vh",
          backgroundColor: {
            sm: "#f4f4f4",
          },
        }}
      >
        {/* Container */}
        <Sheet
          sx={{
            boxShadow: {
              xs: "none",
              sm: "xl",
            },
            px: {
              xs: 2,
              sm: 5,
            },
            py: 5,
            borderRadius: "md",
          }}
        >
          <Box>
            {/* Title and desc */}
            <Box
              sx={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
              }}
            >
              <Shield
                sx={{
                  fontSize: 60,
                  mb: 2,
                  backgroundColor: "#fbcece",
                  padding: 1,
                  borderRadius: 100,
                }}
                color="danger"
              />
              <Typography level="h2" textAlign="center" mb={0}>
                Verificación de 2 pasos
              </Typography>
              <Typography
                level="body-sm"
                textAlign="center"
                maxWidth={370}
                mb={5}
              >
                Por tu seguridad, ingresa el codigo de 6 digitos que fue enviado
                a tu email
              </Typography>
            </Box>

            <FormLabel sx={{ fontSize: "lg", mb: 1 }}>
              Codigo de verificacion
            </FormLabel>
            <InputGroup
              code={code}
              refs={refs}
              onChange={handleChange}
              onKeyDown={handleKeyDown}
            />

            {/* button and resend code */}
            <Box
              mt={1}
              sx={{
                display: "flex",
                flexDirection: "column",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <Button
                loading={isLoading}
                disabled={!isComplete || isLoading}
                onClick={handleVerify}
                sx={{
                  width: "100%",
                  py: 2,
                  my: 3,
                }}
                variant="solid"
                color="danger"
              >
                Verificar
              </Button>

              <Typography level="body-sm">
                No recibiste el codigo?{" "}
                <Link disabled={newCodeDisable} onClick={handleNewCodeRequest}>
                  Reenviar codigo
                </Link>{" "}
                {newCodeDisable && <Typography>en {cooldown}s</Typography>}
              </Typography>
            </Box>
          </Box>
        </Sheet>
      </Sheet>
    </>
  );
}

export default TwoFactor;
