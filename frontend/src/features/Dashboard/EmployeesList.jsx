import Delete from "@mui/icons-material/Delete";
import { Button, Modal, Sheet, Typography } from "@mui/joy";
import Box from "@mui/joy/Box";
import IconButton from "@mui/joy/IconButton";
import List from "@mui/joy/List";
import ListDivider from "@mui/joy/ListDivider";
import ListItem from "@mui/joy/ListItem";
import Snackbar from "@mui/joy/Snackbar";
import Stack from "@mui/joy/Stack";
import Tooltip from "@mui/joy/Tooltip";
import { useEffect, useState } from "react";
import useAuth from "../utils/useAuth";

const EmployeesList = ({ refreshEmployees }) => {
  //const [open, setOpen] = React.useState(false);
  //const [selectedEmployee, setSelectedEmployee] = React.useState(null);
  //const [errorSnackbar, setErrorSnackbar] = useState({ open: false, message: "" });
  //const [loading, setLoading] = useState(false);
  const { post } = useAuth();

  const [employees, setEmployees] = useState([]);
  const [open, setOpen] = useState(false);
  const [selectedEmployee, setSelectedEmployee] = useState(null);
  const [errorSnackbar, setErrorSnackbar] = useState({
    open: false,
    message: "",
  });
  const [loading, setLoading] = useState(false);

  const fetchEmployees = async () => {
    setLoading(true);
    try {
      const response = await post("/getemployees");
      if (response.status === 200) {
        setEmployees(response.data.employees);
      }
    } catch (error) {
      if (error.response) {
        switch (error.response.status) {
          case 401:
            setErrorSnackbar({
              open: true,
              message: "Token inválido. Por favor, inicia sesión de nuevo.",
            });
            break;
          case 403:
            setErrorSnackbar({
              open: true,
              message: "No tienes permisos de administrador.",
            });
            break;
          case 500:
            setErrorSnackbar({
              open: true,
              message: "Error interno del servidor.",
            });
            break;
          default:
            setErrorSnackbar({
              open: true,
              message: error.response.data.message || "Error desconocido.",
            });
        }
      } else {
        setErrorSnackbar({
          open: true,
          message: "No se pudo conectar con el servidor.",
        });
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchEmployees();
  }, [refreshEmployees]);

  const handleDeleteClick = (employee) => {
    setSelectedEmployee(employee);
    setOpen(true);
  };

  const handleConfirmedDelete = async () => {
    if (!selectedEmployee) return;
    setLoading(true);
    try {
      const response = await post("/deletemployee", {
        id: selectedEmployee.id,
      });
      if (response.status === 200) {
        setEmployees((prev) =>
          prev.filter((emp) => emp.id !== selectedEmployee.id)
        );
        setOpen(false);
        setSelectedEmployee(null);
      }
    } catch (error) {
      if (error.response) {
        switch (error.response.status) {
          case 401:
            setErrorSnackbar({
              open: true,
              message: "Token inválido. Por favor, inicia sesión de nuevo.",
            });
            break;
          case 403:
            setErrorSnackbar({
              open: true,
              message: "No tienes permisos de administrador.",
            });
            break;
          case 500:
            setErrorSnackbar({
              open: true,
              message: "Error interno o empleado no encontrado.",
            });
            break;
          default:
            setErrorSnackbar({
              open: true,
              message: error.response.data.message || "Error desconocido.",
            });
        }
      } else {
        setErrorSnackbar({
          open: true,
          message: "No se pudo conectar con el servidor.",
        });
      }
    } finally {
      setLoading(false);
    }
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
              <Box key={index}>
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
              </Box>
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
      <Snackbar
        open={errorSnackbar.open}
        onClose={() => setErrorSnackbar({ open: false, message: "" })}
        message={errorSnackbar.message}
        color="danger"
        variant="soft"
        autoHideDuration={3000}
        sx={{
          position: "fixed",
          bottom: 16,
          left: "70%",
          transform: "translateX(-50%)",
          zIndex: 9999,
        }}
      >
        {errorSnackbar.message}
      </Snackbar>
    </>
  );
};

export default EmployeesList;
