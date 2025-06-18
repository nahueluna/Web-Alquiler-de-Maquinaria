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
} from "@mui/joy";
import axios from "axios";
import { useEffect } from "react";
import { useContext } from "react";
import UserContext from "../../context/UserContext";
import Question from "./Question";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

const QASection = ({ id: id }) => {
  const { user } = useContext(UserContext);
  const access = user?.access || null;
  const [loading, setLoading] = React.useState(true);
  const [refreshQuestions, setRefreshQuestions] = React.useState(false);
  const [QAData, setQAData] = React.useState([]);

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
            <Input placeholder="Escribi tu pregunta" size="lg" />
          </FormControl>
          <FormControl>
            <Button color="danger" size="lg">
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
          QAData.map((question) => <Question question={question} />)
        ) : (
          <Typography level="body2" sx={{ p: 2, textAlign: "center" }}>
            Todavia no hay preguntas. ¡Hacenos la primera pregunta!
          </Typography>
        )}
      </Stack>
    </Stack>
  );
};

export default QASection;
