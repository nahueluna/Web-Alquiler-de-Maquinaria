import {
  Sheet,
  AccordionGroup,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Avatar,
  Typography,
  Button,
  FormControl,
  FormLabel,
  Input,
  Select,
  Option,
  Snackbar,
  Alert,
  Card,
  CardCover,
  CardContent,
  Stack,
  Divider,
  Box,
} from "@mui/joy";
import { Formik, Form, Field, ErrorMessage, useField } from "formik";
import * as Yup from "yup";
import { useState, useEffect, useContext } from "react";
import axios from "axios";
import UserContext from "../../context/UserContext";
import useAuth from "../utils/useAuth";

const MachineCopies = () => {

  const { user } = useContext(UserContext);
  const [serialNumber, setSerialNumber] = useState("");
  const [result, setResult] = useState(null);
  const { get, post } = useAuth();
  const [snackbar, setSnackbar] = useState({
    open: false,
    message: "",
    color: "neutral",
    duration: 3000,
  });
  // Historial
  const [history, setHistory] = useState(null);
  const [loadingHistory, setLoadingHistory] = useState(false);
  const [historyMessage, setHistoryMessage] = useState("");
  // Actualizar historial
  const [showUpdateForm, setShowUpdateForm] = useState(false);
  const [loadingUpdate, setLoadingUpdate] = useState(false);


const handleSearch = async () => {
  const trimmedSerial = serialNumber.trim();
  if (!trimmedSerial) {
    setSnackbar({
      open: true,
      message: "Debe ingresar un número de serie.",
      color: "danger",
      duration: 3000,
    });
    return;
  }

  console.log("🔍 Buscando serial:", trimmedSerial);
  console.log("🔐 Token de acceso:", user.access);

  try {
    const response = await post(
      `unit/${trimmedSerial}`,
      { access: user.access },
      { withCredentials: true }
    );
    setResult(response.data);
    setSnackbar({ open: false, message: "", color: "neutral", duration: 3000 });
    setHistory(null); // Limpiar historial al buscar nueva máquina
  } catch (err) {
    if (err.response) {
      const status = err.response.status;
      let message = "Error desconocido.";
      switch (status) {
        case 400:
          message = "El número de serie es una cadena vacía.";
          break;
        case 401:
          message = "Token inválido. Por favor inicia sesión nuevamente.";
          break;
        case 403:
          message = "No tienes permisos para acceder a esta información.";
          break;
        case 404:
          message = "El número de serie no corresponde a un ejemplar existente.";
          break;
        case 500:
          message = "Ocurrió un error al conectarse a la base de datos.";
          break;
      }
      setSnackbar({
        open: true,
        message,
        color: "danger",
        duration: 4000,
      });
    } else {
      setSnackbar({
        open: true,
        message: "Error de conexión con el servidor.",
        color: "danger",
        duration: 4000,
      });
    }
    setResult(null);
    setHistory(null);
  }
}

const handleFetchHistory = async () => {
  if (!result || !result.unit_info || !result.unit_info.unit_id) return;

  setLoadingHistory(true);
  setHistoryMessage("");
  try {
    const response = await post(
      `unit/${result.unit_info.unit_id}/history`,
      { access: user.access },
      { withCredentials: true }
    );
    const data = response.data;

    if (data.history && data.history.length === 0) {
      setHistory([]);
      setHistoryMessage("La máquina no posee historial de mantenimiento.");
    } else {
      setHistory(data.history);
      setHistoryMessage("");
    }
  } catch (err) {
    console.error("Error al obtener historial:", err);
    let message = "Error desconocido.";
    if (err.response) {
      switch (err.response.status) {
        case 401:
          message = "Token inválido. Por favor inicia sesión nuevamente.";
          break;
        case 403:
          message = "No tienes permisos para acceder a esta información.";
          break;
        case 404:
          message = "El id de la unidad no existe.";
          break;
        case 500:
          message = "Ocurrió un error al obtener el historial.";
          break;
      }
    } else {
      message = "Error de conexión con el servidor.";
    }
    setSnackbar({
      open: true,
      message,
      color: "danger",
      duration: 4000,
    });
    setHistory(null);
    setHistoryMessage("");
  } finally {
    setLoadingHistory(false);
  }
}

const UpdateSchema = Yup.object().shape({
  description: Yup.string().nullable(),
  new_status: Yup.string()
    .oneOf(["available", "maintenance"], "Nuevo estado inválido")
    .required("Requerido"),
});

const handleSubmitUpdate = async (values, { setSubmitting, resetForm }) => {
  setLoadingUpdate(true);
  try {
    const payload = {
      access: user.access,
      unit_id: result.unit_info.unit_id,
      description: values.description || null,
      new_status: values.new_status,
    };

    console.log("Enviando payload: ", payload);
    console.log("Tipo de unit_id:", typeof payload.unit_id);
    console.log("new_status:", payload.new_status);
    const response = await post("unit/history/update", payload, { withCredentials: true });

    if (response.status === 201) {
      setSnackbar({
        open: true,
        message: "Historial actualizado con éxito.",
        color: "success",
        duration: 3000,
      });
      setShowUpdateForm(false);
      resetForm();
      handleFetchHistory();
    } else {
      setSnackbar({
        open: true,
        message: "Respuesta inesperada del servidor.",
        color: "warning",
        duration: 3000,
      });
    }
  } catch (err) {
    let message = "Error desconocido al actualizar historial.";
    if (err.response) {
      switch (err.response.status) {
        case 401:
          message = "Token inválido. Por favor inicia sesión nuevamente.";
          break;
        case 403:
          message = "No tienes permisos para actualizar el historial.";
          break;
        case 500:
          message = err.response.data?.message || "Error interno en el servidor.";
          break;
      }
    } else {
      message = "Error de conexión con el servidor.";
    }
    setSnackbar({
      open: true,
      message,
      color: "danger",
      duration: 4000,
    });
  } finally {
    setLoadingUpdate(false);
    setSubmitting(false);
  }
};


return (
  <>
    <Sheet sx={{ p: 4, maxWidth: 600, mx: "0"}}>
      <Typography level="h4" mb={3} fontWeight="lg" textAlign="center">
        Buscar ejemplar por número de serie
      </Typography>

      <FormControl sx={{ mb: 3 }}>
        <FormLabel>Número de serie</FormLabel>
        <Input
          placeholder="Ej: CAT-001"
          value={serialNumber}
          onChange={(e) => setSerialNumber(e.target.value)}
          size="md"
        />
      </FormControl>

      <Button
        onClick={handleSearch}
        color="danger"
        disabled={serialNumber.trim() === ""}
        size="md"
        fullWidth
        sx={{ mb: 4 }}
      >
        Buscar
      </Button>

      {/* Resultado */}
      {result && (
        <Card sx={{ boxShadow: 4, borderRadius: 3 }}>
          <CardContent>
            <Stack spacing={1.5}>
              <Typography level="h5" fontWeight="lg">
                {result.unit_info.name}
              </Typography>
              <Typography color="text.secondary">
                <strong>Modelo:</strong> {result.unit_info.model}
              </Typography>
              <Typography color="text.secondary">
                <strong>Marca:</strong> {result.unit_info.brand}
              </Typography>
              <Typography color="text.secondary">
                <strong>Año:</strong> {result.unit_info.year}
              </Typography>
              <Typography
                color={
                  result.unit_info.status === "available"
                    ? "success.main"
                    : "warning.main"
                }
                fontWeight="md"
              >
                <strong>Estado:</strong> {result.unit_info.status}
              </Typography>
              <Typography color="text.secondary">
                <strong>Ubicación:</strong> {result.unit_info.street},{" "}
                {result.unit_info.city}
              </Typography>              
            </Stack>

            <Divider sx={{ my: 3 }} />

            <Stack direction="row" spacing={2} flexWrap="nowrap" justifyContent="flex-start">
              <Button
                variant="outlined"
                color="danger"
                onClick={handleFetchHistory}
                disabled={loadingHistory}
                size="md"
                sx={{ minWidth: 140 }}
              >
                {loadingHistory ? "Cargando historial..." : "Ver historial de mantenimiento"}
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() => setShowUpdateForm(true)}
                size="md"
                sx={{ minWidth: 140 }}
              >
                Actualizar historial de mantenimiento
              </Button>
            </Stack>

            {historyMessage && (
              <Typography
                level="body2"
                sx={{ mt: 2, color: "text.secondary", fontStyle: "italic" }}
              >
                {historyMessage}
              </Typography>
            )}

            {history && history.length > 0 && (
              <AccordionGroup sx={{ mt: 3 }}>
                {history.map((event) => (
                  <Accordion key={event.event_id} variant="outlined" sx={{ mb: 1 }}>
                    <AccordionSummary>
                      {new Date(event.created_at).toLocaleString()}
                    </AccordionSummary>
                    <AccordionDetails>
                      <Typography>
                        <strong>Descripción:</strong> {event.description || "-"}
                      </Typography>
                      <Typography>
                        <strong>Estado anterior:</strong> {event.previous_status}
                      </Typography>
                      <Typography>
                        <strong>Nuevo estado:</strong> {event.new_status}
                      </Typography>
                    </AccordionDetails>
                  </Accordion>
                ))}
              </AccordionGroup>
            )}
          </CardContent>
        </Card>
      )}
    </Sheet>

    {/* Formulario de actualización */}
    {showUpdateForm && (
      <Sheet
        sx={{
          p: 3,
          mt: 0,
          maxWidth: 538,
          mx: 4,
          mb: 2,
          borderRadius: 3,
          boxShadow: 3,
          border: "1px solid",
          borderColor: "divider",
          bgcolor: "background.body",
        }}
      >
        <Formik
          initialValues={{
            description: "",
            previous_status: "available",
            new_status: "maintenance",
          }}
          validationSchema={UpdateSchema}
          onSubmit={handleSubmitUpdate}
        >
          {({ isSubmitting }) => (
            <Form>
              <Stack spacing={3}>
                <FormControl>
                  <FormLabel>Descripción (opcional)</FormLabel>
                  <Field
                    name="description"
                    as={Input}
                    placeholder="Ingrese descripción de la actualización"
                    size="md"
                  />
                  <ErrorMessage
                    name="description"
                    component="div"
                    style={{ color: "red", marginTop: 4 }}
                  />
                </FormControl>

                <FormControl>
                  <FormLabel>Nuevo estado</FormLabel>
                  <Field name="new_status">
                    {({ field }) => (
                      <Select
                        {...field}
                        value={field.value}
                        onChange={(_, value) =>
                          field.onChange({
                            target: { name: field.name, value },
                          })
                        }
                        size="md"
                      >
                        <Option value="available">available</Option>
                        <Option value="maintenance">maintenance</Option>
                      </Select>
                    )}
                  </Field>
                  <ErrorMessage
                    name="new_status"
                    component="div"
                    style={{ color: "red", marginTop: 4 }}
                  />
                </FormControl>

                <Stack direction="row" spacing={2} justifyContent="flex-end">
                  <Button
                    type="submit"
                    variant="solid"
                    color="danger"
                    disabled={isSubmitting || loadingUpdate}
                    size="md"
                  >
                    {loadingUpdate ? "Actualizando..." : "Guardar"}
                  </Button>
                  <Button
                    variant="outlined"
                    color="danger"
                    onClick={() => setShowUpdateForm(false)}
                    disabled={isSubmitting || loadingUpdate}
                    size="md"
                  >
                    Cerrar
                  </Button>
                </Stack>
              </Stack>
            </Form>
          )}
        </Formik>
      </Sheet>
    )}

    {/* Snackbar de error */}
    <Snackbar
      open={snackbar.open}
      onClose={() => setSnackbar((prev) => ({ ...prev, open: false }))}
      autoHideDuration={3000}
      anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      variant="soft"
      color={snackbar.color}
    >
      {snackbar.message}
    </Snackbar>
  </>
);
};


export default MachineCopies;