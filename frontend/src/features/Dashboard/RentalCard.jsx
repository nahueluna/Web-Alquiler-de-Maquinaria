import { Box, Button, Chip, Sheet, Stack, Typography } from "@mui/joy";

export default function RentalCard({
  rentalNumber,
  renter,
  status,
  startDate,
  endDate,
  paidAmount,
}) {
  return (
    <Sheet
      variant="outlined"
      sx={{
        minWidth: "400px",
        maxWidth: "800px",
        height: "50%",
        borderRadius: "md",
        padding: 2,
      }}
    >
      <Stack spacing={2}>
        <Box sx={{ display: "flex", justifyContent: "space-between" }}>
          <Chip variant="outlined" color="danger" size="lg">
            ID {rentalNumber}
          </Chip>
          <Chip variant="solid" color="danger" size="lg">
            {status}
          </Chip>
        </Box>
        <Typography level={5}>Alquilado por {renter}</Typography>
        <Box>
          <Typography level={5}>Desde {startDate}</Typography>
          <Typography level={5}>Hasta {endDate}</Typography>
        </Box>
        <Stack direction="row" spacing={2}>
          <Typography>Fecha de retiro: - </Typography>
          <Button color="danger" size="xs" variant="solid">
            Cargar retiro
          </Button>
        </Stack>
        <Stack direction="row" spacing={2}>
          <Typography>Fecha de devolucion: - </Typography>
          <Button color="danger" size="xs" variant="solid">
            Cargar devolucion
          </Button>
        </Stack>
        <Typography>Monto abonado: {paidAmount}</Typography>
        <Button color="danger" size="lg" variant="soft" sx={{ width: "40%" }}>
          Cancelar alquiler
        </Button>
      </Stack>
    </Sheet>
  );
}
