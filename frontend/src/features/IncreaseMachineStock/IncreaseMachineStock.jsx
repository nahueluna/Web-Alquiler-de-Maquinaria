import React, { useState, useEffect } from "react";
import {
  Box,
  Button,
  Input,
  Typography,
  Sheet,
  Divider,
  FormControl,
  FormLabel,
  Select,
  Option,
  Alert,
} from "@mui/joy";
import { useFormik } from "formik";
import * as Yup from "yup";

const IncreaseMachineStock = () => {
  const [success, setSuccess] = useState(false);

  // Máquinas hardcodeadas del catálogo
  const modelosDisponibles = [
    { marca: "Ford", modelo: "7600, simple", año: "2010" },
    { marca: "John Deere", modelo: "Tractor 5090EN", año: "2020" },
    { marca: "John Deere", modelo: "Tractor 6110E", año: "2021" },
    { marca: "Caterpillar", modelo: "Excavadora CAT 336", año: "2019" },
    { marca: "Bobcat", modelo: "S70", año: "2022" },
    { marca: "JLG", modelo: "Plataforma 1930ES", año: "2018" },
    { marca: "Hamm", modelo: "Rodillo H13i", año: "2021" },
    { marca: "Komatsu", modelo: "PC138US-11", año: "2020" },
    { marca: "Caterpillar", modelo: "Cargadora CAT 950GC", año: "2022" },
  ];

  // Ubicaciones hardcodeadas
  const ubicaciones = ["Caballito", "Azul", "Tandil", "Mar del Plata", "Bahía Blanca"];

  const [modelosFiltrados, setModelosFiltrados] = useState([]);
  const [aniosDisponibles, setAniosDisponibles] = useState([]);

  const formik = useFormik({
    initialValues: {
      marca: "",
      modelo: "",
      año: "",
      numeroSerie: "",
      estado: "Disponible",
      ubicacion: "",
    },
    validationSchema: Yup.object({
      marca: Yup.string().required("Seleccioná una marca"),
      modelo: Yup.string().required("Seleccioná un modelo"),
      año: Yup.string().required("Seleccioná un año"),
      numeroSerie: Yup.string().required("El número de serie es requerido"),
      estado: Yup.string().required("Seleccioná un estado"),
      ubicacion: Yup.string().required("Seleccioná una ubicación"),
    }),
    onSubmit: (values) => {
      console.log("Ejemplar agregado:", values);
      setSuccess(true);
    },
  });

  useEffect(() => {
    if (formik.values.marca) {
      const filtrados = modelosDisponibles.filter(
        (m) => m.marca === formik.values.marca
      );
      setModelosFiltrados(filtrados);
      formik.setFieldValue("modelo", "");
      formik.setFieldValue("año", "");
      setAniosDisponibles([]);
    } else {
      setModelosFiltrados([]);
      formik.setFieldValue("modelo", "");
      formik.setFieldValue("año", "");
      setAniosDisponibles([]);
    }
  }, [formik.values.marca]);

  useEffect(() => {
    if (formik.values.modelo) {
      const años = modelosFiltrados
        .filter((m) => m.modelo === formik.values.modelo)
        .map((m) => m.año);
      setAniosDisponibles(años);
      formik.setFieldValue("año", "");
    } else {
      setAniosDisponibles([]);
      formik.setFieldValue("año", "");
    }
  }, [formik.values.modelo]);

  return (
    <Sheet
      sx={{
        maxWidth: 500,
        mx: "auto",
        mt: 4,
        p: 4,
        borderRadius: "lg",
        boxShadow: "sm",
        backgroundColor: "background.surface",
      }}
    >
      <Typography level="h4" fontWeight="lg" mb={2}>
        Agregar ejemplar de máquina
      </Typography>

      <Divider sx={{ mb: 2 }} />

      {success ? (
        <Alert color="success" variant="soft">
          Ejemplar agregado con éxito.
        </Alert>
      ) : (
        <form onSubmit={formik.handleSubmit}>
          {/* Marca */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Marca</FormLabel>
            <Select
              name="marca"
              value={formik.values.marca}
              onChange={(e, value) => formik.setFieldValue("marca", value)}
              onBlur={formik.handleBlur}
            >
              {[...new Set(modelosDisponibles.map((m) => m.marca))].map(
                (marca) => (
                  <Option key={marca} value={marca}>
                    {marca}
                  </Option>
                )
              )}
            </Select>
            {formik.touched.marca && formik.errors.marca && (
              <Typography level="body-sm" color="danger">
                {formik.errors.marca}
              </Typography>
            )}
          </FormControl>

          {/* Modelo */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Modelo</FormLabel>
            <Select
              name="modelo"
              value={formik.values.modelo}
              onChange={(e, value) => formik.setFieldValue("modelo", value)}
              onBlur={formik.handleBlur}
              disabled={!formik.values.marca}
            >
              {[...new Set(modelosFiltrados.map((m) => m.modelo))].map((modelo) => (
                <Option key={modelo} value={modelo}>
                  {modelo}
                </Option>
              ))}
            </Select>
            {formik.touched.modelo && formik.errors.modelo && (
              <Typography level="body-sm" color="danger">
                {formik.errors.modelo}
              </Typography>
            )}
          </FormControl>

          {/* Año */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Año</FormLabel>
            <Select
              name="año"
              value={formik.values.año}
              onChange={(e, value) => formik.setFieldValue("año", value)}
              onBlur={formik.handleBlur}
              disabled={!formik.values.modelo}
            >
              {aniosDisponibles.map((año) => (
                <Option key={año} value={año}>
                  {año}
                </Option>
              ))}
            </Select>
            {formik.touched.año && formik.errors.año && (
              <Typography level="body-sm" color="danger">
                {formik.errors.año}
              </Typography>
            )}
          </FormControl>

          {/* Número de Serie */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Número de Serie</FormLabel>
            <Input
              name="numeroSerie"
              value={formik.values.numeroSerie}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              placeholder="Ej: 666TSD"
            />
            {formik.touched.numeroSerie && formik.errors.numeroSerie && (
              <Typography level="body-sm" color="danger">
                {formik.errors.numeroSerie}
              </Typography>
            )}
          </FormControl>

          {/* Estado */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Estado</FormLabel>
            <Select
              name="estado"
              value={formik.values.estado}
              onChange={(e, newValue) => formik.setFieldValue("estado", newValue)}
              onBlur={formik.handleBlur}
            >
              <Option value="Disponible">Disponible</Option>
              <Option value="En mantenimiento">En mantenimiento</Option>
              <Option value="No disponible">No disponible</Option>
            </Select>
            {formik.touched.estado && formik.errors.estado && (
              <Typography level="body-sm" color="danger">
                {formik.errors.estado}
              </Typography>
            )}
          </FormControl>

          {/* Ubicación */}
          <FormControl sx={{ mb: 2 }}>
            <FormLabel>Ubicación</FormLabel>
            <Select
              name="ubicacion"
              value={formik.values.ubicacion}
              onChange={(e, value) => formik.setFieldValue("ubicacion", value)}
              onBlur={formik.handleBlur}
            >
              {ubicaciones.map((u) => (
                <Option key={u} value={u}>
                  {u}
                </Option>
              ))}
            </Select>
            {formik.touched.ubicacion && formik.errors.ubicacion && (
              <Typography level="body-sm" color="danger">
                {formik.errors.ubicacion}
              </Typography>
            )}
          </FormControl>

          <Button type="submit" color="primary" fullWidth>
            Agregar
          </Button>
        </form>
      )}
    </Sheet>
  );
};

export default IncreaseMachineStock;