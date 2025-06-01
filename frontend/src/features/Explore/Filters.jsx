import { KeyboardArrowRight } from "@mui/icons-material";
import CheckIcon from "@mui/icons-material/Check";
import { Divider, Typography } from "@mui/joy";
import Box from "@mui/joy/Box";
import IconButton from "@mui/joy/Button";
import Checkbox from "@mui/joy/Checkbox";
import Chip from "@mui/joy/Chip";
import FormControl from "@mui/joy/FormControl";
import FormHelperText from "@mui/joy/FormHelperText";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";
import { useFormik } from "formik";
import { useSearchParams } from "react-router-dom";
import * as yup from "yup";
import ResultsResumee from "./ResultsResumee";

const validationSchema = yup.object({
  minPrice: yup.number().nullable().typeError("Debe ser un número"),
  maxPrice: yup
    .number()
    .nullable()
    .typeError("Debe ser un número")
    .test(
      "max-gte-min",
      "El máximo no puede ser menor que el mínimo",
      function (value) {
        const { minPrice } = this.parent;
        if (minPrice != null && value != null) {
          return value >= minPrice;
        }
        return true;
      }
    ),
});

const Filters = ({ results, currentFilters, query }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const handleNumberChange = (e) => {
    const { name, value } = e.target;
    const cleaned = value.replace(/[^\d]/g, "");
    formik.setFieldValue(name, cleaned);
  };

  const toggleCategory = (category) => {
    const currentCategories = searchParams.getAll("category");
    const newCategories = currentCategories.includes(category)
      ? currentCategories.filter((c) => c !== category)
      : [...currentCategories, category];

    const newParams = new URLSearchParams(searchParams);
    newParams.delete("category");
    newCategories.forEach((c) => newParams.append("category", c));
    setSearchParams(newParams);
  };

  const formik = useFormik({
    initialValues: {
      minPrice: null,
      maxPrice: null,
    },
    validationSchema: validationSchema,
    onSubmit: (values) => {
      const newParams = new URLSearchParams(searchParams);
      if (values.minPrice !== null) {
        newParams.set("min_price", values.minPrice);
      } else {
        newParams.delete("min_price");
      }
      if (values.maxPrice !== null) {
        newParams.set("max_price", values.maxPrice);
      } else {
        newParams.delete("max_price");
      }
      setSearchParams(newParams);
    },
    validateOnChange: true,
    validateOnBlur: true,
  });
  return (
    <Sheet sx={{ p: 2 }}>
      <ResultsResumee
        totalItems={results.total_items}
        currentFilters={currentFilters}
        query={query}
      />
      <Typography level="body-lg" sx={{ fontWeight: "lg", mb: 1 }}>
        Categorias
      </Typography>
      <Box
        role="group"
        aria-labelledby="fav-movie"
        sx={{ display: "flex", flexWrap: "wrap", gap: 1 }}
      >
        {Array.isArray(results.all_categories) &&
          results.all_categories.map((name) => {
            const checked = currentFilters.categories.includes(name);
            return (
              <Chip
                key={name}
                variant="outlined"
                color={checked ? "primary" : "neutral"}
                startDecorator={
                  checked && (
                    <CheckIcon sx={{ zIndex: 1, pointerEvents: "none" }} />
                  )
                }
              >
                <Checkbox
                  variant="outlined"
                  color={checked ? "black" : "neutral"}
                  disableIcon
                  overlay
                  label={name}
                  checked={checked}
                  onChange={() => toggleCategory(name)}
                />
              </Chip>
            );
          })}
      </Box>
      <Divider sx={{ my: 2 }} />
      <Typography level="body-lg" sx={{ fontWeight: "lg" }}>
        Precio
      </Typography>
      <form
        style={{
          display: "flex",
          gap: 2,
          alignItems: "center",
          width: "auto",
          marginTop: 8,
        }}
        onSubmit={formik.handleSubmit}
      >
        <FormControl sx={{ minWidth: 40, maxWidth: 100, width: "100%" }}>
          <Input
            name="minPrice"
            value={formik.values.minPrice ?? ""}
            onChange={handleNumberChange}
            placeholder="Minimo"
            size="sm"
          />
        </FormControl>

        <FormControl sx={{ minWidth: 40, maxWidth: 100, width: "100%" }}>
          <Input
            name="maxPrice"
            value={formik.values.maxPrice ?? ""}
            onChange={handleNumberChange}
            placeholder="Maximo"
            size="sm"
          />
        </FormControl>
        <IconButton
          type="submit"
          variant="solid"
          color="danger"
          size="sm"
          sx={{
            borderRadius: "50%",
            width: 32,
            height: 32,
            minWidth: 32,
            minHeight: 32,
            p: 0,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <KeyboardArrowRight />
        </IconButton>
      </form>
      {(formik.errors.minPrice || formik.errors.maxPrice) && (
        <FormHelperText level="body-sm" color="danger" sx={{ mb: 1 }}>
          {formik.errors.minPrice || formik.errors.maxPrice}
        </FormHelperText>
      )}
    </Sheet>
  );
};

export default Filters;
