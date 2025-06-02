import SearchIcon from "@mui/icons-material/Search";
import { FormControl, Input, Sheet, Snackbar } from "@mui/joy";
import Button from "@mui/joy/Button";
import { Stack } from "@mui/material";
import { useState } from "react";
import AddEmployee from "./AddEmployee";
import EmployeesList from "./EmployeesList";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";

const Employees = () => {
  const [registerForm, setRegisterForm] = useState(false);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });

  const [refreshEmployees, setRefreshEmployees] = useState(false);

  const handleEmployeeAdded = () => {
    setRefreshEmployees((prev) => !prev);
  };

  return (
    <>
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
          {registerForm && (
            <AddEmployee
              setRegisterForm={setRegisterForm}
              setOpenSnack={setOpenSnack}
              setStatus={setStatus}
              handleEmployeeAdded={handleEmployeeAdded}
            />
          )}
          <EmployeesList refreshEmployees={refreshEmployees} />
        </Stack>
        <Snackbar
          variant="soft"
          color={status.isError ? "danger" : "success"}
          autoHideDuration={3000}
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
          endDecorator={
            <Button
              onClick={() => setOpenSnack(false)}
              size="sm"
              variant="soft"
              color={status.isError ? "danger" : "success"}
            >
              Dismiss
            </Button>
          }
        >
          {status.message}
        </Snackbar>
      </Sheet>
    </>
  );
};

export default Employees;
