import { Link, Sheet, Stack, Typography, Snackbar, Button, Modal, Select, Option, Textarea } from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import MyRentalCard from "./MyRentalCard";
import axios from "axios";
import React, { useEffect, useState, useContext } from "react";
import UserContext from "../../context/UserContext";
import useAuth from "../utils/useAuth";

const rentalsData = [];

const MyRentalsPage = () => {
  const { user } = useContext(UserContext);
  const [rentalsData, setRentalsData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const { get, post } = useAuth();
  //Modal
  const [modalOpen, setModalOpen] = useState(false);
  const [selectedRental, setSelectedRental] = useState(null);
  const [rating, setRating] = useState(5);
  const [reviewText, setReviewText] = useState("");
  const [submitting, setSubmitting] = useState(false);


  useEffect(() => {
    if (!user?.access) {
      setLoading(false);
      return;
    }

    const fetchRentals = async () => {
      try {
        const response = await post("/myrentals", {
          access: user.access,
        });

        setRentalsData(response.data.rentals);
      } catch (error) {
        if (error.response) {
          switch (error.response.status) {
            case 401:
              setStatus({ isError: true, message: "Token inválido." });
              setOpenSnack(true);
              break;
            case 403:
              setStatus({ isError: true, message: "No eres cliente." });
              setOpenSnack(true);
              break;
            case 500:
              setStatus({
                isError: true,
                message: "Error interno del servidor.",
              });
              setOpenSnack(true);
              break;
            default:
            // No hacemos nada
          }
        } else {
          console.error(error);
        }
      } finally {
        setLoading(false);
      }
    };

    fetchRentals();
  }, [user]);

  const openRatingModal = (rental) => {
    setSelectedRental(rental);
    setRating(5);
    setReviewText("");
    setModalOpen(true);
  };

  const closeModal = () => {
    if (!submitting) {
      setModalOpen(false);
      setSelectedRental(null);
    }
  };

const handleSubmit = async () => {
  if (!selectedRental) return;

  // Validaciones
  if (rating < 1 || rating > 5) {
    setStatus({ isError: true, message: "Rating inválido" });
    setOpenSnack(true);
    return;
  }

  if (rating < 5 && (reviewText.length < 1 || reviewText.length > 256)) {
    setStatus({
      isError: true,
      message: "El texto debe tener entre 1 y 256 caracteres.",
    });
    setOpenSnack(true);
    return;
  }

  setSubmitting(true);
  try {
    const payload = {
      access: user.access,
      rental_id: selectedRental.rental_id,
      rating,
      content: rating === 5 ? "" : reviewText,
    };

    const response = await post("/reviews/machines/new", payload);

    if (response.status === 201) {
      setStatus({ isError: false, message: "Valoración registrada con éxito." });
      setOpenSnack(true);

      setRentalsData((prev) =>
        prev.map((r) =>
          r.rental_id === selectedRental.rental_id
            ? { ...r, has_reviewed: true }
            : r
        )
      );

      closeModal();
    }
  } catch (error) {
    let message = "Error inesperado";
    if (error.response) {
      switch (error.response.status) {
        case 400:
          message = error.response.data?.message || "Datos inválidos";
          break;
        case 401:
          message = "Token inválido.";
          break;
        case 403:
          message = "No sos cliente.";
          break;
        case 500:
          message = "Error interno del servidor.";
          break;
      }
    }
    setStatus({ isError: true, message });
    setOpenSnack(true);
  } finally {
    setSubmitting(false);
  }
};


  return (
    <>
      <Sheet
        sx={{
          width: "80%",
          minWidth: "800px",
          height: "100%",
          backgroundColor: "white",
          boxShadow: "0 1px 4px rgba(0, 0, 0, 0.1)",
        }}
      >
        <Stack spacing={2} sx={{ padding: 5 }}>
          <Typography level="h2">Mis alquileres</Typography>
          {rentalsData.length === 0 ? (
            <Typography level="body-lg" color="text.secondary">
              Parece que no tenes alquileres registrados. Podes alquilar una
              maquina desde nuestro{" "}
              <Link component={RouterLink} to={"/explore"}>
                catalogo
              </Link>
              .
            </Typography>
          ) : (
          <Stack spacing={2}>
            {rentalsData.map((rental) => (
              <Stack
                key={rental.rental_id}
                direction="row"
                alignItems="center"
                spacing={2}
              >
                <MyRentalCard
                  rentalID={rental.rental_id}
                  imageUrl={rental.model_image}
                  modelName={rental.model_name}
                  withdrawnDate={rental.retirement_date}
                  startDate={rental.start_date}
                  endDate={rental.end_date}
                  amountPaid={rental.total_price}
                  days={
                    rental.start_date && rental.end_date
                      ? Math.round(
                          (new Date(rental.end_date) - new Date(rental.start_date)) /
                            (1000 * 60 * 60 * 24)
                        )
                      : "-"
                  }
                  status={rental.status}
                />
                {rental.status === "completed" && !rental.has_reviewed && (
                  <Button
                    variant="outlined"
                    size="sm"
                    color="danger"
                    onClick={() => openRatingModal(rental)}
                    sx={{ whiteSpace: "nowrap" }}
                  >
                    Valorar máquina
                  </Button>
                )}
              </Stack>
            ))}
          </Stack>
          )}
        </Stack>
      </Sheet>
      <Snackbar
        variant="soft"
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
        autoHideDuration={3000}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        {status.message}
      </Snackbar>

  <Modal
  open={modalOpen}
  onClose={closeModal}
  sx={{ backdropFilter: "blur(5px)", display: "flex", justifyContent: "center", alignItems: "center", p: 2 }}
>
  <Sheet
    variant="outlined"
    sx={{ maxWidth: 400, width: "100%", p: 3, borderRadius: 2, boxShadow: "lg", bgcolor: "background.body" }}
  >
    <Typography level="h4" mb={2}>
      Valorar máquina
    </Typography>

    <Select
      label="Estrellas"
      value={rating.toString()}
      onChange={(e, newValue) => setRating(Number(newValue))}
      size="md"
      sx={{ mb: 2 }}
    >
      {["1", "2", "3", "4", "5"].map((num) => (
        <Option key={num} value={num}>
          {"★ " + num}
        </Option>
      ))}
    </Select>

    {(rating < 5 || rating === 5) && (
      <Textarea
        placeholder="Escribe tu reseña (1 a 256 caracteres) (opcional para 5 estrellas)"
        maxLength={256}
        minRows={3}
        value={reviewText}
        onChange={(e) => setReviewText(e.target.value)}
        required={rating < 5} // obligatorio si rating < 5, opcional si es 5
        sx={{ mb: 2 }}
      />
    )}


    <Stack direction="row" spacing={2} justifyContent="flex-end">
      <Button variant="plain" color="neutral" onClick={closeModal} disabled={submitting}>
        Cancelar
      </Button>
      <Button variant="solid" onClick={handleSubmit} disabled={submitting} color="danger">
        {submitting ? "Enviando..." : "Valorar máquina"}
      </Button>
    </Stack>
  </Sheet>
</Modal>
    </>
  );
};

export default MyRentalsPage;
