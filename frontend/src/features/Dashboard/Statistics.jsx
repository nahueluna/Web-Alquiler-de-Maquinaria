import React, { useEffect } from "react";
import HorizontalBarChart from "./HorizontalBarChart";
import Stack from "@mui/joy/Stack";
import MenuButton from "@mui/joy/MenuButton";
import Menu from "@mui/joy/Menu";
import MenuItem from "@mui/joy/MenuItem";
import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import Dropdown from "@mui/joy/Dropdown";
import LineChart from "./LineChart";
import useAuth from "../utils/useAuth";
import Snackbar from "@mui/joy/Snackbar";
import Button from "@mui/joy/Button";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";
import Input from "@mui/joy/Input";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import { FormControl, FormHelperText, FormLabel } from "@mui/joy";
import IconButton from "@mui/joy/IconButton";
import Box from "@mui/joy/Box";
import Link from "@mui/joy/Link";
import Select, { selectClasses } from "@mui/joy/Select";
import Option from "@mui/joy/Option";
import Typography from "@mui/joy/Typography";
import CircularProgress from "@mui/joy/CircularProgress";
import KeyboardArrowDown from "@mui/icons-material/KeyboardArrowDown";

const Statistics = () => {
  const { post } = useAuth();

  const currentYear = new Date().getFullYear();
  const [year, setYear] = React.useState(currentYear.toString());
  const years = [];
  for (let y = currentYear; y >= 1900; y--) {
    years.push(y.toString());
  }

  const handleSelect = (event, value) => {
    setYear(value);
    console.log("Selected year:", value);
  };

  const [fechaInicio, setFechaInicio] = React.useState("");
  const [fechaFin, setFechaFin] = React.useState("");
  const [optionalSettings, setOptionalSettings] = React.useState({
    order: {
      label: "Descendente",
      value: "desc",
    },
    period: [],
  });

  const [statsData, setStatsData] = React.useState([]);
  const [type, setType] = React.useState({
    label: "Ingresos",
    value: "income",
  });
  const [groupBy, setGroupBy] = React.useState({
    label: "Por mes",
    value: "month",
  });

  function isStatsDataEmpty(statsData) {
    // Para estadisticas por empleado o categoria, que son array
    if (Array.isArray(statsData)) {
      return statsData.length === 0;
    }
    if (statsData && typeof statsData === "object") {
      // Para estadisticas por mes, que retorna "mes" : 0
      return Object.values(statsData).every((val) => !val || val === 0);
    }
    return true;
  }

  const [loading, setLoading] = React.useState(false);

  const [openSnack, setOpenSnack] = React.useState(false);
  const [status, setStatus] = React.useState({
    isError: false,
    message: "",
  });

  const [formError, setFormError] = React.useState("");

  const handleDateSubmit = () => {
    if (fechaInicio && fechaFin && fechaInicio <= fechaFin) {
      setOptionalSettings({
        ...optionalSettings,
        period: [fechaInicio, fechaFin],
      });
      setFormError("");
      return;
    }
    if (!fechaInicio && !fechaFin) {
      setFormError("Tenes que indicar un rango de fechas");
    }
    if (fechaInicio && !fechaFin) {
      setFormError("Tenes que indicar la fecha de fin");
    }
    if (!fechaInicio && fechaFin) {
      setFormError("Tenes que indicar la fecha de inicio");
    }
    if (fechaInicio && fechaFin && fechaInicio > fechaFin) {
      setFormError("La fecha de inicio no puede ser mayor a la fecha de fin");
    }
  };
  async function getStats(type, groupBy) {
    setStatsData([]);
    console.log("getStats", type, groupBy);
    try {
      const parameters = {
        stat_type: type,
        group_by: groupBy,
        year: Number(year),
      };
      if (optionalSettings.period.length > 0) {
        parameters.period = optionalSettings.period;
      }
      if (optionalSettings.order) {
        parameters.order = optionalSettings.order.value;
      }
      console.log("getStats parameters", parameters.period);
      const { data } = await post("/stats", parameters);
      console.log("getStats data", data.stats);
      setStatsData(data.stats);
    } catch (error) {
      setStatsData([]); // Por si mete cualquier cosa ns
      let errorMsg = "Ocurrio un error al intentar obtener las estadisticas";
      switch (error.response?.status) {
        case 403:
          errorMsg = "No tenes permisos para ver las estadisticas";
          break;
      }
      setStatus({
        isError: true,
        message: errorMsg,
      });
      setOpenSnack(true);
    }
  }

  useEffect(() => {
    const getNewStats = async () => {
      setLoading(true);
      setStatsData([]);
      await getStats(type.value, groupBy.value);
      setLoading(false);
    };

    getNewStats();
  }, [year, type, groupBy, optionalSettings]);
  return (
    <>
      <Stack
        spacing={2}
        sx={{
          pl: 2,
          pt: 2,
        }}
      >
        <Stack direction="row" spacing={1}>
          <Dropdown>
            <MenuButton
              endDecorator={<ArrowDropDown />}
              sx={{
                minWidth: 200,
              }}
            >
              {type.label}
            </MenuButton>
            <Menu>
              <MenuItem
                selected={type.label === "Ingresos"}
                onClick={() => {
                  setType({
                    label: "Ingresos",
                    value: "income",
                  });
                }}
              >
                Ingresos
              </MenuItem>
              <MenuItem
                selected={type.label === "Alquileres realizados"}
                onClick={() => {
                  setType({
                    label: "Alquileres realizados",
                    value: "rentals",
                  });
                }}
              >
                Alquileres realizados
              </MenuItem>
            </Menu>
          </Dropdown>
          <Dropdown>
            <MenuButton
              endDecorator={<ArrowDropDown />}
              sx={{
                minWidth: 150,
              }}
            >
              {groupBy.label}
            </MenuButton>
            <Menu>
              <MenuItem
                selected={groupBy.label === "Por mes"}
                onClick={() => {
                  setGroupBy({
                    label: "Por mes",
                    value: "month",
                  });
                }}
              >
                Por mes
              </MenuItem>
              <MenuItem
                selected={groupBy.label === "Por empleado"}
                onClick={() => {
                  setGroupBy({
                    label: "Por empleado",
                    value: "employee",
                  });
                }}
              >
                Por empleado
              </MenuItem>
              <MenuItem
                selected={groupBy.label === "Por categoria"}
                onClick={() => {
                  setGroupBy({
                    label: "Por categoria",
                    value: "category",
                  });
                }}
              >
                Por categoria
              </MenuItem>
            </Menu>
          </Dropdown>
          {groupBy.value === "month" && (
            <Stack spacing={2} sx={{ maxWidth: "100px" }}>
              <Select
                placeholder="Año"
                indicator={<ArrowDropDown />}
                defaultValue={year}
                onChange={handleSelect}
                sx={{
                  [`& .${selectClasses.indicator}`]: {
                    transition: "0.2s",
                    [`&.${selectClasses.expanded}`]: {
                      transform: "rotate(-180deg)",
                    },
                  },
                }}
              >
                {years.map((year) => (
                  <Option key={year} value={year}>
                    {year}
                  </Option>
                ))}
              </Select>
            </Stack>
          )}
        </Stack>
        {groupBy.value != "month" && (
          <Stack
            sx={{
              width: "fit-content",
              backgroundColor: "#f5f5f5",
              padding: 1,
              borderRadius: "8px",
            }}
          >
            <Stack spacing={1} direction="row">
              <FormControl>
                <FormLabel>Fecha de inicio</FormLabel>
                <Input
                  type="date"
                  value={fechaInicio}
                  onChange={(e) => {
                    setFechaInicio(e.target.value);
                    if (fechaFin && e.target.value > fechaFin) setFechaFin("");
                  }}
                  slotProps={{
                    input: {
                      max: "9999-12-12",
                    },
                  }}
                  placeholder="Fecha de inicio"
                />
              </FormControl>
              <FormControl>
                <FormLabel>Fecha de fin</FormLabel>
                <Input
                  type="date"
                  value={fechaFin}
                  onChange={(e) => setFechaFin(e.target.value)}
                  slotProps={{
                    input: {
                      max: "9999-12-12",
                    },
                  }}
                  placeholder="Fecha de fin"
                />
              </FormControl>
              <Box
                sx={{
                  alignContent: "flex-end",
                  alignSelf: "flex-end",
                }}
              >
                <Button
                  onClick={handleDateSubmit}
                  variant="plain"
                  color="danger"
                >
                  Aplicar
                </Button>
              </Box>
              <Dropdown>
                <MenuButton
                  endDecorator={<ArrowDropDown />}
                  sx={{
                    minWidth: 130,
                    height: 20,
                    alignSelf: "flex-end",
                    backgroundColor: "white",
                  }}
                >
                  {optionalSettings.order.label}
                </MenuButton>
                <Menu>
                  <MenuItem
                    selected={optionalSettings.order.label === "Descendente"}
                    onClick={() => {
                      setOptionalSettings({
                        ...optionalSettings,
                        order: { label: "Descendente", value: "desc" },
                      });
                    }}
                  >
                    Descendente
                  </MenuItem>
                  <MenuItem
                    selected={optionalSettings.order.label === "Ascendente"}
                    onClick={() => {
                      setOptionalSettings({
                        ...optionalSettings,
                        order: { label: "Ascendente", value: "asc" },
                      });
                    }}
                  >
                    Ascendente
                  </MenuItem>
                </Menu>
              </Dropdown>
            </Stack>
            <FormHelperText>{formError ? formError : ""}</FormHelperText>
            {(fechaInicio ||
              fechaFin ||
              (optionalSettings && optionalSettings.period.length > 0)) && (
              <Typography>
                <Link
                  onClick={() => {
                    setFechaInicio("");
                    setFechaFin("");
                    setFormError("");
                    setOptionalSettings({
                      ...optionalSettings,
                      period: [],
                    });
                  }}
                  level="body-sm"
                  sx={{ fontWeight: "400" }}
                >
                  Limpiar fechas
                </Link>
              </Typography>
            )}
          </Stack>
        )}
        {loading ? (
          <CircularProgress
            color="danger"
            size="md"
            variant="plain"
            sx={{
              alignSelf: "center",
              marginTop: 2,
            }}
          />
        ) : (
          <>
            {isStatsDataEmpty(statsData) && (
              <Typography level="title-md" sx={{ mb: 2 }}>
                Oops. No se encontraron resultados para esta consulta.
              </Typography>
            )}
            {groupBy.value === "month" ? (
              <LineChart typeName={type.label} statsData={statsData} />
            ) : (
              Array.isArray(statsData) && (
                <HorizontalBarChart
                  typeName={type.label}
                  statsData={statsData}
                  period={optionalSettings.period}
                />
              )
            )}
          </>
        )}
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
            Cerrar
          </Button>
        }
      >
        {status.message}
      </Snackbar>
    </>
  );
};

export default Statistics;
