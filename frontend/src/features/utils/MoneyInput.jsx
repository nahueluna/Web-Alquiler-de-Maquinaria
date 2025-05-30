import FormControl from "@mui/joy/FormControl";
import Input from "@mui/joy/Input";
import PropTypes from "prop-types";
import * as React from "react";
import { NumericFormat } from "react-number-format";

const NumericFormatAdapter = React.forwardRef(function NumericFormatAdapter(
  props,
  ref
) {
  const { onChange, ...other } = props;

  return (
    <NumericFormat
      {...other}
      getInputRef={ref}
      onValueChange={(values) => {
        onChange({
          target: {
            name: props.name,
            value: values.value,
          },
        });
      }}
      thousandSeparator
      valueIsNumericString
      prefix="$"
    />
  );
});

NumericFormatAdapter.propTypes = {
  name: PropTypes.string.isRequired,
  onChange: PropTypes.func.isRequired,
};

export default function MoneyInput() {
  const [value, setValue] = React.useState("0");
  return (
    <FormControl>
      <Input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="Precio de alquiler por dia"
        slotProps={{
          input: {
            component: NumericFormatAdapter,
          },
        }}
      />
    </FormControl>
  );
}
