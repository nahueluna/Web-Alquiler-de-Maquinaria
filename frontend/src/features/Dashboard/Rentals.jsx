import SearchIcon from "@mui/icons-material/Search";
import { FormControl, Input, Sheet, Stack } from "@mui/joy";
import RentalCard from "./RentalCard";
import React, { use } from "react";
import { useEffect } from "react";
import useAuth from "../utils/useAuth";
import { useState } from "react";

const Rentals = () => {
  const [rentals, setRentals] = useState([]);
  const [value, setValue] = useState("");

  const { post } = useAuth();

  const fetchRentals = async (rentalId = null) => {
    try {
      const response = await post(`/staff/rentals?id=${rentalId || ""}`);
      setRentals(response.data.rentals);
    } catch (error) {
      console.error("Error fetching rentals:", error);
      setRentals([]);
    }
  };

  useEffect(() => {
    fetchRentals();
  }, []);

  const handleChange = (event) => {
    const cleaned = event.target.value.replace(/[^\d]/g, "");
    setValue(cleaned);
  };

  return (
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
              key={rental.rental_id}
              rentalNumber={rental.rental_id}
              model={rental.rental_model}
              startDate={rental.start_date}
              endDate={rental.end_date}
              status={rental.status}
              paidAmount={rental.total_price}
            />
          ))
        ) : (
          <p>No hay alquileres disponibles.</p>
        )}
      </Stack>
    </Sheet>
  );
};

export default Rentals;
