import {
  AspectRatio,
  Box,
  Button,
  Card,
  CardContent,
  CardOverflow,
  Chip,
  Divider,
  Modal,
  Sheet,
  Stack,
  Typography,
} from "@mui/joy";
import React from "react";

const MyRentalCard = ({
  rentalID,
  imageUrl,
  modelName,
  withdrawnDate,
  startDate,
  endDate,
  amountPaid,
  days,
  status,
}) => {
  const [open, setOpen] = React.useState(false);
  const [selectedRentalID, setSelectedRentalID] = React.useState(null);

  const handleConfirmedCancel = (rentalID) => {
    console.log(`Canceling rental with ID: ${rentalID}`);
    // CONECTAR A LA API
    setOpen(false);
  };
  return (
    <>
      <Card variant="outlined" sx={{ width: "60%", minWidth: "700px" }}>
        <CardOverflow variant="soft">
          <Stack
            direction="row"
            sx={{ py: 1, justifyContent: "space-between" }}
          >
            <Typography fontWeight="bold">ID {rentalID}</Typography>
            <Typography>{startDate}</Typography>
          </Stack>
          <Divider inset="context" />
        </CardOverflow>
        <CardContent variant="solid">
          <Stack
            direction="row"
            spacing={2}
            sx={{
              width: "100%",
              height: "100%",
              justifyContent: "space-between",
            }}
          >
            <Stack direction="row" spacing={2}>
              <AspectRatio ratio="1" sx={{ width: 90 }}>
                <img src={imageUrl} />
              </AspectRatio>
              <Box>
                <Typography level="title-lg">{modelName}</Typography>
                <Typography level="body-md" color="neutral">
                  {withdrawnDate
                    ? `Retirado el ${withdrawnDate}`
                    : "No retirado"}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Fecha de inicio: {startDate}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Fecha de devolucion: {endDate}
                </Typography>
                <Typography level="body-md" color="neutral">
                  Precio de alquiler:
                  <Typography fontWeight="bold"> ${amountPaid}</Typography>
                  <Typography level="body-md" color="neutral">
                    {" "}
                    ({days} días)
                  </Typography>
                </Typography>
              </Box>
            </Stack>
            <Stack
              spacing={1}
              alignItems="flex-end"
              justifyContent="space-between"
              sx={{ minHeight: "100%" }}
            >
              <Chip variant="solid" color="danger" size="lg">
                {status}
              </Chip>
              {status === "Activo" && (
                <Button
                  variant="outlined"
                  color="danger"
                  size="lg"
                  onClick={() => {
                    setOpen(true);
                    setSelectedRentalID(rentalID);
                  }}
                >
                  Cancelar alquiler
                </Button>
              )}
            </Stack>
          </Stack>
        </CardContent>
      </Card>
      <Modal
        open={open}
        onClose={() => setOpen(false)}
        sx={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Sheet
          variant="outlined"
          sx={{ p: 2, width: "400px", borderRadius: "lg" }}
        >
          <Stack spacing={2} sx={{ p: 1 }}>
            <Typography level="title-lg">
              ¿Estás seguro de que queres cancelar el alquiler?
            </Typography>
            <Typography level="body-md">
              La devolucion del monto abonado esta sujeta a las condiciones de
              la empresa.
            </Typography>
            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button
                variant="soft"
                color="neutral"
                onClick={() => setOpen(false)}
              >
                Volver
              </Button>
              <Button
                variant="solid"
                color="danger"
                onClick={() => handleConfirmedCancel(selectedRentalID)}
              >
                Cancelar alquiler
              </Button>
            </Stack>
          </Stack>
        </Sheet>
      </Modal>
    </>
  );
};

export default MyRentalCard;
