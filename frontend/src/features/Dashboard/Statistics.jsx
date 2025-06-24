import React, { useEffect } from "react";
import HorizontalBarChart from "./HorizontalBarChart";
import Stack from "@mui/joy/Stack";
import MenuButton from "@mui/joy/MenuButton";
import Menu from "@mui/joy/Menu";
import MenuItem from "@mui/joy/MenuItem";
import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import Dropdown from "@mui/joy/Dropdown";
import BarChart from "./BarChart";
import useAuth from "../utils/useAuth";
import Typography from "@mui/joy/Typography";
import Snackbar from "@mui/joy/Snackbar";
import Button from "@mui/joy/Button";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";

const Statistics = () => {
  const { post } = useAuth();

  const [statsData, setStatsData] = React.useState([]);
  const [type, setType] = React.useState({
    label: "Ingresos",
    value: "income",
  });
  const [groupBy, setGroupBy] = React.useState({
    label: "Por mes",
    value: "month",
  });

  const [loading, setLoading] = React.useState(false);

  const [openSnack, setOpenSnack] = React.useState(false);
  const [status, setStatus] = React.useState({
    isError: false,
    message: "",
  });

  async function getStats(type, groupBy) {
    setStatsData([]);
    console.log("getStats", type, groupBy);
    try {
      const { data } = await post("/stats", {
        stat_type: type,
        group_by: groupBy,
      });
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
  }, [type, groupBy]);
  return (
    <>
      <Stack spacing={3}>
        <Stack
          direction="row"
          spacing={1}
          sx={{
            pl: 2,
          }}
        >
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
        </Stack>
        {loading ? (
          <p>Cargando...</p>
        ) : statsData ? (
          groupBy.value === "month" ? (
            <BarChart typeName={type.label} statsData={statsData} />
          ) : (
            Array.isArray(statsData) && (
              <HorizontalBarChart typeName={type.label} statsData={statsData} />
            )
          )
        ) : null}
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
