import SearchIcon from "@mui/icons-material/Search";
import { Button, FormControl, Input, Sheet, Snackbar, Stack } from "@mui/joy";
import MachinesList from "./MachinesList";
import RegisterMachineForm from "./RegisterMachineForm";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheck";

import { useEffect, useState } from "react";

const Machines = ({ categories }) => {
  const [registerForm, setRegisterForm] = useState(false);
  const [openSnack, setOpenSnack] = useState(false);
  const [refreshMachines, setRefreshMachines] = useState(false);
  const [status, setStatus] = useState({
    isError: false,
    message: "",
  });

  return (
    <>
      <Snackbar
        variant="soft"
        autoHideDuration={5000}
        color={status.isError ? "danger" : "success"}
        open={openSnack}
        onClose={() => setOpenSnack(false)}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
        startDecorator={
          status.isError ? (
            <ErrorOutlineIcon />
          ) : (
            <PlaylistAddCheckCircleRoundedIcon />
          )
        }
      >
        {status.message}
      </Snackbar>
      <Sheet
        sx={{
          display: "flex",
          justifyContent: "flex-start",
          alignItems: "center",
          height: "100%",
          width: "100%",
        }}
      >
        <Stack spacing={4} sx={{ padding: 2, width: "100%", height: "100%" }}>
          <Stack direction={"row"} spacing={2}>
            {/* 
              
            <FormControl>
              <Input
                endDecorator={<SearchIcon />}
                placeholder="Buscar por modelo..."
              />
            </FormControl>
              */}
            <Button color="danger" onClick={() => setRegisterForm(true)}>
              Registrar nuevo modelo
            </Button>
          </Stack>
          {registerForm && (
            <RegisterMachineForm
              categories={categories}
              setRegisterForm={setRegisterForm}
              setOpenSnack={setOpenSnack}
              setStatus={setStatus}
              setRefreshMachines={setRefreshMachines}
            />
          )}
          <MachinesList refreshMachines={refreshMachines} />
        </Stack>
      </Sheet>
    </>
  );
};

export default Machines;
