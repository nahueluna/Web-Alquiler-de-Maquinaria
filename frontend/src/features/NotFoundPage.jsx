import { Typography } from "@mui/joy";
import Sheet from "@mui/joy/Sheet";
import ErrorImage from "../assets/404.png";
import AspectRatio from "@mui/joy/AspectRatio";
import Stack from "@mui/joy/Stack";
import React from "react";
import Box from "@mui/joy/Box";

const NotFoundPage = () => {
  return (
    <Sheet
      sx={{
        backgroundImage: `url(${ErrorImage})`,
        backgroundSize: "cover",
        backgroundPosition: "center",
        width: "50%",
        height: "100%",
      }}
    ></Sheet>
  );
};

export default NotFoundPage;
