import { Skeleton } from "@mui/joy";
import AspectRatio from "@mui/joy/AspectRatio";
import Card from "@mui/joy/Card";
import CardContent from "@mui/joy/CardContent";
import CardOverflow from "@mui/joy/CardOverflow";
import Divider from "@mui/joy/Divider";
import Typography from "@mui/joy/Typography";
import React from "react";

export default function MachineCard({ imageUrl, model, category, price }) {
  const [loading, setLoading] = React.useState(false);

  return (
    <Card
      variant="outlined"
      sx={{
        width: "100%",
        maxWidth: 320,
        minWidth: 120,
      }}
    >
      <CardOverflow>
        <AspectRatio ratio="4/3">
          <Skeleton loading={loading}>
            <img src={imageUrl} loading="lazy" alt="" />
          </Skeleton>
        </AspectRatio>
      </CardOverflow>
      <CardContent>
        <Typography level="title-md">
          <Skeleton loading={loading}>{model}</Skeleton>
        </Typography>
        <Typography level="body-sm">
          <Skeleton loading={loading}>{category}</Skeleton>
        </Typography>
      </CardContent>
      <CardOverflow variant="soft" sx={{ bgcolor: "background.level1" }}>
        <Divider inset="context" />
        <CardContent orientation="horizontal">
          <Typography
            level="body-xs"
            textColor="text.secondary"
            sx={{ fontWeight: "md" }}
          >
            <Skeleton loading={loading}>
              ARS ${price}
              <Typography
                level="body-xs"
                textColor="grey"
                sx={{ fontWeight: "md" }}
              >
                {" "}
                por día
              </Typography>
            </Skeleton>
          </Typography>
        </CardContent>
      </CardOverflow>
    </Card>
  );
}
