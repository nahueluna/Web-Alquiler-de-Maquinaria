import { Box, Sheet, Typography, Stack, IconButton } from "@mui/joy";
import ThumbUpIcon from "@mui/icons-material/ThumbUp";
import SubdirectoryArrowRightIcon from "@mui/icons-material/SubdirectoryArrowRight";

const Question = ({ question }) => {
  return (
    <Sheet
      sx={{
        width: "80%",
        maxWidth: "800px",
        backgroundColor: "white",
      }}
    >
      <Stack direction="row">
        <Box>
          <Typography level="title-md" fontWeight="xs">
            {question.content}
          </Typography>
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
        <Box
          sx={{
            ml: "auto",
            display: "flex",
            alignItems: "center",
          }}
        >
          <IconButton>
            <ThumbUpIcon />
          </IconButton>
          {question.upvotes}
        </Box>
      </Stack>
      {question.answer && (
        <Stack direction="row" sx={{ mt: 1, pl: 2 }}>
          {/* Responsive una legumbre */}
          <SubdirectoryArrowRightIcon sx={{ mt: 0.5 }} />
          <Box>
            <Typography level="title-md" fontWeight={300} textColor="#b6aeae">
              {question.answer.content}
            </Typography>
            <Typography level="body-sm" fontWeight={300} textColor="#b6aeae">
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
