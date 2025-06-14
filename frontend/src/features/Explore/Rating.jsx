import React from "react";
import EmptyTractor from "../../assets/EmptyTractor.png";
import FullTractor from "../../assets/FullTractor.png";
import HalfTractor from "../../assets/HalfTractor.png";
import { Box, Stack } from "@mui/material";

const getRatingTractors = (rating) => {
  const fullTractor = Math.floor(rating);
  const emptyTractor = Math.floor(5 - rating);
  const halfTractor = rating - fullTractor;
  const tractors = [];

  for (let i = 0; i < fullTractor; i++) {
    tractors.push(
      <img src={FullTractor} alt="Full Tractor" width={20} height={20} />
    );
  }

  halfTractor > 0.1 &&
    halfTractor < 0.9 &&
    tractors.push(
      <img src={HalfTractor} alt="Half Tractor" width={20} height={20} />
    );

  for (let i = 0; i < emptyTractor; i++) {
    tractors.push(
      <img src={EmptyTractor} alt="Empty Tractor" width={20} height={20} />
    );
  }

  return tractors;
};

const Rating = ({ reviews }) => {
  return (
    <Box>
      <Stack direction="row" spacing={1}>
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            gap: 0.5,
            fontSize: "1rem",
          }}
        >
          {getRatingTractors(reviews.rating).map((tractor) => (
            <>{tractor}</>
          ))}
        </Box>
        <Box sx={{ fontSize: "0.875rem", color: "text.secondary" }}>
          {reviews.totalReviews}
        </Box>
      </Stack>
    </Box>
  );
};

export default Rating;
