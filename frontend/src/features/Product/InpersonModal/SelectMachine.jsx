import React from "react";
import { useEffect } from "react";
import useAuth from "../../utils/useAuth";
import { Stack, FormControl, Select, Box, FormLabel } from "@mui/joy";
import Option from "@mui/joy/Option";
import { set } from "date-fns";

const SelectMachine = ({
  machineId,
  setDisable,
  setSelectedCity,
  setUnitId,
}) => {
  const { post } = useAuth();

  const [locations, setLocations] = React.useState([]);
  const [selectedLocation, setSelectedLocation] = React.useState(locations[0]);
  const [units, setUnits] = React.useState([]);
  const [selectedUnit, setSelectedUnit] = React.useState(units[0]);

  useEffect(() => {
    setDisable(true);
    getLocations();
    return () => setDisable(false);
  }, []);

  useEffect(() => {
    if (selectedLocation) {
      getUnits();
    }
  }, [selectedLocation]);

  useEffect(() => {
    if (locations.length > 0) {
      setSelectedLocation(locations[0]);
    }
  }, [locations]);

  useEffect(() => {
    if (units.length > 0) {
      setSelectedUnit(units[0]);
    }
  }, [units]);

  useEffect(() => {
    if (selectedUnit && selectedLocation) {
      console.log("Selected Unit:", selectedUnit);
      console.log("Selected Location:", selectedLocation);
      setUnitId(selectedUnit);
      setSelectedCity(selectedLocation.city);
      setDisable(false);
    }
  }, [selectedUnit, selectedLocation]);

  async function getLocations() {
    try {
      const response = await post(`/explore/${machineId}/locations`);
      console.log(response.data);
      setLocations(response.data.locations);
    } catch (error) {
      console.error("Error fetching locations:", error);
    }
  }

  const handleLocationSelect = (event, newValue) => {
    const loc = locations.find((x) => x.id === newValue);
    setSelectedLocation(loc);
    setSelectedCity(loc.city);
  };

  async function getUnits() {
    try {
      const response = await post(`/staff/rental/getunits`, {
        model_id: Number(machineId),
        location_id: selectedLocation.id,
      });
      console.log(response.data);
      setUnits(response.data.units_id);
    } catch (error) {
      console.error("Error fetching units:", error);
    }
  }

  const handleUnitSelect = (event, newValue) => {
    setSelectedUnit(newValue);
    setUnitId(newValue);
    setDisable(false);
  };

  return (
    <Box sx={{ width: "70%" }}>
      <Stack
        spacing={2}
        sx={{ display: "flex", alignItems: "center", justifyContent: "center" }}
      >
        <FormControl sx={{ width: "70%" }}>
          <FormLabel>Selecciona una ubicacion</FormLabel>
          <Select
            placeholder="Ubicacion"
            onChange={handleLocationSelect}
            value={selectedLocation?.id || ""}
          >
            {locations &&
              locations.map((location) => (
                <Option key={location.id} value={location.id}>
                  {location.city}, {location.street} {location.number}
                </Option>
              ))}
          </Select>
        </FormControl>
        {selectedLocation && (
          <FormControl sx={{ width: "70%" }}>
            <FormLabel>Selecciona un ejemplar (ID de ejemplar)</FormLabel>
            <Select
              placeholder="Ejemplar"
              onChange={handleUnitSelect}
              value={selectedUnit || ""}
            >
              {units &&
                units.map((unit) => (
                  <Option key={unit} value={unit}>
                    {unit}
                  </Option>
                ))}
            </Select>
          </FormControl>
        )}
      </Stack>
    </Box>
  );
};

export default SelectMachine;
