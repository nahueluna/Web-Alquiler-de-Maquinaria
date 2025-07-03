import {
  Box,
  Button,
  FormControl,
  Input,
  Select,
  Sheet,
  Stack,
  Textarea,
  Typography,
  Option,
  FormLabel,
  Chip,
  FormHelperText,
} from "@mui/joy";
import { useState } from "react";
import MoneyInput from "../utils/MoneyInput";
import useAuth from "../utils/useAuth";
import AddIcon from "@mui/icons-material/Add";
import IconButton from "@mui/joy/IconButton";
import ChipDelete from "@mui/joy/ChipDelete";

const RegisterMachineForm = ({
  categories,
  setRegisterForm,
  setOpenSnack,
  setStatus,
  setRefreshMachines,
}) => {
  const { post } = useAuth();
  const [loading, setLoading] = useState(false);
  const [policy, setPolicy] = useState("");
  const [selectedCategories, setSelectedCategories] = useState([]);
  const [newCategory, setNewCategory] = useState("");
  const [mainImage, setMainImage] = useState(null);
  const [extraImages, setExtraImages] = useState([]);

  const normalize = (str) =>
    str
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLocaleLowerCase();

  const handleAddCategory = () => {
    const cat = newCategory.trim();
    if (
      cat &&
      !selectedCategories.some((c) => normalize(c) === normalize(cat)) &&
      !categories.some((c) => normalize(c) === normalize(cat))
    ) {
      setSelectedCategories([...selectedCategories, cat]);
    }
    setNewCategory("");
  };

  const handleSelectCategory = (event, value) => {
    if (value && !selectedCategories.includes(value)) {
      setSelectedCategories([...selectedCategories, value]);
    }
  };

  const handleRemoveCategory = (cat) => {
    setSelectedCategories(selectedCategories.filter((c) => c !== cat));
  };

  // ============== MANEJO DE IMAGENES ==============

  const handleMainImageChange = (e) => {
    const file = e.target.files[0];
    if (file) setMainImage(file);
  };

  const handleExtraImagesChange = (e) => {
    const files = Array.from(e.target.files);
    const newFiles = files.slice(0, 9 - extraImages.length);
    setExtraImages((prev) => [...prev, ...newFiles].slice(0, 9));
  };

  const removeMainImage = () => setMainImage(null);

  const removeExtraImage = (index) =>
    setExtraImages(extraImages.filter((_, i) => i !== index));

  // No se si funciona quien te manda a hacer esto hijos de puta no queres que te haga la base de datos tambien
  const fileToBase64 = (file) =>
    new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result.split(",")[1]);
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });

  const fetchNewModel = async (data) => {
    setLoading(true);
    try {
      console.log(data);
      const response = await post("/newmodel", data);
      setOpenSnack(true);
      setStatus({
        isError: false,
        message: "Modelo registrado correctamente.",
      });
      // Refrescar lista de maquinas
      setRefreshMachines((prev) => !prev);
      // Mover todo a un solo state?
      setMainImage(null);
      setExtraImages([]);
      setSelectedCategories([]);
      setNewCategory("");
      setRegisterForm(false);
    } catch (error) {
      let msg = "Ocurrió un error. Intentalo más tarde";
      switch (error.response?.status) {
        case 403:
          msg = "No tienes permisos para realizar esta acción.";
          break;
        case 400:
          msg =
            "Hubo un error al procesar las imagenes, las imagenes son mas de 10 o ya existe un modelo con la misma marca, modelo y año.";
          break;
        case 413:
          msg = "Las imágenes son demasiado pesadas.";
          break;
        default:
          msg = "Ocurrió un error. Intentalo más tarde";
      }
      setOpenSnack(true);
      setStatus({
        isError: true,
        message: msg,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    const form = e.target;
    const name = form.name.value;
    const brand = form.brand.value;
    const model = form.model.value;
    const year = Number(form.year.value);
    const description = form.description.value;
    const price = Number(form.price.value.replace(/[^\d]/g, ""));

    let mainImageBase64 = null;
    let extraImagesBase64 = [];
    if (mainImage) mainImageBase64 = await fileToBase64(mainImage);
    if (extraImages.length > 0)
      extraImagesBase64 = await Promise.all(extraImages.map(fileToBase64));

    const data = {
      name,
      brand,
      model,
      year,
      policy: policy,
      description,
      price,
      categories: selectedCategories,
      image: mainImageBase64,
      extra_images: extraImagesBase64,
    };

    fetchNewModel(data);
  };

  return (
    <Sheet
      variant="outlined"
      sx={{
        p: 3,
        borderRadius: "sm",
        mb: 2,
        minWidth: 400,
        maxWidth: 800,
        width: 600,
      }}
    >
      <form
        onSubmit={(e) => {
          handleSubmit(e);
        }}
      >
        <Stack spacing={2}>
          <Typography level="h4">Registrar nuevo modelo</Typography>
          <FormControl>
            <FormLabel>Nombre del modelo</FormLabel>
            <Input name="name" placeholder="Nombre" maxLength={50} required />
          </FormControl>
          <FormControl>
            <FormLabel>Marca</FormLabel>
            <Input name="brand" placeholder="Marca" maxLength={50} required />
          </FormControl>
          <FormControl>
            <FormLabel>Modelo</FormLabel>
            <Input name="model" placeholder="Modelo" maxLength={50} required />
          </FormControl>
          <FormControl>
            <FormLabel>Año de fabricación</FormLabel>
            <Input
              name="year"
              placeholder="Año de fabricación"
              maxLength={4}
              inputMode="numeric"
              pattern="[0-9]*"
              onInput={(e) => {
                e.target.value = e.target.value.replace(/\D/g, "").slice(0, 4);
              }}
              required
            />
          </FormControl>
          <FormControl>
            <FormLabel>Política de cancelación</FormLabel>
            <Select
              name="policy"
              placeholder="Elegi una politica de cancelacion"
              value={policy}
              onChange={(event, newValue) => setPolicy(newValue)}
              required
            >
              <Option value="total">Total</Option>
              <Option value="parcial">Parcial</Option>
              <Option value="none">Ninguna</Option>
            </Select>
          </FormControl>
          <FormControl>
            <FormLabel>Descripción</FormLabel>
            <Textarea
              minRows={2}
              maxRows={3}
              name="description"
              placeholder="Descripción general"
              maxLength={300}
              required
            />
          </FormControl>
          <FormControl>
            <FormLabel>Precio por día</FormLabel>
            <MoneyInput
              name="price"
              placeholder="Precio por dia"
              required={true}
            />
          </FormControl>
          <FormLabel>Categorías</FormLabel>
          <Box sx={{ display: "flex", gap: 1, alignItems: "space-around" }}>
            <FormControl>
              <Select
                placeholder="Seleccionar categoría"
                value=""
                onChange={handleSelectCategory}
                sx={{ minWidth: 180 }}
              >
                {categories
                  .filter((cat) => !selectedCategories.includes(cat))
                  .map((cat, idx) => (
                    <Option key={idx} value={cat}>
                      {cat}
                    </Option>
                  ))}
              </Select>
            </FormControl>
            <FormControl>
              <Input
                placeholder="Nueva categoría"
                value={newCategory}
                onChange={(e) => setNewCategory(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleAddCategory();
                  }
                }}
                sx={{ minWidth: 140 }}
              />
            </FormControl>
            <IconButton
              variant="plain"
              color="danger"
              onClick={handleAddCategory}
              disabled={!newCategory.trim()}
            >
              <AddIcon />
            </IconButton>
          </Box>
          <Box sx={{ mt: 1, display: "flex", gap: 1, flexWrap: "wrap" }}>
            {selectedCategories.map((cat, index) => (
              <Chip
                key={index}
                color="danger"
                variant="soft"
                endDecorator={
                  <ChipDelete onDelete={() => handleRemoveCategory(cat)} />
                }
              >
                {cat}
              </Chip>
            ))}
          </Box>

          {/* Imagen principal */}
          <FormControl>
            <Typography level="body-sm" sx={{ mb: 0.5 }}>
              Imagen principal {mainImage && `(${mainImage.name})`}
            </Typography>
            <Input
              type="file"
              slotProps={{
                input: {
                  accept: "image/png, image/jpeg, image/webp",
                  onChange: handleMainImageChange,
                },
              }}
            />
            {/* Mostrar imagen principal (se reemplaza) */}
            {mainImage && (
              <Box sx={{ display: "flex", alignItems: "center", mt: 1 }}>
                <Typography level="body-sm">{mainImage.name}</Typography>
                <Button
                  size="sm"
                  color="danger"
                  variant="plain"
                  onClick={removeMainImage}
                >
                  Quitar
                </Button>
              </Box>
            )}
          </FormControl>

          {/* Imagenes extra AYUDAAAAAA */}
          <FormControl>
            <Typography level="body-sm" sx={{ mb: 0.5 }}>
              Imágenes extra (opcional){" "}
              {extraImages.length > 0 && `(${extraImages.length}/9)`}
            </Typography>
            <Input
              name="extra-images"
              type="file"
              slotProps={{
                input: {
                  accept: "image/png, image/jpeg, image/webp",
                  onChange: handleExtraImagesChange,
                },
              }}
              multiple
              disabled={extraImages.length >= 9}
            />
            {/* Mostrar imágenes extra */}
            <Stack spacing={1} sx={{ mt: 1 }}>
              {/* Mensaje para mi del futuro: cambiar show de imagenes a Chip o cambiar de carrera? */}
              {extraImages.map((file, index) => (
                <Box
                  key={index}
                  sx={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                    background: "#f7f7f7",
                    borderRadius: "sm",
                    px: 1,
                    py: 0.5,
                  }}
                >
                  <Typography level="body-sm">{file.name}</Typography>
                  <Button
                    size="sm"
                    color="danger"
                    variant="plain"
                    onClick={() => removeExtraImage(index)}
                  >
                    Quitar
                  </Button>
                </Box>
              ))}
            </Stack>
          </FormControl>
          <FormHelperText color="danger" sx={{ mt: 1 }}>
            {selectedCategories.length === 0 && !mainImage
              ? "Debes seleccionar al menos una categoría y una imagen principal."
              : selectedCategories.length === 0
              ? "Debes seleccionar al menos una categoría."
              : !mainImage
              ? "Debes seleccionar una imagen principal."
              : ""}
          </FormHelperText>
          <Stack
            direction="row"
            spacing={2}
            sx={{ mt: 2, justifyContent: "flex-end" }}
          >
            <Button
              variant="plain"
              onClick={() => {
                setRegisterForm(false);
              }}
            >
              Cancelar
            </Button>
            <Button
              type="submit"
              color="danger"
              variant="solid"
              loading={loading}
              disabled={selectedCategories.length === 0 || !mainImage}
            >
              Registrar
            </Button>
          </Stack>
        </Stack>
      </form>
    </Sheet>
  );
};

export default RegisterMachineForm;
