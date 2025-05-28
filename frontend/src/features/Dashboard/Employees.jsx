import SearchIcon from "@mui/icons-material/Search";
import { FormControl, Input, Sheet } from "@mui/joy";
import Button from "@mui/joy/Button";
import { Stack } from "@mui/material";
import { useState } from "react";
import AddEmployee from "../AddEmployee/AddEmployee";
import EmployeesList from "./EmployeesList";

const Employees = () => {
  const [registerForm, setRegisterForm] = useState(false);

  return (
    <Sheet sx={{ p: 2 }}>
      <Stack spacing={2} sx={{ width: "50%", minWidth: "500px" }}>
        <Stack direction="row" spacing={2}>
          <FormControl sx={{ width: "350px" }}>
            <Input
              endDecorator={<SearchIcon />}
              placeholder="Buscar por nombre..."
            />
          </FormControl>
          <Button color="danger" onClick={() => setRegisterForm(true)}>
            Registrar empleado
          </Button>
        </Stack>
        {registerForm && <AddEmployee setRegisterForm={setRegisterForm} />}
        <EmployeesList />
      </Stack>
    </Sheet>
  );
};

export default Employees;
