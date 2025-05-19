import {
  Button,
  FormHelperText,
  FormLabel,
  Input,
  Sheet,
  Stack,
  Typography,
  Link,
  FormControl,
} from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import { useFormik } from "formik";
import * as yup from "yup";

export default function LoginPage() {
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
    onSubmit: (values) => {
      alert(JSON.stringify(values, null, 2));
    },
  });

  return (
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
            underline="always"
          >
            Olvide mi contraseña
          </Link>

          <Button type="submit" variant="solid" color="primary">
            Ingresar
          </Button>
          <Typography level="body-xs" sx={{ textAlign: "center" }}>
            No tenes una cuenta?{" "}
            <Link component={RouterLink} to={"/register"} underline="always">
              Registrarse
            </Link>
          </Typography>
        </Stack>
      </form>
    </Sheet>
  );
}
