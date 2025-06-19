import React, { useEffect, useState } from "react";
import useAuth from "../utils/useAuth";
import { Button, Snackbar, Sheet, Typography, Stack } from "@mui/joy";
import ErrorOutlineIcon from "@mui/icons-material/ErrorOutline";
import PlaylistAddCheckCircleRoundedIcon from "@mui/icons-material/PlaylistAddCheckCircleRounded";
import UnansweredQuestion from "./UnansweredQuestion";

const QAPanel = () => {
  const { post } = useAuth();
  const [openSnack, setOpenSnack] = useState(false);
  const [status, setStatus] = useState({ isError: false, message: "" });
  const [unansweredQuestions, setUnansweredQuestions] = useState([]);
  const [refreshUnansweredQuestions, setRefreshUnansweredQuestions] =
    useState(false);

  async function getUnansweredQuestions() {
    try {
      const { data } = await post("/getunansweredquestions");
      console.log("Unanswered Questions:", data);
      setUnansweredQuestions(data.questions);
    } catch (error) {
      console.error("Error fetching unanswered questions:", error);
    }
  }

  useEffect(() => {
    getUnansweredQuestions();
  }, [refreshUnansweredQuestions]);

  return (
    <>
      {unansweredQuestions && unansweredQuestions.length > 0 ? (
        <Stack spacing={2} sx={{ p: 2 }}>
          {unansweredQuestions.map((question) => (
            <UnansweredQuestion
              questionData={question}
              setOpenSnack={setOpenSnack}
              setStatus={setStatus}
              key={question.question_id}
              setRefreshUnansweredQuestions={setRefreshUnansweredQuestions}
            />
          ))}
        </Stack>
      ) : (
        <Typography level="h4" sx={{ textAlign: "center", p: 2 }}>
          No hay preguntas sin responder en este momento.
        </Typography>
      )}

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

export default QAPanel;
