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

export default function MoneyInput({ placeholder, name }) {
  const [value, setValue] = React.useState("");
  return (
    <FormControl>
      <Input
        name={name ? name : "money-input"}
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder={placeholder}
        slotProps={{
          input: {
            component: NumericFormatAdapter,
          },
        }}
      />
    </FormControl>
  );
}
