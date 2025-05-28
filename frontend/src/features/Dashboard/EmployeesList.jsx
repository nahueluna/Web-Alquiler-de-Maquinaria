import Delete from "@mui/icons-material/Delete";
import { Button, Modal, Sheet, Typography } from "@mui/joy";
import IconButton from "@mui/joy/IconButton";
import List from "@mui/joy/List";
import ListDivider from "@mui/joy/ListDivider";
import ListItem from "@mui/joy/ListItem";
import Stack from "@mui/joy/Stack";
import Tooltip from "@mui/joy/Tooltip";
import React from "react";

const employees = [
  {
    name: "Juan",
    surname: "Pérez",
    email: "sadasda",
  },
  {
    name: "Ana",
    surname: "Gómez",
    email: "dsadassdadas",
  },
];

const EmployeesList = () => {
  const [open, setOpen] = React.useState(false);
  const [selectedEmployee, setSelectedEmployee] = React.useState(null);
  const handleConfirmedDelete = (employeeToDelete) => {
    console.log(employeeToDelete);
    // Conectar para eliminar empleado hola soy el ing ramos
    setOpen(false);
  };

  return (
    <>
      {employees.length === 0 ? (
        <Typography variant="h6" color="textSecondary" align="center">
          Oopsie. Parece que no hay empleados registrados.
        </Typography>
      ) : (
        <>
          <List
            variant="outlined"
            size="lg"
            sx={{ borderRadius: "sm", width: "50%", minWidth: "500px" }}
          >
            {employees.map((employee, index) => (
              <>
                <ListItem
                  endAction={
                    <Tooltip title="Eliminar empleado">
                      <IconButton
                        aria-label="Delete"
                        size="sm"
                        color="danger"
                        onClick={() => {
                          setSelectedEmployee(employee);
                          setOpen(true);
                        }}
                      >
                        <Delete />
                      </IconButton>
                    </Tooltip>
                  }
                >
                  <Stack>
                    <Typography level="title-sm">
                      {employee.name} {employee.surname}
                    </Typography>
                    <Typography level="body-sm">{employee.email}</Typography>
                  </Stack>
                </ListItem>
                {index < employees.length - 1 && <ListDivider />}
              </>
            ))}
          </List>
          <Modal
            open={open}
            onClose={() => setOpen(false)}
            sx={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
            }}
          >
            <Sheet
              variant="outlined"
              sx={{ p: 2, width: "400px", borderRadius: "lg" }}
            >
              <Stack>
                <Typography variant="h6" sx={{ p: 2 }}>
                  ¿Estás seguro de que queres eliminar a{" "}
                  <Typography fontWeight="bold">
                    {selectedEmployee?.name} {selectedEmployee?.surname}
                  </Typography>{" "}
                  ({selectedEmployee?.email})?
                </Typography>
                <Stack direction="row" justifyContent="flex-end" spacing={1}>
                  <Button
                    variant="soft"
                    color="neutral"
                    onClick={() => setOpen(false)}
                  >
                    Cancelar
                  </Button>
                  <Button
                    variant="solid"
                    color="danger"
                    onClick={() => handleConfirmedDelete(selectedEmployee)}
                  >
                    Eliminar
                  </Button>
                </Stack>
              </Stack>
            </Sheet>
          </Modal>
        </>
      )}
    </>
  );
};

export default EmployeesList;
