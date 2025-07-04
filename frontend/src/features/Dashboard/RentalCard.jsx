import { Box, Button, Chip, Sheet, Stack, Typography } from "@mui/joy";

const onSameDay = (startDate) => {
  const today = new Date();
  const start = new Date(startDate);
  return (
    today.getFullYear() === start.getFullYear() &&
    today.getMonth() === start.getMonth() &&
    today.getDate() === start.getDate()
  );
};

const isAfterEndDate = (endDate) => {
  const today = new Date();
  const end = new Date(endDate);
  today.setHours(0, 0, 0, 0);
  end.setHours(0, 0, 0, 0);
  return today > end;
};

const isBeforeStartDate = (startDate) => {
  const today = new Date();
  const start = new Date(startDate);
  today.setHours(0, 0, 0, 0);
  start.setHours(0, 0, 0, 0);
  return today < start;
};

const translateStatus = (status) => {
  switch (status) {
    case "pending_payment":
      return "Pago pendiente";
    case "active":
      return "Activo";
    case "cancelled":
      return "Cancelado";
    case "completed":
      return "Completado";
    case "failed":
      return "Fallido";
    default:
      return " ? ";
  }
};

const parseFullDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleDateString("es-AR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
};

const parseDate = (dateString) => {
  if (!dateString) return "";
  const [year, month, day] = dateString.split("-");
  return `${day}/${month}/${year}`;
};

export default function RentalCard({
  setOpen,
  setType,
  setRentalInfo,
  rentalId,
  modelName,
  modelModel,
  status,
  createdAt,
  startDate,
  endDate,
  totalPrice,
  retirementDate,
  returnDate,
  modelPolicy,
  daysLate,
  percentageLate,
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
        display: "flex",
        flexDirection: "column",
        gap: 2,
      }}
    >
      <Stack spacing={1}>
        <Box
          sx={{
            display: "flex",
            justifyContent: "space-between",
            flexWrap: "wrap",
            gap: 1,
          }}
        >
          <Chip variant="outlined" color="danger" size="lg">
            ID {rentalId}
          </Chip>
          <Typography level="body-lg">
            {modelName} {modelModel}
          </Typography>
          <Chip variant="solid" color="danger" size="lg">
            {translateStatus(status)}
          </Chip>
        </Box>
        <Typography level="body-sm">
          Creado el {parseFullDate(createdAt)}
        </Typography>
        <Box>
          <Typography>Inicio de alquiler: {parseDate(startDate)}</Typography>
          <Typography>
            Finalizacion acordada de alquiler: {parseDate(endDate)}
          </Typography>
          <Typography>Monto abonado: {totalPrice} pesos</Typography>
        </Box>
        {retirementDate ? (
          <Typography>Retirado el: {parseDate(retirementDate)}</Typography>
        ) : (
          <Stack direction="row" spacing={1}>
            <Typography>
              {isAfterEndDate(endDate) || status != "active"
                ? "No retirado"
                : "Retiro pendiente"}
            </Typography>
            {status === "active" &&
              !onSameDay(endDate) &&
              !isAfterEndDate(endDate) &&
              (onSameDay(startDate) || isAfterEndDate(startDate)) && (
                <Button
                  color="danger"
                  variant="plain"
                  size="xs"
                  onClick={() => {
                    setOpen(true);
                    setType("retirement");
                    setRentalInfo({
                      rentalId,
                      modelName,
                      modelModel,
                    });
                  }}
                >
                  Cargar retiro
                </Button>
              )}
          </Stack>
        )}

        {returnDate ? (
          <Typography>Devuelto el: {parseDate(returnDate)}</Typography>
        ) : (
          retirementDate &&
          status == "active" && (
            <Stack direction="row" spacing={1}>
              <Typography>Devolucion pendiente</Typography>
              <Button
                color="danger"
                variant="plain"
                size="xs"
                onClick={() => {
                  setOpen(true);
                  setType("return");
                  setRentalInfo({
                    rentalId,
                    modelName,
                    modelModel,
                  });
                }}
              >
                Cargar devolucion
              </Button>
            </Stack>
          )
        )}
        <Typography>Politica de cancelacion: {modelPolicy}</Typography>
        {(status === "pending_payment" ||
          (status === "active" &&
            !onSameDay(startDate) &&
            (isBeforeStartDate(startDate) || isAfterEndDate(endDate)))) && (
          <Button
            color="danger"
            size="lg"
            variant="soft"
            sx={{ width: { xs: "100%", sm: "40%", alignSelf: "center" } }}
            onClick={() => {
              setOpen(true);
              setType("cancel");
              setRentalInfo({
                rentalId,
                modelName,
                modelModel,
              });
            }}
          >
            Cancelar alquiler
          </Button>
        )}
      </Stack>
    </Sheet>
  );
}
