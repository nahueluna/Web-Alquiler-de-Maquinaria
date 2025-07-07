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
  //Valorar maquina
  const [modalOpen, setModalOpen] = useState(false);
  const [selectedRental, setSelectedRental] = useState(null);
  const [rating, setRating] = useState(5);
  const [reviewText, setReviewText] = useState("");
  const [submitting, setSubmitting] = useState(false);
  //Valorar servicio
  const [serviceModalOpen, setServiceModalOpen] = useState(false);
  const [selectedServiceRental, setSelectedServiceRental] = useState(null);
  const [serviceRating, setServiceRating] = useState(5);
  const [serviceReviewText, setServiceReviewText] = useState("");
  const [submittingService, setSubmittingService] = useState(false);


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
    setStatus({ isError: true, message: "Rating inválido." });
    setOpenSnack(true);
    return;
  }

  if (rating < 5 && reviewText.trim().length === 0) {
    setStatus({
      isError: true,
      message: "La reseña no puede estar vacía.",
    });
    setOpenSnack(true);
    return;
  }

  if (reviewText.length > 256) {
    setStatus({ isError: true, message: "La reseña no puede superar los 256 caracteres." });
    setOpenSnack(true);
    return;
  }

  setSubmitting(true);
  try {
    const payload = {
      access: user.access,
      rental_id: selectedRental.rental_id,
      rating,
      content: reviewText,
    };

    const response = await post("/reviews/machines/new", payload);

    if (response.status === 201) {
      setStatus({ isError: false, message: "Valoración registrada con éxito." });
      setOpenSnack(true);

      setRentalsData((prev) =>
        prev.map((r) =>
          r.rental_id === selectedRental.rental_id
            ? { ...r, has_machine_review: true }
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
          message = "Ya se ha realizado una valoración para esta máquina.";;
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

  const openServiceRatingModal = (rental) => {
    setSelectedServiceRental(rental);
    setServiceRating(5);
    setServiceReviewText("");
    setServiceModalOpen(true);
  };

  const closeServiceModal = () => {
    if (!submittingService) {
      setServiceModalOpen(false);
      setSelectedServiceRental(null);
    }
  };

const handleSubmitServiceReview = async () => {
  if (!selectedServiceRental) return;

  // Validaciones locales
  if (serviceRating < 1 || serviceRating > 5) {
    setStatus({ isError: true, message: "Rating inválido." });
    setOpenSnack(true);
    return;
  }

  if (serviceReviewText.length > 256) {
    setStatus({
      isError: true,
      message: "La reseña no puede superar los 256 caracteres.",
    });
    setOpenSnack(true);
    return;
  }

  if (serviceRating < 5 && serviceReviewText.trim().length === 0) {
    setStatus({
      isError: true,
      message: "La reseña no puede estar vacía.",
    });
    setOpenSnack(true);
    return;
  }

  setSubmittingService(true);

  try {
    const payload = {
      access: user.access,
      rental_id: selectedServiceRental.rental_id,
      rating: serviceRating,
      content: serviceReviewText.trim(),
    };

    const response = await post("/reviews/service/new", payload);

    if (response.status === 201) {
      setStatus({
        isError: false,
        message: "Valoración de servicio registrada con éxito.",
      });
      setOpenSnack(true);

      setRentalsData((prev) =>
        prev.map((r) =>
          r.rental_id === selectedServiceRental.rental_id
            ? { ...r, has_service_review: true }
            : r
        )
      );

      closeServiceModal();
    }
  } catch (error) {
    let message = "Error inesperado";
    if (error.response) {
      switch (error.response.status) {
        case 400:
          message =
            "Ya se ha realizado una valoración del servicio para este alquiler.";
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
    setSubmittingService(false);
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
                direction="column"
                alignItems="flex-start"
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
                <Stack direction="row" spacing={2}>
                  {rental.status === "completed" && !rental.has_machine_review && (
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
                  {rental.status === "completed" && !rental.has_service_review && (
                  <Button
                    variant="outlined"
                    size="sm"
                    color="danger"
                    onClick={() => openServiceRatingModal(rental)} // aún hay que definir esta función
                    sx={{ whiteSpace: "nowrap" }}
                  >
                    Valorar servicio
                  </Button>
                )}
                </Stack>
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
  sx={{
    backdropFilter: "blur(5px)",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    p: 2,
  }}
>
  <Sheet
    variant="outlined"
    sx={{
      maxWidth: 440,
      width: "100%",
      p: 4,
      borderRadius: "lg",
      boxShadow: "xl",
      bgcolor: "background.surface",
    }}
  >
    <Typography level="h4" fontWeight="xl" textAlign="center" mb={2}>
      Valorar máquina
    </Typography>

    <Select
      label="Estrellas"
      value={rating.toString()}
      onChange={(e, newValue) => setRating(Number(newValue))}
      size="md"
      sx={{ mb: 3 }}
    >
      {["1", "2", "3", "4", "5"].map((num) => (
        <Option key={num} value={num}>
          {"⭐ " + num}
        </Option>
      ))}
    </Select>

    <Typography level="body-sm" sx={{ mb: 1, color: "text.secondary" }}>
      Dejá tu reseña (opcional si calificás con 5 estrellas)
    </Typography>

    <Textarea
      placeholder="Escribí tu opinión (máx. 256 caracteres)"
      maxLength={256}
      minRows={4}
      value={reviewText}
      onChange={(e) => setReviewText(e.target.value)}
      required={rating < 5}
      sx={{ mb: 3 }}
    />

    <Stack direction="row" spacing={2} justifyContent="flex-end">
      <Button
        variant="plain"
        color="neutral"
        onClick={closeModal}
        disabled={submitting}
      >
        Cancelar
      </Button>
      <Button
        variant="solid"
        onClick={handleSubmit}
        disabled={submitting}
        color="danger"
      >
        {submitting ? "Enviando..." : "Valorar máquina"}
      </Button>
    </Stack>
  </Sheet>
</Modal>


<Modal
  open={serviceModalOpen}
  onClose={closeServiceModal}
  sx={{
    backdropFilter: "blur(5px)",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    p: 2,
  }}
>
  <Sheet
    variant="outlined"
    sx={{
      maxWidth: 440,
      width: "100%",
      p: 4,
      borderRadius: "lg",
      boxShadow: "xl",
      bgcolor: "background.surface",
    }}
  >
    <Typography level="h4" fontWeight="xl" textAlign="center" mb={2}>
      Valorar servicio
    </Typography>

    <Select
      label="Estrellas"
      value={serviceRating.toString()}
      onChange={(e, newValue) => setServiceRating(Number(newValue))}
      size="md"
      sx={{ mb: 3 }}
    >
      {["1", "2", "3", "4", "5"].map((num) => (
        <Option key={num} value={num}>
          {"⭐ " + num}
        </Option>
      ))}
    </Select>

    <Typography level="body-sm" sx={{ mb: 1, color: "text.secondary" }}>
      Dejá tu reseña (opcional si calificás con 5 estrellas)
    </Typography>

    <Textarea
      placeholder="Escribí tu opinión (máx. 256 caracteres)"
      maxLength={256}
      minRows={4}
      value={serviceReviewText}
      onChange={(e) => setServiceReviewText(e.target.value)}
      required={serviceRating < 5}
      sx={{ mb: 3 }}
    />

    <Stack direction="row" spacing={2} justifyContent="flex-end">
      <Button
        variant="plain"
        color="neutral"
        onClick={closeServiceModal}
        disabled={submittingService}
      >
        Cancelar
      </Button>
      <Button
        variant="solid"
        color="danger"
        onClick={handleSubmitServiceReview}
        disabled={submittingService}
      >
        {submittingService ? "Enviando..." : "Valorar servicio"}
      </Button>
    </Stack>
  </Sheet>
</Modal>

    </>
  );
};

export default MyRentalsPage;
