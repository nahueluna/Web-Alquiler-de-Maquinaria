import { Box, Sheet, Typography, Stack, IconButton } from "@mui/joy";
import ThumbUpIcon from "@mui/icons-material/ThumbUp";
import SubdirectoryArrowRightIcon from "@mui/icons-material/SubdirectoryArrowRight";
import useAuth from "../utils/useAuth";
import userContext from "../../context/UserContext";
import { useContext } from "react";
import { useState } from "react";
const Question = ({ question, setOpenSnack, setStatus }) => {
  const { post } = useAuth();
  const { user } = useContext(userContext);
  const [loadingUpvote, setLoadingUpvote] = useState(false);

  const handleUpvote = async () => {
    setLoadingUpvote(true);
    if (!user) {
      setStatus({
        isError: true,
        message: "Tenes que iniciar sesion para votar este comentario.",
      });
      setOpenSnack(true);
    } else if (user.pub_user.role != 2) {
      setStatus({
        isError: true,
        message: "Solo los clientes pueden votar preguntas.",
      });
      setOpenSnack(true);
    } else {
      try {
        const response = await post("/votequestion", {
          question_id: question.question_id,
          upvote: !question.upvoted,
        });
        if (response.status === 201) {
          // Se supone que va a cambiar el boton porque es re dinamico react
          // PD: Creo que no deberia haber incongruencia si se pone a spamear
          question.upvoted = !question.upvoted;
          question.upvotes += question.upvoted ? 1 : -1;
        }
      } catch (error) {
        let errorMsg = "Ocurrio un error con el servidor. Intentalo más tarde.";
        if (error.response) {
          switch (error.response.status) {
            case 403:
              errorMsg = "Solo los clientes pueden votar.";
              break;
            case 401:
              errorMsg = "Hubo un error al intentar verificar tu sesion.";
              break;
            case 400:
              errorMsg = "La pregunta que intentas votar ya no existe.";
              break;
            default:
              errorMsg = errorMsg;
          }
        }
        setStatus({ isError: true, message: errorMsg });
        setOpenSnack(true);
      }
    }
    setLoadingUpvote(false);
  };

  return (
    <Sheet
      sx={{
        width: "80%",
        maxWidth: "800px",
        wordBreak: "break-word",
      }}
    >
      <Box>
        <Stack
          direction="row"
          spacing={1}
          sx={{
            alignItems: "center",
            justifyContent: "space-between",
          }}
        >
          <Typography level="title-md" fontWeight="xs" sx={{ width: "90%" }}>
            {question.content}
          </Typography>
          <Box sx={{ display: "flex", alignItems: "center" }}>
            <IconButton
              color={question.upvoted ? "success" : "neutral"}
              disabled={loadingUpvote}
              onClick={() => handleUpvote()}
            >
              <ThumbUpIcon />
            </IconButton>
            {question.upvotes}
          </Box>
        </Stack>
        <Typography level="body-sm" fontWeight="md">
          {question.user_name} {question.user_surname} -{" "}
          {new Date(question.created_at).toLocaleString([], {
            year: "numeric",
            month: "2-digit",
            day: "2-digit",
            hour: "2-digit",
            minute: "2-digit",
          })}
        </Typography>
      </Box>
      {question.answer && (
        <Stack direction="row" sx={{ mt: 1, pl: 2 }}>
          {/* Responsive una legumbre */}
          <SubdirectoryArrowRightIcon sx={{ mt: 0.5 }} />
          <Box>
            <Typography level="title-md" fontWeight={300} textColor="#989191">
              {question.answer.content}
            </Typography>
            <Typography level="body-sm" fontWeight={300} textColor="#989191">
              {question.answer.user_name} {question.answer.user_surname} -{" "}
              {new Date(question.answer.created_at).toLocaleString([], {
                year: "numeric",
                month: "2-digit",
                day: "2-digit",
                hour: "2-digit",
                minute: "2-digit",
              })}
            </Typography>
          </Box>
        </Stack>
      )}
    </Sheet>
  );
};

export default Question;
