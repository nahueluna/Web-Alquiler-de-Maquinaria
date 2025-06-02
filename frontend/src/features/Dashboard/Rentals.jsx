import SearchIcon from "@mui/icons-material/Search";
import { FormControl, Input, Sheet, Stack, Textarea } from "@mui/joy";
import RentalCard from "./RentalCard";
import React, { use } from "react";
import { useEffect } from "react";
import useAuth from "../utils/useAuth";
import { useState } from "react";
import Modal from "@mui/joy/Modal";
import Typography from "@mui/joy/Typography";
import Button from "@mui/joy/Button";
import Snackbar from "@mui/joy/Snackbar";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";
import { Box } from "@mui/joy";

const Rentals = () => {
  const { post } = useAuth();
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const [rentals, setRentals] = useState([]);
  const [value, setValue] = useState("");
  const [open, setOpen] = useState(false);
  const [type, setType] = useState("");
  const [loading, setLoading] = useState(false);
  const [refreshRentals, setRefreshRentals] = useState(false);
  const [cancelReason, setCancelReason] = useState("");
  const [rentalInfo, setRentalInfo] = useState({
    rentalId: null,
    modelName: "",
    modelModel: "",
  });

  useEffect(() => {
    fetchRentals();
  }, [refreshRentals]);

  useEffect(() => {
    setCancelReason("");
  }, [open]);

  const handleConfirmedCancel = async (rentalId, reason = null) => {
    setLoading(true);
    try {
      const response = await post("/rental/cancel", {
        rental_id: rentalId,
        reason: reason.trim() == "" ? null : reason,
      });
      setRefreshRentals((prev) => !prev);
      setOpenSnack(true);
      setStatus({
        isError: false,
        message: `Se cancelo el alquiler de ID ${rentalId} y se notifico al cliente.`,
      });
    } catch (error) {
      let errorMsg = "Ocurrió un error inesperado. Intente nuevamente.";
      switch (error.response?.status) {
        case 400:
          errorMsg =
            "No se puede realizar la acción: fecha de inicio del alquiler es mayor a la actual";
          break;
        case 404:
          errorMsg =
            "No se encontró el alquiler o no está en un estado válido para cancelarlo.";
          break;
        default:
          errorMsg = errorMsg;
      }
      setOpenSnack(true);
      setStatus({
        isError: true,
        message: errorMsg,
      });
    } finally {
      setLoading(false);
      setOpen(false);
    }
  };

  const handleConfirmedRetirement = async (rentalId) => {
    setLoading(true);
    try {
      const response = await post("/loadretirement", { rental_id: rentalId });
      // Por que no hacer un fetchRentals() directamente? Polque no hay polque
      setRefreshRentals((prev) => !prev);
      setOpenSnack(true);
      setStatus({
        isError: false,
        message: `Se registro el retiro para el alquiler ID ${rentalId}.`,
      });
    } catch (error) {
      let errorMsg = "Ocurrió un error inesperado. Intente nuevamente.";
      switch (error.response?.status) {
        case 400:
          errorMsg =
            "No se puede registrar el retiro: el alquiler no existe, no está activo, ya pasó la fecha de finalizacion o es hoy.";
          break;
        case 403:
          errorMsg = "No tenes permisos para realizar esta acción.";
          break;
        default:
          errorMsg = errorMsg;
      }
      setOpenSnack(true);
      setStatus({
        isError: true,
        message: errorMsg,
      });
    } finally {
      setLoading(false);
      setOpen(false);
    }
  };

  const handleConfirmedReturn = () => {
    console.log("Confirming action...");
  };

  const getModalContent = () => {
    switch (type) {
      case "cancel":
        return (
          <>
            <Typography level="h4" mb={2}>
              Estas seguro que queres cancelar el alquiler ID{" "}
              {rentalInfo.rentalId} ({rentalInfo.modelName}{" "}
              {rentalInfo.modelModel})?
            </Typography>
            <Textarea
              placeholder="Indica un motivo (opcional)"
              minRows={2}
              maxRows={4}
              value={cancelReason}
              onChange={(e) => setCancelReason(e.target.value)}
            ></Textarea>
            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button
                variant="soft"
                color="neutral"
                onClick={() => {
                  setOpen(false);
                  setCancelReason("");
                }}
              >
                Cancelar
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() =>
                  handleConfirmedCancel(rentalInfo.rentalId, cancelReason)
                }
                loading={loading}
              >
                Confirmar cancelacion
              </Button>
            </Stack>
          </>
        );
      case "retirement":
        return (
          <>
            <Typography level="title-lg">
              Estas seguro que queres cargar el retiro del alquiler ID{" "}
              {rentalInfo.rentalId} ({rentalInfo.modelName}{" "}
              {rentalInfo.modelModel})?
            </Typography>
            <Typography level="body-md">
              Se va a tomar la fecha y hora actual como fecha de retiro y no se
              va a poder modificar ni eliminar el retiro.
            </Typography>
            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button
                variant="soft"
                color="neutral"
                onClick={() => setOpen(false)}
              >
                Cancelar
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() => handleConfirmedRetirement(rentalInfo.rentalId)}
                loading={loading}
              >
                Confirmar retiro
              </Button>
            </Stack>
          </>
        );
      case "return":
        return (
          <>
            <Typography level="h4" mb={2}>
              Cargar fecha de devolución
            </Typography>

            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button
                variant="soft"
                color="neutral"
                onClick={() => setOpen(false)}
              >
                Cancelar
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() => handleConfirmedReturn(rentalInfo.rentalId)}
                loading={loading}
              >
                Confirmar devolución
              </Button>
            </Stack>
          </>
        );
      default:
        return null;
    }
  };

  const fetchRentals = async (rentalId = null) => {
    try {
      const response = await post(`/staff/rentals?id=${rentalId || ""}`);
      setRentals(response.data.rentals);
    } catch (error) {
      console.error("Error fetching rentals:", error);
      setRentals([]);
    }
  };

  const handleChange = (event) => {
    const cleaned = event.target.value.replace(/[^\d]/g, "");
    setValue(cleaned);
  };

  return (
    <>
      <Snackbar
        variant="soft"
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
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
            Cerrar
          </Button>
        }
      >
        {status.message}
      </Snackbar>
      <Sheet
        sx={{
          display: "flex",
          justifyContent: "flex-start",
          alignItems: "center",
          height: "100%",
        }}
      >
        <Stack spacing={4} sx={{ padding: 2, width: "100%", height: "100%" }}>
          <FormControl sx={{ width: "350px" }}>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                value ? fetchRentals(value) : fetchRentals();
              }}
            >
              <Input
                value={value}
                onChange={handleChange}
                endDecorator={<SearchIcon />}
                placeholder="Buscar por ID de alquiler.."
              ></Input>
            </form>
          </FormControl>
          {Array.isArray(rentals) && rentals.length > 0 ? (
            rentals.map((rental) => (
              <RentalCard
                setOpen={setOpen}
                setType={setType}
                setRentalInfo={setRentalInfo}
                key={rental.rental_id}
                rentalId={rental.rental_id}
                modelName={rental.model_name}
                modelModel={rental.model_model}
                status={rental.status}
                createdAt={rental.created_at}
                startDate={rental.start_date}
                endDate={rental.end_date}
                totalPrice={rental.total_price}
                retirementDate={rental.retirement_date}
                returnDate={rental.return_date}
                modelPolicy={rental.model_policy}
                daysLate={rental.days_late}
                percentageLate={rental.percentage_per_late_day}
              />
            ))
          ) : (
            <p>No hay alquileres disponibles.</p>
          )}
        </Stack>
        <Modal
          open={open}
          onClose={() => setOpen(false)}
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
          }}
        >
          <form>
            <Sheet
              sx={{
                p: 3,
                borderRadius: "md",
                length: "30%",
                maxLength: 500,
                maxWidth: "500px",
                display: "flex",
                flexDirection: "column",
                gap: 1,
              }}
            >
              {getModalContent()}
            </Sheet>
          </form>
        </Modal>
      </Sheet>
    </>
  );
};

export default Rentals;
