import { initMercadoPago, Wallet } from "@mercadopago/sdk-react";
import Check from "@mui/icons-material/Check";
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
import useAuth from "../../utils/useAuth";
import Duration from "./Duration";
import Location from "./Location";
import Summary from "./Summary";
import axios from "axios";

initMercadoPago(import.meta.env.VITE_MP_PUBLIC);

function reducer(state, action) {
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
}

function RentalModal({ setOpen, open, machine, locations }) {
  const [activeStep, setActiveStep] = useState(0);
  const [mlId, setMlId] = useState("");
  const [loadingMl, setLoadingMl] = useState(false);
  const [availability, setAvailability] = useState(null);
  const [disable, setDisable] = useState(false);

  const { post } = useAuth();
  const [state, dispatch] = useReducer(reducer, {
    machine,
    selectedLocation: {},
    dates: [],
    days: 0,
    unitId: 0,
  });

  const steps = [
    {
      name: "Elegir ubicacion",
      component: <Location dispatch={dispatch} locations={locations} />,
    },
    {
      name: "Elegir duracion",
      component: (
        <Duration
          availability={availability}
          setDisable={setDisable}
          dispatch={dispatch}
          loading={loadingMl}
        />
      ),
    },
    { name: "Realizar pago", component: <Summary info={state} /> },
  ];

  function handleBack() {
    if (activeStep === 0) {
      setOpen(false);
    } else {
      setActiveStep((prev) => prev - 1);
    }
  }

  async function handleNext() {
    if (activeStep === 0) {
      setLoadingMl(true);
      const {
        data: { units_and_their_unavailable_dates },
      } = await post(
        `/rental/availability?model_id=${state.machine.id}&location_id=${state.selectedLocation.id}`
      );

      setLoadingMl(false);
      setAvailability(units_and_their_unavailable_dates);
    } else if (activeStep === 1) {
      setLoadingMl(true);
      const { data } = await axios.post("http://localhost:3000/pago", state);

      setMlId(data.id);
      setLoadingMl(false);
    }
    setActiveStep((prev) => prev + 1);
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
          Realizar alquiler
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
        <Box>{steps[activeStep].component}</Box>
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

          {/* Show mp Wallet button when on the last step */}
          {activeStep === 2 ? (
            <Wallet
              // Create rental on the backend when the button is pressed
              onSubmit={async () => {
                const body = {
                  machine_id: state.unitId,
                  start_date: state.dates[0],
                  end_date: state.dates[1],
                  total_price: state.days * state.machine.price,
                };
                const { data } = await post("/rental/new", body);

                window.localStorage.setItem("rentalInfo", JSON.stringify(data));
              }}
              initialization={{ preferenceId: mlId }}
              customization={{
                theme: "dark",
                customStyle: {
                  hideValueProp: true,
                },
              }}
            />
          ) : (
            <Button
              loading={loadingMl}
              onClick={handleNext}
              disabled={disable}
              variant="solid"
              color="danger"
            >
              Siguiente
            </Button>
          )}
        </Box>
      </Sheet>
    </Modal>
  );
}

export default RentalModal;
