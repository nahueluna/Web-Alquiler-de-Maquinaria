import React from "react";
import EmptyTractor from "../../assets/EmptyTractor.png";
import FullTractor from "../../assets/FullTractor.png";
import HalfTractor from "../../assets/HalfTractor.png";
import { Box, Stack, Typography } from "@mui/material";

const getRatingTractors = (rating) => {
  const fullTractor = Math.floor(rating);
  const emptyTractor = Math.floor(5 - rating);
  const halfTractor = rating - fullTractor;
  const tractors = [];

  for (let i = 0; i < fullTractor; i++) {
    tractors.push(
      <img key={`full-${i}`} src={FullTractor} alt="Full Tractor" width={20} height={20} />
    );
  }

  if (halfTractor > 0.1 && halfTractor < 0.9) {
    tractors.push(
      <img key="half" src={HalfTractor} alt="Half Tractor" width={20} height={20} />
    );
  }

  for (let i = 0; i < emptyTractor; i++) {
    tractors.push(
      <img key={`empty-${i}`} src={EmptyTractor} alt="Empty Tractor" width={20} height={20} />
    );
  }

  return tractors;
};

const Rating = ({ reviews }) => {
  const roundedRating =
    reviews.rating % 1 === 0
      ? reviews.rating.toFixed(0)
      : reviews.rating.toFixed(1);

  return (
    <Box>
      <Stack direction="row" spacing={1} alignItems="center">
        {/* Promedio numérico */}
        <Typography fontSize="1rem" fontWeight="bold">
          {roundedRating}
        </Typography>

        {/* Iconos de tractor */}
        <Box sx={{ display: "flex", alignItems: "center", gap: 0.5 }}>
          {getRatingTractors(reviews.rating)}
        </Box>

        {/* Total entre paréntesis */}
        <Typography fontSize="0.875rem" color="text.secondary">
          ({reviews.totalReviews})
        </Typography>
      </Stack>
    </Box>
  );
};


export default Rating;
