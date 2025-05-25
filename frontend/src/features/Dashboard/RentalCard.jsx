import Card from "@mui/joy/Card";
import CardContent from "@mui/joy/CardContent";
import CardOverflow from "@mui/joy/CardOverflow";
import Divider from "@mui/joy/Divider";
import Typography from "@mui/joy/Typography";

export default function RentalCard({ model, renter, startDate, finished }) {
  return (
    <Card
      variant={"outlined"}
      sx={{
        cursor: "pointer",
        transition: "transform 0.2s, box-shadow 0.2s",
        transform: "scale(1)",
        "&:hover": {
          transform: "scale(1.01)",
          boxShadow: "0 4px 10px rgba(0,0,0,0.15)",
        },
      }}
    >
      <CardContent>
        <Typography level="title-lg">{model}</Typography>
        <Typography level="body-sm" textColor="grey">
          Alquilado por {renter}
        </Typography>
        <Typography level="body-sm"></Typography>
      </CardContent>
      <CardOverflow variant="soft" sx={{ bgcolor: "background.level1" }}>
        <Divider inset="context" />
        <CardContent orientation="horizontal">
          <Typography
            level="body-xs"
            textColor="text.secondary"
            sx={{ fontWeight: "md" }}
          >
            Fecha de inicio {startDate}
          </Typography>
          <Divider orientation="vertical" sx={{ mx: 1 }} />
          <Typography>
            {finished ? "Alquiler terminado" : "Alquiler activo"}
          </Typography>
        </CardContent>
      </CardOverflow>
    </Card>
  );
}
