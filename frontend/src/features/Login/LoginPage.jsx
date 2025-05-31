import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";
import {
  Button,
  FormControl,
  FormHelperText,
  FormLabel,
  Input,
  Link,
  Sheet,
  Snackbar,
  Stack,
  Typography,
} from "@mui/joy";
import { useFormik } from "formik";
import { useContext, useEffect, useState } from "react";
import { Link as RouterLink, useNavigate } from "react-router-dom";
import * as yup from "yup";
import UserContext from "../../context/UserContext";

export default function LoginPage() {
  const { user, setUser, login } = useContext(UserContext);
  const [loading, setLoading] = useState(false);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const nav = useNavigate();

  useEffect(() => {
    if (user !== null) {
      nav("/");
    }
  }, [user]);

  const validationSchema = yup.object({
    email: yup
      .string("Enter your email")
      .matches(
        /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
        "Enter a valid email"
      )
      .required("Email is required"),
    password: yup
      .string("Enter your password")
      .min(8, "Password should be of minimum 8 characters length")
      .required("Password is required"),
  });

  const formik = useFormik({
    initialValues: {
      email: "",
      password: "",
    },
    validationSchema: validationSchema,
    onSubmit: async (values) => {
      setLoading(true);
      try {
        const user = await login(values);

        if (user?.message && user.message.includes("2FA")) {
          return nav("/two-factor", {
            state: values,
          });
        }

        setStatus({ isError: false, message: "Successfully logged in!" });
        setOpenSnack(true);
        setTimeout(() => setUser(user), 1000);
      } catch (error) {
        setStatus({ isError: true, message: error.response.data.message });
        setOpenSnack(true);
        console.error(error);
      } finally {
        setLoading(false);
      }
    },
  });

  return (
    <>
      <Snackbar
        variant="soft"
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
        autoHideDuration={3000}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
        startDecorator={
          status.isError ? (
            <ErrorOutlineIcon />
          ) : (
            <PlaylistAddCheckCircleRoundedIcon />
          )
        }
        endDecorator={
          <Button
            onClick={() => setOpenSnack(false)}
            size="sm"
            variant="soft"
            color={status.isError ? "danger" : "success"}
          >
            Dismiss
          </Button>
        }
      >
        {status.message}
      </Snackbar>
      <Sheet
        variant="outlined"
        sx={{
          p: 4,
          borderRadius: "md",
          width: 400,
          mx: "auto",
        }}
      >
        <Typography level="h4" mb={2}>
          Iniciar sesión
        </Typography>

        <form onSubmit={formik.handleSubmit}>
          <Stack spacing={2}>
            <FormControl error={formik.errors.email}>
              <div>
                <FormLabel>Email</FormLabel>
                <Input
                  name="email"
                  type="email"
                  value={formik.values.email}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  error={formik.touched.email && Boolean(formik.errors.email)}
                />
                {formik.touched.email && formik.errors.email && (
                  <FormHelperText>{formik.errors.email}</FormHelperText>
                )}
              </div>
            </FormControl>

            <FormControl error={formik.errors.password}>
              <div>
                <FormLabel>Contraseña</FormLabel>
                <Input
                  name="password"
                  type="password"
                  value={formik.values.password}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  error={
                    formik.touched.password && Boolean(formik.errors.password)
                  }
                />
                {formik.touched.password && formik.errors.password && (
                  <FormHelperText>{formik.errors.password}</FormHelperText>
                )}
              </div>
            </FormControl>

            <Link
              component={RouterLink}
              to={"/recover-password"}
              level="body-sm"
              sx={{ alignSelf: "center", fontWeight: "500" }}
            >
              ¿Olvidaste tu contraseña?
            </Link>

            <Button
              disabled={!(formik.isValid && formik.dirty)} // Disable the button initially and until the form is valid
              loading={loading}
              type="submit"
              variant="solid"
              color="primary"
            >
              Ingresar
            </Button>
            <Typography level="body-xs" sx={{ textAlign: "center" }}>
              No tenes una cuenta?{" "}
              <Link component={RouterLink} to={"/register"}>
                Registrarse
              </Link>
            </Typography>
          </Stack>
        </form>
      </Sheet>
    </>
  );
}
