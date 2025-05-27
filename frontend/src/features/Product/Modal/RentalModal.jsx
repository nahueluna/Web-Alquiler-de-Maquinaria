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
import { useState } from "react";
import Check from "@mui/icons-material/Check";
import Location from "./Location";

// const today = new Date();
// const yyyy = today.getFullYear();
// const mm = String(today.getMonth() + 1).padStart(2, "0");
// const dd = String(today.getDate()).padStart(2, "0");
// const todayStr = `${yyyy}-${mm}-${dd}`;

// const oneWeekLater = new Date(today);
// oneWeekLater.setDate(today.getDate() + 7);

// const yyyy2 = oneWeekLater.getFullYear();
// const mm2 = String(oneWeekLater.getMonth() + 1).padStart(2, "0");
// const dd2 = String(oneWeekLater.getDate()).padStart(2, "0");
// const oneWeekLaterStr = `${yyyy2}-${mm2}-${dd2}`;

function RentalModal({ setOpen, open, locations }) {
  //   const [endDate, setEndDate] = useState(null);
  const [activeStep, setActiveStep] = useState(0);
  const steps = [
    {
      name: "Elegir ubicacion",
      component: <Location locations={locations} />,
    },
    { name: "Elegir duracion", component: <div>test</div> },
    { name: "Realizar pago", component: "" },
  ];

  function handleBack() {
    if (activeStep === 0) {
      setOpen(false);
    } else {
      setActiveStep((prev) => prev - 1);
    }
  }

  function handleNext() {
    console.log("Test", activeStep);
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
        sx={{ width: 600, borderRadius: "md", p: 3, boxShadow: "lg" }}
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
                  {activeStep <= i ? i + 1 : <Check />}
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
          <Button onClick={handleNext} variant="solid" color="danger">
            Siguiente
          </Button>
        </Box>
      </Sheet>
    </Modal>
  );
}

export default RentalModal;
