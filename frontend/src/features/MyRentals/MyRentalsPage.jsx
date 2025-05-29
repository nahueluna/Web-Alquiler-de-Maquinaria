import { Link, Sheet, Stack, Typography } from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import MyRentalCard from "./MyRentalCard";

const rentalsData = [];

const MyRentalsPage = () => {
  return (
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
            Oopsie. Parece que no tenes alquileres registrados. Sumale una magia
            mas alquilando una maquina de nuestro{" "}
            <Link component={RouterLink} to={"/explore"}>
              catalogo
            </Link>
            .
          </Typography>
        ) : (
          <Stack spacing={1}>
            {rentalsData.map((rental, index) => (
              <MyRentalCard
                key={rental.rentalID}
                rentalID={rental.rentalID}
                imageUrl={rental.imageUrl}
                modelName={rental.modelName}
                withdrawnDate={rental.withdrawnDate}
                startDate={rental.startDate}
                endDate={rental.endDate}
                amountPaid={rental.amountPaid}
                days={rental.days}
                status={rental.status}
              />
            ))}
          </Stack>
        )}
      </Stack>
    </Sheet>
  );
};

export default MyRentalsPage;
