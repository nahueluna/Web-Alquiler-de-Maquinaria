import React from "react";
import {
  Box,
  Stack,
  Input,
  FormLabel,
  FormHelperText,
  FormControl,
  Button,
  Typography,
  Card,
} from "@mui/joy";
import { useEffect, useState } from "react";
import useAuth from "../../utils/useAuth";

const SelectPeriod = ({ unitId, setDisable, setValidPeriod }) => {
  const { post } = useAuth();

  const [fechaInicio, setFechaInicio] = useState("");
  const [fechaFin, setFechaFin] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [overlapedDates, setOverlapedDates] = useState({
    start_date: "",
    end_date: "",
  });
  const [validDates, setValidDates] = useState({
    start_date: "",
    end_date: "",
  });

  function formatDateDMY(dateStr) {
    if (!dateStr) return "";
    const [year, month, day] = dateStr.split("-");
    return `${day}/${month}/${year}`;
  }

  async function validateDates() {
    setLoading(true);
    try {
      const response = await post(`/staff/rental/validatedates`, {
        unit_id: Number(unitId),
        start_date: fechaInicio,
        end_date: fechaFin,
      });

      console.log("Response:", response.data);
      setValidDates({
        start_date: fechaInicio,
        end_date: fechaFin,
      });
      setValidPeriod({
        start_date: fechaInicio,
        end_date: fechaFin,
      });
      setOverlapedDates({
        start_date: "",
        end_date: "",
      });
      setDisable(false);
    } catch (error) {
      console.error("Error validating dates:", error);
      switch (error.response?.status) {
        case 400:
          setError("Las fechas no son válidas o el periodo es menor a 7 dias.");
          break;
        case 409:
          if (error.response.data) {
            setError(error.response.data.message);
            setOverlapedDates({
              start_date: error.response.data.overlaped_date.start_date,
              end_date: error.response.data.overlaped_date.end_date,
            });
          }
      }
    }

    setLoading(false);
  }

  const handleSubmit = () => {
    validateDates();
  };

  useEffect(() => {
    setDisable(true);
    return () => setDisable(false);
  }, []);

  useEffect(() => {
    if (fechaFin && fechaFin < fechaInicio) {
      setError("La fecha de fin no puede ser anterior a la de inicio.");
      setDisable(true);
      return;
    }
    const diffDays =
      (new Date(fechaFin) - new Date(fechaInicio)) / (1000 * 60 * 60 * 24);
    if (diffDays < 7) {
      setError("El periodo debe ser de al menos 7 días.");
      setDisable(true);
      return;
    }
    setError("");
  }, [fechaInicio, fechaFin]);

  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        flexDirection: "column",
        width: "100%",
        gap: 2,
      }}
    >
      <Box>
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Box>
            <FormLabel size="lg">Periodo de alquiler</FormLabel>
            <Stack direction="row" spacing={1}>
              <FormControl error={!!error && (fechaInicio || fechaFin)}>
                <Input
                  type="date"
                  value={fechaInicio}
                  disabled={loading}
                  size="lg"
                  slotProps={{
                    input: {
                      max: "9999-12-31",
                    },
                  }}
                  onChange={(e) => setFechaInicio(e.target.value)}
                />
              </FormControl>
              <FormControl error={!!error && (fechaInicio || fechaFin)}>
                <Input
                  type="date"
                  value={fechaFin}
                  disabled={loading}
                  size="lg"
                  slotProps={{
                    input: {
                      max: "9999-12-31",
                    },
                  }}
                  onChange={(e) => setFechaFin(e.target.value)}
                />
              </FormControl>
              <Button
                size="lg"
                color="danger"
                onClick={handleSubmit}
                disabled={!fechaFin || !fechaInicio || error}
              >
                Validar
              </Button>
            </Stack>
          </Box>
          {error && (fechaFin || fechaInicio) && (
            <FormHelperText
              sx={{
                color: "var(--joy-palette-danger-plainColor)",
                width: "80%",
              }}
            >
              {error}
            </FormHelperText>
          )}
        </Box>
      </Box>
      {overlapedDates.start_date && overlapedDates.end_date ? (
        <Card variant="soft">
          <Typography>
            Oops! Parece que las fechas que indicaste estan chocando con un
            alquiler existente entre{" "}
            <strong>{formatDateDMY(overlapedDates.start_date)}</strong> y{" "}
            <strong>{formatDateDMY(overlapedDates.end_date)}</strong>
          </Typography>
        </Card>
      ) : validDates.start_date && validDates.end_date ? (
        <Card variant="soft">
          <Typography>
            BINGO! Podes realizar un alquiler para el periodo de{" "}
            <strong>{formatDateDMY(validDates.start_date)}</strong> a{" "}
            <strong>{formatDateDMY(validDates.end_date)}</strong>
          </Typography>
        </Card>
      ) : null}
    </Box>
  );
};

export default SelectPeriod;
