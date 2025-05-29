import {
  Box,
  Button,
  FormControl,
  Input,
  Sheet,
  Stack,
  Textarea,
  Typography,
} from "@mui/joy";

import { useState } from "react";
import MoneyInput from "../utils/MoneyInput";

const RegisterMachineForm = ({ setRegisterForm }) => {
  const [images, setImages] = useState([]);
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
          e.preventDefault();
        }}
      >
        <Stack spacing={2}>
          <Typography level="h4">Registrar nuevo modelo</Typography>
          <Stack direction="row" spacing={2} flexWrap="wrap">
            <FormControl sx={{ flex: 1, minWidth: 150, maxWidth: 200 }}>
              <Input
                name="marca"
                placeholder="Marca"
                required
                sx={{ width: "100%" }}
              />
            </FormControl>
            <FormControl sx={{ flex: 1, minWidth: 150, maxWidth: 200 }}>
              <Input
                name="modelo"
                placeholder="Modelo"
                required
                sx={{ width: "100%" }}
              />
            </FormControl>
            <FormControl sx={{ flex: 1, minWidth: 100, maxWidth: 120 }}>
              <Input
                name="año"
                placeholder="Año"
                required
                sx={{ width: "100%" }}
              />
            </FormControl>
          </Stack>
          <Stack direction="row" spacing={2} flexWrap="wrap">
            <FormControl sx={{ flex: 1, minWidth: 150, maxWidth: 200 }}>
              <MoneyInput />
            </FormControl>
            <FormControl sx={{ flex: 1, minWidth: 150, maxWidth: 200 }}>
              <Input
                name="etiqueta"
                placeholder="Etiqueta"
                required
                sx={{ width: "100%" }}
              />
            </FormControl>
          </Stack>
          <FormControl>
            <Textarea
              name="politica"
              placeholder="Política de cancelación"
              required
              minRows={2}
              maxRows={3}
              sx={{ width: "100%" }}
            />
          </FormControl>
          <FormControl>
            <Textarea
              minRows={2}
              maxRows={3}
              name="descripcion"
              placeholder="Descripción general"
              required
              sx={{ width: "100%" }}
            />
          </FormControl>
          <FormControl>
            <Typography level="body-sm" sx={{ mb: 0.5 }}>
              Imágenes {images.length > 0 && `(subidas: ${images.length})`}
            </Typography>
            <Input
              type="file"
              multiple
              onChange={(e) => {
                const files = Array.from(e.target.files);
                setImages((prev) => {
                  const all = [...prev, ...files];
                  return all.filter(
                    (file, idx, arr) =>
                      arr.findIndex((f) => f.name === file.name) === idx
                  );
                });
              }}
            />
            <Stack spacing={1} sx={{ mt: 1 }}>
              {images &&
                images.map((file, idx) => (
                  <Box
                    key={idx}
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
                      onClick={() => {
                        setImages(images.filter((_, i) => i !== idx));
                      }}
                    >
                      Quitar
                    </Button>
                  </Box>
                ))}
            </Stack>
          </FormControl>
          <Stack direction="row" spacing={2} sx={{ mt: 2 }}>
            <Button type="submit" color="danger" variant="solid">
              Registrar
            </Button>
            <Button
              variant="plain"
              onClick={() => {
                setRegisterForm(false);
                setImages([]);
              }}
            >
              Cancelar
            </Button>
          </Stack>
        </Stack>
      </form>
    </Sheet>
  );
};

export default RegisterMachineForm;
