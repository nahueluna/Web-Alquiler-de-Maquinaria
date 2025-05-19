import { Box, Button, FormLabel, Link, Sheet, Typography } from "@mui/joy";
import React, { useEffect, useRef, useState } from "react";
import InputGroup from "./InputGroup";
import Shield from "@mui/icons-material/Shield";

function TwoFactor() {
  // TODO: Check if the user should be here, if not redirect him to login
  const [code, setCode] = useState(new Array(6).fill(""));
  const [isComplete, setIsComplete] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const refs = useRef([]);

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

  function handleVerify() {
    // TODO:
    setIsLoading(true);
    editInputs(true);
    setTimeout(() => {
      setIsLoading(false);
      editInputs(false);
    }, 2000);
  }

  return (
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
              Por tu seguridad, ingresa el codigo de 6 digitos que fue enviado a
              tu email
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
              disabled={!isComplete}
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
              No recibiste el codigo? <Link>Reenviar codigo</Link>
            </Typography>
          </Box>
        </Box>
      </Sheet>
    </Sheet>
  );
}

export default TwoFactor;
