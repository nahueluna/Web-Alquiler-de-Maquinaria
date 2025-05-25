import SearchIcon from "@mui/icons-material/Search";
import { FormControl, Input, Sheet, Stack } from "@mui/joy";
import RentalCard from "./RentalCard";

const Rentals = () => {
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
          <Input
            endDecorator={<SearchIcon />}
            placeholder="Buscar por numero de alquiler.."
          ></Input>
        </FormControl>
        <RentalCard
          rentalNumber="223"
          model="HGG-345"
          renter="Carlitos Marado"
          startDate="2022-04-16"
          endDate="2022-04-20"
          status="active"
          paidAmount="234.423,33"
        />
      </Stack>
    </Sheet>
  );
};

export default Rentals;
