import { Input, Sheet } from "@mui/joy";

function InputGroup({ code, refs, onChange, onKeyDown }) {
  return (
    <Sheet
      sx={{
        display: "flex",
        gap: {
          xs: 1,
          sm: 2,
        },
      }}
    >
      {code.map((_, i) => (
        <Input
          slotProps={{
            input: {
              ref: (c) => (refs.current[i] = c), // Save the ref
              inputMode: "numeric", // Use the numeric keyboard on mobile
              style: {
                textAlign: "center",
              },
            },
          }}
          type="text"
          inputmode="numeric"
          value={code[i]}
          onChange={(e) => onChange(e, i)}
          onKeyDown={(e) => onKeyDown(e, i)}
          sx={{
            width: "60px",
            height: "70px",
            fontSize: "xl",
            fontWeight: 800,
          }}
        />
      ))}
    </Sheet>
  );
}

export default InputGroup;
