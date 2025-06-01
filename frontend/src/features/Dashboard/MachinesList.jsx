import {
  Sheet,
  AccordionGroup,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Avatar,
  Typography,
  Button,
  FormControl,
  FormLabel,
  Input,
  Select,
  Option,
  Snackbar,
  Alert
} from "@mui/joy";
import { useState, useEffect, useContext } from "react";
import axios from "axios"
import UserContext from "../../context/UserContext"

export const MachinesList = () => {
  const { user } = useContext(UserContext);
  const [machinesData, setMachinesData] = useState([]);
  const [feedback, setFeedback] = useState("");
  const [openFormId, setOpenFormId] = useState(null);
  const [expandedId, setExpandedId] = useState(null);
  const [estado, setEstado] = useState("");
  const [ubicacion, setUbicacion] = useState("");
  const [snackbar, setSnackbar] = useState({
    open: false,
    message: "",
    color: "neutral",
    duration: 3000,
  });

  const token = user?.access || "";

  useEffect(() => {
    axios
      .get("http://localhost:8000/explore")
      .then((response) => {
        console.log(response.data.items[0])
        setMachinesData(response.data.items || []);
      })
      .catch((error) => {
        console.error("Error cargando máquinas:", error);
      });
  }, []);

    const handleSubmit = (e, machine) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const serial_number = formData.get("numeroSerie");
    const estado = formData.get("estado");
    const ubicacion = formData.get("ubicacion");

    const locationMap = {
      marDelPlata: 1,
      bahiaBlanca: 2,
      caballito: 3,
    };
    const location_id = locationMap[ubicacion];

    axios
      .post(
        "http://localhost:8000/newunit",
        {
          access: token,
          serial_number,
          model_id: machine.id,
          location_id,
        },
        {
          headers: {
            "Content-Type": "application/json",
          },
        }
      )
      .then((res) => {
        setSnackbar({
          open: true,
          message: "Unidad agregada con éxito.",
          color: "success",
          duration: 3000,
        });
        setOpenFormId(null);
      })
      .catch((err) => {
        if (err.response) {
          const status = err.response.status;
          switch (status) {
            case 400:
              setSnackbar({
                open: true,
                message: "Error: El número de serie ya existe.",
                color: "danger",
                duration: 3000,
              });
              break;
            case 401:
              setSnackbar({
                open: true,
                message: "Error: Token inválido.",
                color: "danger",
                duration: 3000,
              });
              break;
            case 403:
              setSnackbar({
                open: true,
                message: "Error: No posee permisos para esta acción.",
                color: "danger",
                duration: 3000,
              });
              break;
            case 404:
              setSnackbar({
                open: true,
                message: "Error: No se pudo encontrar a la máquina o al usuario.",
                color: "danger",
                duration: 3000,
              });
              break;
            case 500:
              setSnackbar({
                open: true,
                message: "Error interno.",
                color: "danger",
                duration: 3000,
              });
              break;
            default:
              setSnackbar({
                open: true,
                message: "Error desconocido.",
                color: "danger",
                duration: 3000,
              });
          }
        } else {
          // Error de conexión o sin respuesta del servidor
          setSnackbar({
            open: true,
            message: "Error en la conexión.",
            color: "danger",
            duration: 3000,
          });
        }
});
  };

  return (
    <>
    <Sheet
      variant="outlined"
      sx={{ borderRadius: "sm", width: "60%", minWidth: "600px" }}
    >
      <AccordionGroup>
        {machinesData.map((machine) => (
          <Accordion
            key={machine.id}
            expanded={expandedId === machine.id}
            onChange={(_, expanded) =>
              setExpandedId(expanded ? machine.id : null)
            }
          >
            <AccordionSummary>
              <Avatar src={machine.main_image} sx={{ mr: 2, width: 56, height: 56 }} alt={machine.name} />
              <Typography level="title-md" sx={{ flex: 1 }}>
                {machine.name + " " + machine.brand + " " + machine.model}
              </Typography>
            </AccordionSummary>
            <AccordionDetails>
                <Button
                  size="sm"
                  color="danger"
                  variant="soft"
                  onClick={(e) => {
                    e.stopPropagation();
                    setOpenFormId(
                      openFormId === machine.id ? null : machine.id
                    );
                    if (expandedId !== machine.id) setExpandedId(machine.id);
                  }}
                  sx={{ width: "30%", alignSelf: "flex-end" }}
                >
                  Agregar ejemplar
                </Button>

                {openFormId === machine.id && (
              <form
                onSubmit={(e) => handleSubmit(e, machine)}
                style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}
              >
                <FormControl>
                  <FormLabel>Número de serie</FormLabel>
                  <Input name="numeroSerie" required />
                </FormControl>

                <FormControl>
                  <FormLabel>Estado</FormLabel>
                  <Select name="estado" defaultValue="disponible" required>
                    <Option value="disponible">Disponible</Option>
                    <Option value="mantenimiento">En mantenimiento</Option>
                    <Option value="noDisponible">No disponible</Option>
                  </Select>
                </FormControl>

                <FormControl>
                  <FormLabel>Ubicación</FormLabel>
                  <Select name="ubicacion" defaultValue="marDelPlata" required>
                    <Option value="marDelPlata">Mar del Plata</Option>
                    <Option value="bahiaBlanca">Bahía Blanca</Option>
                    <Option value="caballito">Caballito</Option>
                  </Select>
                </FormControl>

                <Button type="submit" color="primary" variant="solid">
                  Confirmar
                </Button>
              </form>
            )}
          </AccordionDetails>
        </Accordion>
      ))}
    </AccordionGroup>
  </Sheet>
  <Snackbar
    open={snackbar.open}
    onClose={() => setSnackbar((prev) => ({ ...prev, open: false }))}
    autoHideDuration={3000}
    anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
    variant="soft"
    color={snackbar.color}
  >
    {snackbar.message}
  </Snackbar>
  </>
  );
};

export default MachinesList;
