import React from "react";
import { Box, Typography } from "@mui/joy";

function formatDateDMY(dateStr) {
  if (!dateStr) return "";
  const [year, month, day] = dateStr.split("-");
  return `${day}/${month}/${year}`;
}

function getDaysBetween(period) {
  if (!period?.start_date || !period?.end_date) return 0;
  const start = new Date(period.start_date + "T00:00:00");
  const end = new Date(period.end_date + "T00:00:00");
  const diffMs = end - start;
  return diffMs >= 0 ? Math.ceil(diffMs / (1000 * 60 * 60 * 24)) : 0;
}

const InpersonSummary = ({ userId, machineId, validPeriod, price }) => {
  const days = getDaysBetween(validPeriod);
  return (
    <Box sx={{ p: 4 }}>
      <Typography level="h2">A ver a ver, dejame ver si entiendo...</Typography>
      <Typography variant="body1">
        Estas queriendo registrar un alquiler presencial para el usuario ID{" "}
        <strong>{userId}</strong>, sobre el ejemplar ID{" "}
        <strong>{machineId}</strong> y para el periodo{" "}
        <strong>
          {formatDateDMY(validPeriod.start_date)} -{" "}
          {formatDateDMY(validPeriod.end_date)}
        </strong>
        ..
      </Typography>
      <Typography variant="body1" sx={{ mt: 2 }}>
        Tenes idea de cuanto le vas a cobrar? Mira que son{" "}
        <strong>{days}</strong> dias...{" "}
        {days > 365 ? "ah mierda, mas que un año! " : ""}
      </Typography>
      <Typography variant="body1" sx={{ mt: 2 }}>
        Si.... son {price * days} pesos
      </Typography>
    </Box>
  );
};

export default InpersonSummary;
