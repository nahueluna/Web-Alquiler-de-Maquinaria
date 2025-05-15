import { ErrorMessage, Field, Form, Formik } from "formik";
import React from "react";
import { Link } from "react-router-dom";
import * as Yup from "yup";

const schema = Yup.object().shape({
  email: Yup.string().email("Invalid email").required("Email is required"),
  password: Yup.string()
    .min(8, "Password must be at least 8 characters")
    .required("Password is required"),
});

const Login = () => {
  const [localStore, setLocalStore] = React.useState();
  return (
    <div>
      <h2>Ingresa tu email para iniciar sesion</h2>
      <Formik
        initialValues={{ email: "", password: "" }}
        validationSchema={schema}
        onSubmit={(values, { setSubmitting }) => {
          values.email = values.email.trim();
          values.password = values.password.trim();
          setLocalStore(values);
          setTimeout(() => {
            console.log(values);
            setSubmitting(false);
          }, 400);
        }}
      >
        {({ isSubmitting }) => (
          <>
            <Form>
              <label htmlFor="email">Email</label>
              <Field type="email" name="email" />
              <ErrorMessage
                name="email"
                component="p"
                className="error-message"
              />
              <label htmlFor="password">Password</label>
              <Field type="password" name="password" />
              <ErrorMessage
                name="password"
                component="p"
                className="error-message"
              />
              <button type="submit" disabled={isSubmitting}>
                Iniciar sesion
              </button>
            </Form>
            <pre>{JSON.stringify(localStore, null, 2)}</pre>
          </>
        )}
      </Formik>
      <Link to="/forgot-password">
        <a>Olvidaste tu contraseña?</a>
      </Link>
      <Link to="/register">
        <button>Registrarse</button>
      </Link>
    </div>
  );
};

export default Login;
