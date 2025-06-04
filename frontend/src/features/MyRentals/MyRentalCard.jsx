import {
  AspectRatio,
  Box,
  Button,
  Card,
  CardContent,
  CardOverflow,
  Chip,
  Divider,
  Modal,
  Sheet,
  Stack,
  Typography,
  Snackbar
} from "@mui/joy";
import React, { useContext, useState } from "react";
import UserContext from "../../context/UserContext";
import useAuth from "../utils/useAuth";

const MyRentalCard = ({
  rentalID,
  imageUrl,
  modelName,
  withdrawnDate,
  startDate,
  endDate,
  amountPaid,
  days,
  status,
}) => {
  const [open, setOpen] = React.useState(false);
  const [selectedRentalID, setSelectedRentalID] = React.useState(null);
  const { user } = useContext(UserContext);
  const token = user?.access || "";
  const { post } = useAuth();
  const [snackbar, setSnackbar] = useState({
    open: false,
    message: '',
    color: '', // o "danger"
  });
  
  const translateStatus = (status) => {
  switch (status) {
    case "pending_payment":
      return "Pago pendiente";
    case "active":
      return "Activo";
    case "completed":
      return "Completado";
    case "cancelled":
      return "Cancelado";
    case "failed":
      return "Fallido";
    default:
      return status;
  }
};

const handleConfirmedCancel = async (rentalID) => {
  try {
    console.log("Antes de hacer la llamada API");
    const data = {
      access: token,
  rental_id: rentalID,
  reason: null,
};
console.log("Datos enviados a la API:", data);

const response = await post("/rental/cancel", data);
    console.log("-----------Respuesta de la API:-----------", response);

    if (response.status === 200) {
      setSnackbar({
        open: true,
        message: "Alquiler cancelado exitosamente.",
        color: "success",
      });
    }
  } catch (error) {
    console.log("Error capturado:", error);
    if (error.response) {
      const { status, data } = error.response;
      switch (status) {
      case 400:
        setSnackbar({
          open: true,
          message:
            "No se puede cancelar este alquiler porque ya comenzó y no ha finalizado o ya fue retirado.",
          color: "danger",
        });
        break;
      case 401:
        setSnackbar({
          open: true,
          message: "Token inválido. Por favor, inicia sesión de nuevo.",
          color: "danger",
        });
        break;
      case 404:
        setSnackbar({
          open: true,
          message:
            "No se encontró el alquiler o no está en un estado que pueda ser cancelado (ya cancelado, completado o fallido).",
          color: "danger",
        });
        break;
      case 500:
        setSnackbar({
          open: true,
          message: "Error interno con la base de datos. Intenta más tarde.",
          color: "danger",
        });
        break;
      default:
        setSnackbar({
          open: true,
          message: `Error inesperado: ${data?.mensaje || "sin detalles"}`,
          color: "danger",
        });
    }
  } else {
    setSnackbar({
      open: true,
      message: "Error de conexión con el servidor.",
      color: "danger",
    });
  }
  } finally {
    setOpen(false);
  }
};
  return (
    <>
      <Card variant="outlined" sx={{ width: "60%", minWidth: "700px" }}>
        <CardOverflow variant="soft">
          <Stack
            direction="row"
            sx={{ py: 1, justifyContent: "space-between" }}
          >
            <Typography fontWeight="bold">ID {rentalID}</Typography>
            <Typography>
              {new Date(startDate).toLocaleDateString("es-AR", {
                day: "2-digit",
                month: "2-digit",
                year: "numeric",
              })}
            </Typography>
          </Stack>
          <Divider inset="context" />
        </CardOverflow>
        <CardContent variant="solid">
          <Stack
            direction="row"
            spacing={2}
            sx={{
              width: "100%",
              height: "100%",
              justifyContent: "space-between",
            }}
          >
            <Stack direction="row" spacing={2}>
              <AspectRatio ratio="1" sx={{ width: 90 }}>
                <img src={imageUrl} />
              </AspectRatio>
              <Box>
                <Typography level="title-lg">{modelName}</Typography>
                <Typography level="body-md" color="neutral">
                  {withdrawnDate
                    ? `Retirado el ${withdrawnDate}`
                    : "No retirado"}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Fecha de inicio: {startDate}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Fecha de devolucion: {endDate}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Precio de alquiler:
                  <Typography fontWeight="bold"> ${amountPaid}</Typography>
                  <Typography level="body-md" color="neutral">
                    {" "}
                    ({days} días)
                  </Typography>
                </Typography>
              </Box>
            </Stack>
            <Stack
              spacing={1}
              alignItems="flex-end"
              justifyContent="space-between"
              sx={{ minHeight: "100%" }}
            >
              <Chip variant="solid" color="danger" size="lg">
                {translateStatus(status)}
              </Chip>
              {(status === "pending_payment" || status === "active") && (
                <Button
                  variant="outlined"
                  color="danger"
                  size="lg"
                  onClick={() => {
                    setOpen(true);
                    setSelectedRentalID(rentalID);
                  }}
                >
                  Cancelar alquiler
                </Button>
              )}
            </Stack>
          </Stack>
        </CardContent>
      </Card>
      <Modal
        open={open}
        onClose={() => setOpen(false)}
        sx={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Sheet
          variant="outlined"
          sx={{ p: 2, width: "400px", borderRadius: "lg" }}
        >
          <Stack spacing={2} sx={{ p: 1 }}>
            <Typography level="title-lg">
              ¿Estás seguro de que queres cancelar el alquiler?
            </Typography>
            <Typography level="body-md">
              La devolucion del monto abonado esta sujeta a las condiciones de
              la empresa.
            </Typography>
            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button
                variant="soft"
                color="neutral"
                onClick={() => setOpen(false)}
              >
                Volver
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() => handleConfirmedCancel(selectedRentalID)}
              >
                Cancelar alquiler
              </Button>
            </Stack>
          </Stack>
        </Sheet>
      </Modal>
      <Snackbar
        variant="soft"
        color={snackbar.color}
        open={snackbar.open}
        onClose={() => setSnackbar({ ...snackbar, open: false })}
        autoHideDuration={5000}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        {snackbar.message}
      </Snackbar>
    </>
  );
};

export default MyRentalCard;
