import React from "react";
import {
  Box,
  Stack,
  Sheet,
  Typography,
  Chip,
  Card,
  Textarea,
  FormControl,
  FormHelperText,
  Button,
} from "@mui/joy";
import { useState } from "react";
import useAuth from "../utils/useAuth";

// CONTENIDO, FECHA, NOMBRE Y APELLIDO, MARCA, NOMBRE, MODELO
// MODEL ID Y QUESTION ID

const UnansweredQuestion = ({
  questionData,
  setStatus,
  setOpenSnack,
  setRefreshUnansweredQuestions,
}) => {
  const { post } = useAuth();
  const [answer, setAnswer] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleChange = (e) => {
    const value = e.target.value;
    setAnswer(e.target.value);
    if (value.trim() === "") {
      setError("La respuesta no puede estar vacía.");
      return;
    } else if (value.length > 256) {
      setError("La respuesta no puede tener más de 256 caracteres.");
      return;
    }
    setError("");
    console.log(value);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (answer.trim() === "") {
      setError("La respuesta no puede estar vacía.");
      return;
    } else if (answer.length > 256) {
      setError("La respuesta no puede tener más de 256 caracteres.");
      return;
    }
    console.log("Enviando respuesta:", answer);
    setLoading(true);
    try {
      await post("/newanswer", {
        question_id: questionData.question_id,
        content: answer,
      });
      setAnswer("");
      setStatus({
        isError: false,
        message: "Respuesta enviada correctamente.",
      });
      setOpenSnack(true);
      setRefreshUnansweredQuestions((prev) => !prev);
    } catch (error) {
      let errorMsg = "Ocurrio un error con el servidor. Intentalo más tarde.";
      if (error.response) {
        switch (error.response.status) {
          case 403:
            errorMsg =
              "No tenes los permisos necesarios para realizar la accion.";
            break;
          case 401:
            errorMsg = "Hubo un error al intentar verificar tu sesion.";
            break;
          case 400:
            errorMsg =
              "La pregunta ya fue respondida o la respuesta excede el limite de caracteres.";
            break;
          default:
            errorMsg = errorMsg;
        }
      }
      setStatus({ isError: true, message: errorMsg });
      setOpenSnack(true);
    }
    setLoading(false);
  };

  return (
    <Sheet variant="outlined" sx={{ p: 2, borderRadius: "md", maxWidth: 600 }}>
      <Stack spacing={2}>
        <Stack direction="row" sx={{ justifyContent: "space-between" }}>
          <Chip color="danger" size="lg" variant="outlined">
            {new Date(questionData.created_at).toLocaleString([], {
              year: "numeric",
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit",
            })}
          </Chip>
          <Chip color="danger" size="lg" variant="soft">
            {questionData.user_name} {questionData.user_surname}
          </Chip>
        </Stack>
        <Box>
          <Typography level="title-lg">
            {questionData.model_name} {questionData.model_model}
          </Typography>
          <Typography level="body-md">{questionData.model_brand}</Typography>
        </Box>
        <Card
          variant="soft"
          sx={{
            backgroundColor: "#f6f2f2",
          }}
        >
          <Typography level="body-md" sx={{ wordBreak: "break-word" }}>
            "{questionData.content}"
          </Typography>
        </Card>
        <form onSubmit={handleSubmit}>
          <Stack direction="row" spacing={1}>
            <FormControl sx={{ flexGrow: 1 }} error={!!error}>
              <Textarea
                minRows={3}
                maxRows={3}
                placeholder="Escribí tu respuesta"
                value={answer}
                onChange={handleChange}
                disabled={loading}
                endDecorator={
                  <Box
                    sx={{
                      display: "flex",
                      gap: "var(--Textarea-paddingBlock)",
                      pt: "var(--Textarea-paddingBlock)",
                      flex: "auto",
                    }}
                  >
                    <FormControl sx={{ ml: "auto" }}>
                      <Button
                        color="danger"
                        type="submit"
                        disabled={
                          loading || answer.length < 1 || answer.length > 256
                        }
                      >
                        Responder
                      </Button>
                    </FormControl>
                  </Box>
                }
              />
              <FormHelperText>{error || `${answer.length}/256`}</FormHelperText>
            </FormControl>
          </Stack>
        </form>
      </Stack>
    </Sheet>
  );
};

export default UnansweredQuestion;
