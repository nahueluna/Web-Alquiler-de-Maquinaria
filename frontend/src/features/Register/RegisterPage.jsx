import MailIcon from "@mui/icons-material/Mail";
import {
  Button,
  Checkbox,
  Divider,
  FormControl,
  FormHelperText,
  Input,
  Link,
  Sheet,
  Stack,
  Typography,
} from "@mui/joy";
import React from "react";
import { Link as RouterLink } from "react-router-dom";

const RegisterPage = () => {
  return (
    <Sheet
      variant="outlined"
      sx={{
        p: 4,
        borderRadius: "md",
        width: 600,
        mx: "auto",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <Typography level="h2" mb={2}>
        Crea una cuenta
      </Typography>
      <Divider></Divider>
      <form>
        <Stack spacing={1.5} sx={{ pt: 2 }}>
          <Stack
            direction="row"
            spacing={1}
            sx={{
              justifyContent: "center",
              alignItems: "center",
            }}
          >
            <FormControl>
              <Input placeholder="Nombre" />
            </FormControl>
            <FormControl>
              <Input placeholder="Apellido" />
            </FormControl>
          </Stack>
          <Stack
            direction="row"
            spacing={1}
            sx={{
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <FormControl>
              <Input placeholder="DNI" />
            </FormControl>
            <FormControl sx={{ flex: 1 }}>
              <Input type="date" fullWidth />
            </FormControl>
          </Stack>
          <FormControl>
            <Input
              startDecorator={<MailIcon />}
              type="email"
              placeholder="Correo electronico"
            />
          </FormControl>
          <FormControl>
            <Input
              type="tel"
              placeholder="Numero de celular (opcional)"
              name="telefono"
            />
          </FormControl>
          <FormControl size="sm" sx={{ width: 400 }}>
            <Checkbox
              label={
                <React.Fragment>
                  He leido y acepto los{" "}
                  <Typography sx={{ fontWeight: "md" }}>
                    terminos y condiciones
                  </Typography>
                  .
                </React.Fragment>
              }
            />
            <FormHelperText>
              <Typography level="body-sm">
                Lea nuestros{" "}
                <Link component={RouterLink} to={"/terms"} level="body-sm">
                  terminos y condiciones
                </Link>
                .
              </Typography>
            </FormHelperText>
          </FormControl>
          <Divider></Divider>
          <Stack spacing={2} sx={{ alignItems: "center" }}>
            <Button
              color="success"
              size="lg"
              sx={{ width: "50%" }}
              type="submit"
            >
              Registrarse
            </Button>
            <Link component={RouterLink} to={"/login"} level="body-sm">
              Ya tengo una cuenta
            </Link>
          </Stack>
        </Stack>
      </form>
    </Sheet>
  );
};

export default RegisterPage;
