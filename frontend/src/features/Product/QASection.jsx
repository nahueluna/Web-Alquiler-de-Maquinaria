import React from "react";
import {
  Button,
  FormControl,
  Input,
  Sheet,
  Typography,
  Stack,
  Textarea,
  Box,
  Snackbar,
  FormHelperText,
  Dropdown,
  Menu,
  MenuButton,
  MenuItem,
  IconButton,
} from "@mui/joy";
import useAuth from "../utils/useAuth";
import { useEffect } from "react";
import { useContext } from "react";
import UserContext from "../../context/UserContext";
import { useState } from "react";
import Question from "./Question";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";
import ArrowDropDown from "@mui/icons-material/ArrowDropDown";

const QASection = ({ id: id }) => {
  const { user } = useContext(UserContext);
  const { post } = useAuth();
  const [loading, setLoading] = React.useState(true);
  const [QAData, setQAData] = React.useState([]);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const [question, setQuestion] = useState("");
  const [error, setError] = useState("");
  const [open, setOpen] = useState(false);
  const [selected, setSelected] = useState(1);

  async function fetchQuestions(orderByRecent = false) {
    setLoading(true);
    try {
      const parameters = {
        model_id: parseInt(id),
        order_by_recent: orderByRecent,
      };

      const { data } = await post(`/getquestions`, parameters);
      console.log(data);
      setQAData(data.questions);
      console.log(QAData);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  }

  const handleChange = (e) => {
    const value = e.target.value;
    setQuestion(e.target.value);
    if (value && value.trim() === "") {
      setError("La pregunta no puede estar vacía.");
      return;
    } else if (value.length > 256) {
      setError("La pregunta no puede tener más de 256 caracteres.");
      return;
    }
    setError("");
    console.log(value);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!user) {
      setOpenSnack(true);
      setStatus({
        isError: true,
        message: "Tenes que iniciar sesion para hacer una pregunta.",
      });
      return;
    }

    // Posiblemente no tenga sentido ponerlo doble, pero quien sabe
    if (!question || question.trim() === "") {
      setError("La pregunta no puede estar vacía.");
      return;
    } else if (question.length > 256) {
      setError("La pregunta no puede tener más de 256 caracteres.");
      return;
    }

    setError("");
    setLoading(true);
    try {
      await post("/newquestion", {
        model_id: parseInt(id),
        content: question,
      });
      setOpenSnack(true);
      setStatus({
        isError: false,
        message: "Pregunta enviada exitosamente.",
      });
      setQuestion("");
      fetchQuestions(selected === 1 ? false : true);
    } catch (error) {
      let errorMsg = "Ocurrio un error con el servidor. Intentalo más tarde.";
      if (error.response) {
        switch (error.response.status) {
          case 403:
            errorMsg = "Solo los clientes pueden hacer preguntas.";
            break;
          case 401:
            errorMsg = "Hubo un error al intentar verificar tu sesion.";
            break;
          default:
            errorMsg = errorMsg;
        }
      }
      setStatus({ isError: true, message: errorMsg });
      setOpenSnack(true);
    }
    setLoading(false);
    console.log(question);
  };

  const handleSortBy = (sortType) => () => {
    setSelected(sortType);
    setOpen(false);
    if (sortType === 1) {
      fetchQuestions(false);
    } else if (sortType === 2) {
      fetchQuestions(true);
    }
  };

  useEffect(() => {
    fetchQuestions();
  }, []);

  return (
    <>
      <Stack
        spacing={1}
        sx={{
          minWidth: "900px",
          p: 2,
        }}
      >
        <Typography level="h2">Preguntas</Typography>
        {user && user.pub_user.role === 2 && (
          <form onSubmit={handleSubmit}>
            <Stack direction="row" spacing={1}>
              <FormControl sx={{ width: "70%" }} error={!!error}>
                <Input
                  placeholder="Escribi tu pregunta"
                  size="lg"
                  value={question}
                  onChange={(e) => handleChange(e)}
                  disabled={loading || (user && user.pub_user.role !== 2)}
                  maxLength={256}
                />
                <FormHelperText>{error || ""}</FormHelperText>
              </FormControl>
              <FormControl>
                {/* Me estoy cagando en el FormHelperText cuando es menos de 1 caracter y todos los snackbar
                  al deshabilitar el boton de pregunta pero bueno, ing2 */}
                <Button
                  color="danger"
                  size="lg"
                  type="submit"
                  disabled={
                    loading ||
                    (user && user.pub_user.role !== 2) ||
                    question.length < 1 ||
                    question.length > 256
                  }
                >
                  Preguntar
                </Button>
              </FormControl>
            </Stack>
          </form>
        )}
        <Box
          sx={{
            width: "20%",
            alignSelf: "flex-start",
          }}
        >
          <Dropdown open={open} onOpenChange={() => setOpen(!open)}>
            <MenuButton disabled={loading} endDecorator={<ArrowDropDown />}>
              Ordenar por...
            </MenuButton>
            <Menu>
              <MenuItem selected={selected === 1} onClick={handleSortBy(1)}>
                Mas valoradas
              </MenuItem>
              <MenuItem selected={selected === 2} onClick={handleSortBy(2)}>
                Mas recientes
              </MenuItem>
            </Menu>
          </Dropdown>
        </Box>
        <Stack
          spacing={3}
          sx={{
            p: 2,
          }}
        >
          {QAData?.length > 0 ? (
            QAData.map((question) => (
              <Question
                question={question}
                setOpenSnack={setOpenSnack}
                setStatus={setStatus}
              />
            ))
          ) : (
            <Typography level="body2" sx={{ p: 2, textAlign: "center" }}>
              Todavia no hay preguntas. ¡Hacenos la primera pregunta!
            </Typography>
          )}
        </Stack>
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

export default QASection;
