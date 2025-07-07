import React from "react";
import { Box, Typography } from "@mui/joy";
import { Table } from "@mui/joy";

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

function formatARS(amount) {
  if (isNaN(amount)) return "";
  return `ARS $${amount.toLocaleString("en-US", {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  })}`;
}

function formatDays(amount) {
  if (isNaN(amount)) return "";
  return `${amount.toLocaleString("en-US", {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  })}`;
}

const InpersonSummary = ({ selectedCity, unitId, validPeriod, machine }) => {
  const days = getDaysBetween(validPeriod);
  return (
    <Box>
      <Table
        sx={{
          width: "100%",
        }}
        stripe={"odd"}
        borderAxis="none"
      >
        <tbody>
          <tr>
            <td>Maquina</td>
            <td>
              {machine.name} {machine.model}
            </td>
          </tr>
          <tr>
            <td>ID de ejemplar</td> <td>{unitId}</td>
          </tr>
          <tr>
            <td>Ubicacion</td> <td>{selectedCity}</td>
          </tr>
          <tr>
            <td>Fecha de inicio</td>{" "}
            <td>{formatDateDMY(validPeriod.start_date)}</td>
          </tr>
          <tr>
            <td>Fecha de fin</td> <td>{formatDateDMY(validPeriod.end_date)}</td>
          </tr>
          <tr>
            <td>Duracion total</td> <td>{formatDays(days)} dias</td>
          </tr>
          <tr>
            <td>Precio total</td>
            <td>{formatARS(machine.price * days)}</td>
          </tr>
        </tbody>
      </Table>
    </Box>
  );
};

export default InpersonSummary;
