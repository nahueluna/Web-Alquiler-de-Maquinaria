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
      }}
    >
      <Stack spacing={4} sx={{ padding: 2, maxWidth: "80%" }}>
        <FormControl sx={{ width: "350px" }}>
          <Input
            endDecorator={<SearchIcon />}
            placeholder="Buscar por numero de alquiler.."
          ></Input>
        </FormControl>
        <RentalCard
          model="HGG-345"
          renter="Carlitos Marado"
          startDate="2022-04-16"
          finished={false}
        />
      </Stack>
    </Sheet>
  );
};

export default Rentals;
