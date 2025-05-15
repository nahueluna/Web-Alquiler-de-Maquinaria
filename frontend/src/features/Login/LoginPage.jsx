import Button from "@mui/joy/Button";
import FormHelperText from "@mui/joy/FormHelperText";
import FormLabel from "@mui/joy/FormLabel";
import Input from "@mui/joy/Input";
import Option from "@mui/joy/Option";
import Select from "@mui/joy/Select";
import Sheet from "@mui/joy/Sheet";
import Stack from "@mui/joy/Stack";
import { useColorScheme } from "@mui/joy/styles";
import Typography from "@mui/joy/Typography";
import { useFormik } from "formik";
import * as React from "react";
import * as yup from "yup";

function ModeToggle() {
  const { mode, setMode } = useColorScheme();
  const [mounted, setMounted] = React.useState(false);

  // necessary for server-side rendering
  // because mode is undefined on the server
  React.useEffect(() => {
    setMounted(true);
  }, []);
  if (!mounted) {
    return <Button variant="soft">Change mode</Button>;
  }

  return (
    <Select
      variant="soft"
      value={mode}
      onChange={(event, newMode) => {
        setMode(newMode);
      }}
      sx={{ width: "max-content" }}
    >
      <Option value="system">System</Option>
      <Option value="light">Light</Option>
      <Option value="dark">Dark</Option>
    </Select>
  );
}

export default function LoginPage(props) {
  const validationSchema = yup.object({
    email: yup
      .string("Enter your email")
      .email("Enter a valid email")
      .required("Email is required"),
    password: yup
      .string("Enter your password")
      .min(8, "Password should be of minimum 8 characters length")
      .required("Password is required"),
  });

  const formik = useFormik({
    initialValues: {
      email: "foobar@example.com",
      password: "foobar",
    },
    validationSchema: validationSchema,
    onSubmit: (values) => {
      alert(JSON.stringify(values, null, 2));
    },
  });
  return (
    <Sheet
      variant="outlined"
      sx={{ p: 4, borderRadius: "md", width: 400, mx: "auto", mt: 6 }}
    >
      <Typography level="h4" mb={2}>
        Iniciar sesión
      </Typography>

      <form onSubmit={formik.handleSubmit}>
        <Stack spacing={2}>
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
              <FormHelperText color="danger">
                {formik.errors.email}
              </FormHelperText>
            )}
          </div>

          <div>
            <FormLabel>Contraseña</FormLabel>
            <Input
              name="password"
              type="password"
              value={formik.values.password}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.password && Boolean(formik.errors.password)}
            />
            {formik.touched.password && formik.errors.password && (
              <FormHelperText color="danger">
                {formik.errors.password}
              </FormHelperText>
            )}
          </div>

          <Button type="submit" variant="solid" color="primary">
            Ingresar
          </Button>
        </Stack>
      </form>
    </Sheet>
  );
}
