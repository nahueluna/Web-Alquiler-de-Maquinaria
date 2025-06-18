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
} from "@mui/joy";
import axios from "axios";
import { useEffect } from "react";
import { useContext } from "react";
import UserContext from "../../context/UserContext";
import { useState } from "react";
import Question from "./Question";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

const QASection = ({ id: id }) => {
  const { user } = useContext(UserContext);
  const access = user?.access || null;
  const [loading, setLoading] = React.useState(true);
  const [refreshQuestions, setRefreshQuestions] = React.useState(false);
  const [QAData, setQAData] = React.useState([]);
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });

  useEffect(() => {
    async function fetchMachine() {
      try {
        const parameters = {
          model_id: parseInt(id),
          order_by_recent: true,
        };

        if (access) {
          parameters.access = access;
        }
        const { data } = await axios.post(
          `${BACKEND_URL}/getquestions`,
          parameters
        );
        console.log(data);
        setQAData(data.questions);
        console.log(QAData);
      } catch (error) {
        console.error(error);
      } finally {
        setLoading(false);
      }
    }
    fetchMachine();
  }, [refreshQuestions]);

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
        <form>
          <Stack direction="row" spacing={1}>
            <FormControl sx={{ width: "70%" }}>
              <Input
                placeholder="Escribi tu pregunta"
                size="lg"
                disabled={loading}
              />
            </FormControl>
            <FormControl>
              <Button color="danger" size="lg" disabled={loading}>
                Preguntar
              </Button>
            </FormControl>
          </Stack>
        </form>
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
            Dismiss
          </Button>
        }
      >
        {status.message}
      </Snackbar>
    </>
  );
};

export default QASection;
