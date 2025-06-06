import { Skeleton } from "@mui/joy";
import AspectRatio from "@mui/joy/AspectRatio";
import Card from "@mui/joy/Card";
import CardContent from "@mui/joy/CardContent";
import CardOverflow from "@mui/joy/CardOverflow";
import Divider from "@mui/joy/Divider";
import Typography from "@mui/joy/Typography";
import React from "react";

export default function MachineCardSkeleton() {
  return (
    <Card
      variant={"outlined"}
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
          <Skeleton />
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
          <Skeleton />
        </Typography>
        <Typography level="body-sm">
          <Skeleton />
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
            <Skeleton>
              ARS $
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
