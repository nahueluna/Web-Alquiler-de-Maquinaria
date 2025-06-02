import { Box, Button, Chip, Sheet, Stack, Typography } from "@mui/joy";

export default function RentalCard({
  rentalNumber,
  status,
  startDate,
  endDate,
  paidAmount,
}) {
  return (
    <Sheet
      variant="outlined"
      sx={{
        minWidth: { xs: "90vw", sm: 320, md: 400 },
        maxWidth: 800,
        width: "100%",
        borderRadius: "md",
        padding: 2,
        boxSizing: "border-box",
        // Elimina height fijo para que el contenido fluya
        display: "flex",
        flexDirection: "column",
        gap: 2,
      }}
    >
      <Stack spacing={2}>
        <Box
          sx={{
            display: "flex",
            justifyContent: "space-between",
            flexWrap: "wrap",
            gap: 1,
          }}
        >
          <Chip variant="outlined" color="danger" size="lg">
            ID {rentalNumber}
          </Chip>
          <Chip variant="solid" color="danger" size="lg">
            {status}
          </Chip>
        </Box>
        <Box>
          <Typography level={5}>Desde {startDate}</Typography>
          <Typography level={5}>Hasta {endDate}</Typography>
        </Box>
        <Stack direction="row" spacing={2} flexWrap="wrap" alignItems="center">
          <Typography>Fecha de retiro: - </Typography>
          <Button color="danger" size="xs" variant="solid">
            Cargar retiro
          </Button>
        </Stack>
        <Stack direction="row" spacing={2} flexWrap="wrap" alignItems="center">
          <Typography>Fecha de devolucion: - </Typography>
          <Button color="danger" size="xs" variant="solid">
            Cargar devolucion
          </Button>
        </Stack>
        <Typography>Monto abonado: {paidAmount}</Typography>
        <Button
          color="danger"
          size="lg"
          variant="soft"
          sx={{ width: { xs: "100%", sm: "40%" } }}
        >
          Cancelar alquiler
        </Button>
      </Stack>
    </Sheet>
  );
}
