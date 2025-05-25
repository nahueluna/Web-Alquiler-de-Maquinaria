import SearchIcon from "@mui/icons-material/Search";
import {
  Accordion,
  AccordionDetails,
  AccordionGroup,
  AccordionSummary,
  Avatar,
  Button,
  FormControl,
  Input,
  Sheet,
  Stack,
  Typography,
} from "@mui/joy";
import { useState } from "react";
const machinesData = [
  {
    id: 1,
    modelo: "Excavadora ZX200",
    imagen: "https://via.placeholder.com/40",
    descripcion: "Excavadora de gran porte.",
    detalles: ["Mantenimiento: OK", "Año: 2022", "Horas: 1500"],
    ejemplares: [
      {
        id: "ZX200-1",
        imagen: "https://via.placeholder.com/32",
        estado: "Disponible",
        ubicacion: "Depósito 1",
      },
      {
        id: "ZX200-2",
        imagen: "https://via.placeholder.com/32",
        estado: "En reparación",
        ubicacion: "Taller",
      },
    ],
  },
  {
    id: 2,
    modelo: "Retroexcavadora CAT",
    imagen: "https://via.placeholder.com/40",
    descripcion: "Retroexcavadora compacta.",
    detalles: ["Mantenimiento: Pendiente", "Año: 2021", "Horas: 2000"],
    ejemplares: [
      {
        id: "CAT-1",
        imagen: "https://via.placeholder.com/32",
        estado: "Alquilada",
        ubicacion: "Obra 3",
      },
    ],
  },
];

const Machines = () => {
  const [openFormId, setOpenFormId] = useState(null);
  const [expandedId, setExpandedId] = useState(null);

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
          <Button color="danger">Registrar nuevo modelo</Button>
        </Stack>
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
                <Avatar src={machine.imagen} sx={{ mr: 2 }} />
                <Typography level="title-md" sx={{ flex: 1 }}>
                  {machine.modelo}
                </Typography>
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
                >
                  Agregar ejemplar
                </Button>
              </AccordionSummary>
              <AccordionDetails>
                <Typography level="title-sm" sx={{ mt: 2, mb: 1 }}>
                  Ejemplares:
                </Typography>
                <Stack spacing={2}>
                  {machine.ejemplares.map((ejemplar) => (
                    <Sheet
                      key={ejemplar.id}
                      variant="outlined"
                      sx={{
                        display: "flex",
                        alignItems: "center",
                        p: 1,
                        borderRadius: "sm",
                        gap: 2,
                      }}
                    >
                      <Typography sx={{ minWidth: 100 }}>
                        {ejemplar.id}
                      </Typography>
                      <Typography sx={{ minWidth: 100 }}>
                        {ejemplar.estado}
                      </Typography>
                      <Typography sx={{ minWidth: 120 }}>
                        {ejemplar.ubicacion}
                      </Typography>
                    </Sheet>
                  ))}
                  {openFormId === machine.id && (
                    <Sheet
                      variant="soft"
                      sx={{
                        p: 2,
                        borderRadius: "sm",
                        mt: 2,
                        backgroundColor: "#fff7f7",
                      }}
                    >
                      <Typography level="title-sm" sx={{ mb: 1 }}>
                        Nuevo ejemplar
                      </Typography>
                      <Stack direction="row" spacing={2}>
                        <Input placeholder="ID" size="sm" />
                        <Input placeholder="Estado" size="sm" />
                        <Input placeholder="Ubicación" size="sm" />
                        <Button size="sm" color="danger" variant="solid">
                          Guardar
                        </Button>
                        <Button
                          size="sm"
                          variant="plain"
                          onClick={() => setOpenFormId(null)}
                        >
                          Cancelar
                        </Button>
                      </Stack>
                    </Sheet>
                  )}
                </Stack>
              </AccordionDetails>
            </Accordion>
          ))}
        </AccordionGroup>
      </Stack>
    </Sheet>
  );
};

export default Machines;
