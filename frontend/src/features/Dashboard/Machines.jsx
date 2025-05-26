import SearchIcon from "@mui/icons-material/Search";
import { Button, FormControl, Input, Sheet, Stack } from "@mui/joy";
import MachinesList from "./MachinesList";
import RegisterMachineForm from "./RegisterMachineForm";

import { useState } from "react";

const Machines = () => {
  const [registerForm, setRegisterForm] = useState(false);

  return (
    <Sheet
      sx={{
        display: "flex",
        justifyContent: "flex-start",
        alignItems: "center",
        height: "100%",
        width: "70%",
      }}
    >
      <Stack spacing={4} sx={{ padding: 2, width: "100%", height: "100%" }}>
        <Stack direction={"row"} spacing={2}>
          <FormControl sx={{ width: "350px" }}>
            <Input
              endDecorator={<SearchIcon />}
              placeholder="Buscar por modelo..."
            />
          </FormControl>
          <Button color="danger" onClick={() => setRegisterForm(true)}>
            Registrar nuevo modelo
          </Button>
        </Stack>
        {registerForm && (
          <RegisterMachineForm setRegisterForm={setRegisterForm} />
        )}
        <MachinesList />
      </Stack>
    </Sheet>
  );
};

export default Machines;
