import Sheet from "@mui/joy/Sheet";
import ErrorImage from "../assets/404.png";

const NotFoundPage = () => {
  return (
    <Sheet
      sx={{
        width: "50%",
        height: "100%",
        display: "flex",
        justifyContent: "center",
      }}
    >
      <img
        src={ErrorImage}
        alt="404 Not Found"
        style={{ width: "150%", height: "100%" }}
      />
    </Sheet>
  );
};

export default NotFoundPage;
