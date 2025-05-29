import { FormControl, FormLabel, Input, Sheet } from "@mui/joy";
import { useEffect, useState } from "react";

function Duration({ dispatch }) {
  const [minEnd, setMinEnd] = useState(() => {
    const d = new Date();
    d.setDate(d.getDate() + 7);
    return d.toISOString().split("T")[0]; // Return as value for the input
  });
  const [endValue, setEndValue] = useState(minEnd);
  const [startValue, setStartValue] = useState(() => {
    const d = new Date();
    const value = d.toISOString().split("T")[0];
    return {
      value,
      min: value,
    };
  });

  // Update the end date whenever the start changes
  function handleChange(e) {
    const {
      target: { value },
    } = e;
    setStartValue((prev) => ({ ...prev, value }));

    const d = new Date(value);
    d.setDate(d.getDate() + 7);
    const newValue = d.toISOString().split("T")[0];

    setEndValue(newValue);
    setMinEnd(newValue);
  }

  useEffect(() => {
    dispatch({ type: "setDates", value: [startValue.value, endValue] });
  }, [startValue, endValue]);

  return (
    <Sheet
      sx={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        gap: 2,
      }}
    >
      <FormControl sx={{ width: "100%" }}>
        <FormLabel>Fecha de inicio:</FormLabel>
        <Input
          slotProps={{
            input: { min: startValue.min },
          }}
          onKeyDown={(e) => {
            return e.preventDefault(); // Disable keyboard input
          }}
          value={startValue.value}
          onChange={handleChange}
          type="date"
        ></Input>
      </FormControl>
      <FormControl sx={{ width: "100%" }}>
        <FormLabel>Fecha de fin</FormLabel>
        <Input
          slotProps={{ input: { min: minEnd } }}
          value={endValue}
          onKeyDown={(e) => {
            return e.preventDefault();
          }}
          onChange={(e) => setEndValue(e.target.value)}
          type="date"
        ></Input>
      </FormControl>
    </Sheet>
  );
}

export default Duration;
