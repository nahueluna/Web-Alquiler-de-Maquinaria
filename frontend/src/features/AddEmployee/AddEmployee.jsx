import React, { useState } from 'react';
import { Box, Button, Input, FormLabel, FormHelperText, Typography, Snackbar, Alert } from '@mui/joy';
import { useFormik } from 'formik';
import * as Yup from 'yup';

function AddEmployee() {
  const [openSnackbar, setOpenSnackbar] = useState(false);

  const formik = useFormik({
    initialValues: {
      email: '',
      nombre: '',
      apellido: '',
      fechaNacimiento: '',
      dni: '',
      telefono: '',
    },
    validationSchema: Yup.object({
      email: Yup.string()
        .email('Email inválido')
        .required('Email es obligatorio'),
      nombre: Yup.string().required('Nombre es obligatorio'),
      apellido: Yup.string().required('Apellido es obligatorio'),
      fechaNacimiento: Yup.date()
        .required('Fecha de nacimiento es obligatoria')
        .typeError('Fecha inválida (formato YYYY-MM-DD)'),
      dni: Yup.string()
        .matches(/^\d+$/, 'DNI debe contener solo números')
        .required('DNI es obligatorio'),
      telefono: Yup.string().matches(/^\d*$/, 'Teléfono debe contener solo números'),
    }),
    onSubmit: (values, { setSubmitting, resetForm }) => {
      setSubmitting(false);
      setOpenSnackbar(true);
      resetForm();
      // ACÁ CONECTAR CON EL BACKEND
    },
  });

  const handleCloseSnackbar = (event, reason) => {
    if (reason === 'clickaway') return;
    setOpenSnackbar(false);
  };

  const renderInput = (label, name, type = 'text', required = false) => {
    const showError = formik.touched[name] && Boolean(formik.errors[name]);
    return (
      <Box>
        <FormLabel htmlFor={name}>
          {label} {required && '*'}
        </FormLabel>
        <Input
          id={name}
          name={name}
          type={type}
          value={formik.values[name]}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={showError}
        />
        {showError && <FormHelperText color="danger">{formik.errors[name]}</FormHelperText>}
      </Box>
    );
  };

  return (
    <>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        sx={{
          maxWidth: 400,
          mx: 'auto',
          mt: 4,
          mb: 4,
          display: 'flex',
          flexDirection: 'column',
          gap: 2,
          p: 4,
          borderRadius: 'lg',
          boxShadow: 'sm',
          backgroundColor: 'background.surface',
          border: '1px solid',
          borderColor: 'neutral.outlinedBorder',
        }}
        noValidate
      >
        <Typography level="h4" component="h1" textAlign="center" mb={2}>
          Registrar empleado
        </Typography>

        {renderInput('Email', 'email', 'email', true)}
        {renderInput('Nombre', 'nombre', 'text', true)}
        {renderInput('Apellido', 'apellido', 'text', true)}
        {renderInput('Fecha de nacimiento', 'fechaNacimiento', 'date', true)}
        {renderInput('DNI', 'dni', 'text', true)}
        {renderInput('Teléfono (opcional)', 'telefono', 'text', false)}

        <Button type="submit" disabled={formik.isSubmitting}>
          Registrarse
        </Button>
      </Box>

      <Snackbar
        open={openSnackbar}
        autoHideDuration={3000}
        onClose={handleCloseSnackbar}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
        sx={{
          backgroundColor: 'transparent',
          boxShadow: 'none',
          padding: 0,
        }}
      >
        <Alert color="success" variant="soft" onClose={handleCloseSnackbar}>
          El empleado ha sido registrado correctamente.
        </Alert>
      </Snackbar>
    </>
  );
}

export default AddEmployee;