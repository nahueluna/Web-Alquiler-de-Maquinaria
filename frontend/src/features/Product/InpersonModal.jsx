import React from "react";
import {
  Box,
  Button,
  Divider,
  Modal,
  ModalClose,
  Sheet,
  Step,
  StepIndicator,
  Stepper,
  Typography,
} from "@mui/joy";
import { useReducer, useState } from "react";
import useAuth from "../utils/useAuth";
import SelectUser from "./InpersonModal/SelectUser";
import SelectMachine from "./InpersonModal/SelectMachine";
import SelectPeriod from "./InpersonModal/SelectPeriod";
import InpersonSummary from "./InpersonModal/InpersonSummary";
import Check from "@mui/icons-material/Check";

/*function reducer(state, action) {
  switch (action.type) {
    case "setLocation":
      return {
        ...state,
        selectedLocation: action.value,
      };

    case "setDates": {
      let [start, end] = action.value;
      let ms = new Date(end).getTime() - new Date(start).getTime();
      let days = ms / (1000 * 60 * 60 * 24);

      return {
        ...state,
        dates: action.value,
        days: days,
      };
    }

    case "setUnitId":
      return {
        ...state,
        unitId: action.value,
      };

    case "clear":
      return {
        machine: state.machine,
        selectedLocation: {},
        dates: [],
        days: 0,
        unitId: 0,
      };
  }
}*/

function getDaysBetween(period) {
  if (!period?.start_date || !period?.end_date) return 0;
  const start = new Date(period.start_date + "T00:00:00");
  const end = new Date(period.end_date + "T00:00:00");
  const diffMs = end - start;
  return diffMs >= 0 ? Math.ceil(diffMs / (1000 * 60 * 60 * 24)) : 0;
}

const InpersonModal = ({
  id,
  open,
  setOpen,
  setOpenSnack,
  setStatus,
  machine,
}) => {
  console.log("InpersonModal machine:", machine);
  const { post } = useAuth();

  const [activeStep, setActiveStep] = useState(0);
  const [loading, setloading] = useState(false);
  const [disable, setDisable] = useState(true);
  const [userId, setUserId] = useState(null);
  const [selectedCity, setSelectedCity] = useState("");
  const [unitId, setUnitId] = useState(null);
  const [validPeriod, setValidPeriod] = useState({
    start_date: "",
    end_date: "",
  });
  const [error, setError] = useState("");

  const steps = [
    {
      name: "Seleccionar usuario",
      component: <SelectUser setDisable={setDisable} setUserId={setUserId} />,
    },
    {
      name: "Elegir ubicacion y ejemplar",
      component: (
        <SelectMachine
          machineId={id}
          setDisable={setDisable}
          setSelectedCity={setSelectedCity}
          setUnitId={setUnitId}
        />
      ),
    },
    {
      name: "Indicar periodo",
      component: (
        <SelectPeriod
          unitId={unitId}
          setDisable={setDisable}
          setValidPeriod={setValidPeriod}
        />
      ),
    },
    {
      name: "Resumen",
      component: (
        <InpersonSummary
          userId={userId}
          selectedCity={selectedCity}
          unitId={unitId}
          validPeriod={validPeriod}
          days={getDaysBetween(validPeriod)}
          machine={machine}
        />
      ),
    },
  ];

  function handleBack() {
    if (activeStep === 0) {
      setOpen(false);
      setUserId(null);
      setLocationId(null);
      setUnitId(null);
    } else {
      setActiveStep((prev) => prev - 1);
    }
  }

  async function handleNext() {
    setActiveStep((prev) => prev + 1);
  }

  async function handleNewRental() {
    setloading(true);
    try {
      const response = await post(`/staff/rental/new`, {
        machine_id: unitId,
        user_id: userId,
        start_date: validPeriod.start_date,
        end_date: validPeriod.end_date,
        total_price: getDaysBetween(validPeriod) * machine.price,
      });
      setStatus({
        isError: false,
        message: "Alquiler registrado correctamente.",
      });
      setOpenSnack(true);
      setOpen(false);
      setActiveStep(0);
      setUserId(null);
      setSelectedCity(null);
      setUnitId(null);
      setValidPeriod({ start_date: "", end_date: "" });
      setError("");
      setDisable(true);
    } catch (error) {
      console.error("Error al registrar el alquiler:", error);
      let errorMessage =
        "Hubo un error al registrar el alquiler. Intentalo mas tarde.";
      switch (error.response?.status) {
        case 403:
          errorMessage = "No tenes permisos para registrar un alquiler.";
          break;
        case 404:
          errorMessage =
            "No se pudo encontrar el usuario o el usuario no es cliente.";
          break;
        case 409:
          errorMessage =
            "El periodo indicado se solapa con un alquiler existente.";
          break;
        case 400:
          errorMessage =
            "El precio es incorrecto o el periodo indicado no es valido.";
          break;
      }
      setStatus({
        isError: true,
        message: errorMessage,
      });
      setOpenSnack(true);
    }
    setloading(false);
  }

  return (
    <Modal
      aria-labelledby="modal-title"
      aria-describedby="modal-desc"
      open={open}
      onClose={() => setOpen(false)}
      sx={{ display: "flex", justifyContent: "center", alignItems: "center" }}
    >
      <Sheet
        variant="outlined"
        sx={{
          minWidth: 600,
          maxWidth: 714,
          borderRadius: "md",
          p: 3,
          boxShadow: "lg",
        }}
      >
        <ModalClose variant="plain" sx={{ m: 1 }} />
        <Typography
          component="h2"
          id="modal-title"
          level="h4"
          textColor="inherit"
          sx={{ fontWeight: "lg", mb: 1 }}
        >
          Registrar alquiler presencial
        </Typography>
        <Stepper sx={{ width: "100%" }}>
          {steps.map((step, i) => (
            <Step
              indicator={
                <StepIndicator
                  variant="soft"
                  color={
                    activeStep < i
                      ? "neutral"
                      : activeStep > i
                      ? "success"
                      : "danger"
                  }
                >
                  {activeStep <= i ? i + 1 : <Check sx={{ fontSize: "lg" }} />}
                </StepIndicator>
              }
              sx={[
                activeStep > i &&
                  i !== 2 && { "&::after": { bgcolor: "success.300" } },
              ]}
            >
              {step.name}
            </Step>
          ))}
        </Stepper>
        <Divider
          sx={{
            mt: 2,
            mb: 2,
          }}
        />
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            minHeight: "250px",
          }}
        >
          {steps[activeStep].component}
        </Box>
        <Box
          sx={{
            display: "flex",
            justifyContent: "space-between",
            mt: 2,
          }}
        >
          <Button onClick={handleBack} variant="plain" color="neutral">
            {activeStep === 0 ? "Cancelar" : "Atras"}
          </Button>

          {activeStep < 3 ? (
            <Button
              loading={loading}
              onClick={handleNext}
              disabled={disable}
              variant="solid"
              color="danger"
            >
              Siguiente
            </Button>
          ) : (
            <Button
              loading={loading}
              onClick={handleNewRental}
              disabled={disable}
              variant="solid"
              color="danger"
            >
              Confirmar alquiler presencial
            </Button>
          )}
        </Box>
      </Sheet>
    </Modal>
  );
};

export default InpersonModal;
