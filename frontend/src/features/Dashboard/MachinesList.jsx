import {
  Accordion,
  AccordionDetails,
  AccordionGroup,
  AccordionSummary,
  Avatar,
  Button,
  Chip,
  Divider,
  Input,
  Sheet,
  Stack,
  Typography,
  Select,
  Option
} from "@mui/joy";
import { useState } from "react";

const machinesData = [
  {
    id: 1,
    modelo: "Excavadora ZX200",
    imagen:
      "https://image.made-in-china.com/202f0j00rpiRscOFCWzH/Sdlg-36t-E6360f-Manufacturing-Technology-Volvo-Excavator-with-High-Response-Rate.webp",
    descripcion: "Excavadora de gran porte.",
    detalles: ["Mantenimiento: OK", "Año: 2022", "Horas: 1500"],
    ejemplares: [
      {
        id: "ZX200-1",
        estado: "Disponible",
        ubicacion: "Depósito 1",
      },
      {
        id: "ZX200-2",
        estado: "En reparación",
        ubicacion: "Taller",
      },
    ],
  },
  {
    id: 2,
    modelo: "Retroexcavadora CAT",
    imagen: "https://http2.mlstatic.com/D_944682-MLA84275111016_052025-C.jpg",
    descripcion: "Retroexcavadora compacta.",
    detalles: ["Mantenimiento: Pendiente", "Año: 2021", "Horas: 2000"],
    ejemplares: [
      {
        id: "CAT-1",
        estado: "Alquilada",
        ubicacion: "Obra 3",
      },
    ],
  },
];
export const MachinesList = () => {
  const [openFormId, setOpenFormId] = useState(null);
  const [expandedId, setExpandedId] = useState(null);
  const [estado, setEstado] = useState("");
  const [ubicacion, setUbicacion] = useState("");

  return (
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
              <Avatar src={machine.imagen} sx={{ mr: 2 }} />
              <Typography level="title-md" sx={{ flex: 1 }}>
                {machine.modelo}
              </Typography>
            </AccordionSummary>
            <AccordionDetails>
              <Typography level="title-sm" sx={{ mt: 2, mb: 1 }}>
                Ejemplares:
              </Typography>
              <Stack spacing={0.5}>
                {machine.ejemplares.map((ejemplar) => (
                  <Sheet
                    key={ejemplar.id}
                    variant="outlined"
                    sx={{
                      display: "flex",
                      justifyContent: "space-evenly",
                      p: 1,
                      borderRadius: "sm",
                      backgroundColor: "#f7f7f7",
                      width: "100%",
                    }}
                  >
                    <Stack direction="row" spacing={1}>
                      <Typography sx={{ fontWeight: "bold" }}>
                        Numero de serie:{" "}
                      </Typography>
                      <Typography color="neutral">{ejemplar.id}</Typography>
                      <Divider orientation="vertical"></Divider>
                      <Typography sx={{ fontWeight: "bold" }}>
                        Ubicacion:{" "}
                      </Typography>
                      <Typography color="neutral">
                        {ejemplar.ubicacion}
                      </Typography>
                      <Divider
                        orientation="vertical"
                        sx={{ height: "100%" }}
                      ></Divider>

                      <Chip color="danger" variant="solid">
                        {ejemplar.estado}
                      </Chip>
                    </Stack>
                  </Sheet>
                ))}
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
                    <Select
                      placeholder="Estado"
                      value={estado}
                      onChange={(e, value) => setEstado(value)}
                      sx={{ minWidth: 120 }}
                      variant="outlined"
                    >
                      <Option value="Disponible">Disponible</Option>
                      <Option value="En mantenimiento">En mantenimiento</Option>
                      <Option value="No disponible">No disponible</Option>
                    </Select>
                    <Select
                      placeholder="Ubicación"
                      value={ubicacion}
                      onChange={(e, value) => setUbicacion(value)}
                      sx={{ minWidth: 130 }}
                      variant="outlined"
                    >
                      <Option value="mar_del_plata">Mar del Plata</Option>
                      <Option value="bahia_blanca">Bahía Blanca</Option>
                      <Option value="caballito">Caballito</Option>
                    </Select>
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
    </Sheet>
  );
};

export default MachinesList;
