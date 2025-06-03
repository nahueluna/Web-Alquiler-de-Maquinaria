import { Link, Sheet, Stack, Typography, Snackbar } from "@mui/joy";
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
            <Stack spacing={1}>
              {rentalsData.map((rental, index) => (
                <MyRentalCard
                  key={rental.rental_id}
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
                          (new Date(rental.end_date) -
                            new Date(rental.start_date)) /
                            (1000 * 60 * 60 * 24)
                        )
                      : "-"
                  }
                  status={rental.status}
                />
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
    </>
  );
};

export default MyRentalsPage;
