import { Skeleton } from "@mui/joy";
import AspectRatio from "@mui/joy/AspectRatio";
import Box from "@mui/joy/Box";
import Card from "@mui/joy/Card";
import CardContent from "@mui/joy/CardContent";
import CardOverflow from "@mui/joy/CardOverflow";
import Chip from "@mui/joy/Chip";
import Divider from "@mui/joy/Divider";
import Typography from "@mui/joy/Typography";
import React from "react";

export default function MachineCard({
  imageUrl,
  model,
  categories,
  price,
  onClick,
}) {
  const [loading, setLoading] = React.useState(true);

  return (
    <Card
      variant={"outlined"}
      onClick={onClick}
      sx={{
        cursor: "pointer",
        transition: "transform 0.2s, box-shadow 0.2s",
        transform: "scale(1)",
        "&:hover": {
          transform: "scale(1.04)",
          boxShadow: "0 4px 20px rgba(0,0,0,0.15)",
        },
      }}
    >
      <CardOverflow>
        <AspectRatio ratio="4/3">
          <Skeleton loading={loading}>
            <img
              src={imageUrl}
              loading="lazy"
              alt=""
              onLoad={() => setLoading(false)}
            />
          </Skeleton>
        </AspectRatio>
      </CardOverflow>
      <CardContent>
        <Typography
          level="title-md"
          sx={{
            display: "-webkit-box",
            WebkitLineClamp: 2,
            WebkitBoxOrient: "vertical",
            overflow: "hidden",
            textOverflow: "ellipsis",
            minHeight: "3em",
            lineHeight: 1.5,
          }}
        >
          {model}
        </Typography>
        <Box sx={{ minHeight: "4em" }}>
          {categories.map((category, index) => (
            <Chip
              key={index}
              variant="soft"
              size="sm"
              color="danger"
              sx={{ mr: 0.5, mb: 0.5 }}
            >
              {category.name}
            </Chip>
          ))}
        </Box>
      </CardContent>
      <CardOverflow variant="soft" sx={{ bgcolor: "background.level1" }}>
        <Divider inset="context" />
        <CardContent orientation="horizontal">
          <Typography
            level="body-xs"
            textColor="text.secondary"
            sx={{ fontWeight: "md" }}
          >
            ARS ${price}
            <Typography
              level="body-xs"
              textColor="grey"
              sx={{ fontWeight: "md" }}
            >
              {" "}
              por día
            </Typography>
          </Typography>
        </CardContent>
      </CardOverflow>
    </Card>
  );
}
