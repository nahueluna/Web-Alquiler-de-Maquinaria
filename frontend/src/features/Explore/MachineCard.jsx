  import { Skeleton } from "@mui/joy";
  import AspectRatio from "@mui/joy/AspectRatio";
  import Box from "@mui/joy/Box";
  import Card from "@mui/joy/Card";
  import CardContent from "@mui/joy/CardContent";
  import CardOverflow from "@mui/joy/CardOverflow";
  import Chip from "@mui/joy/Chip";
  import Divider from "@mui/joy/Divider";
  import Typography from "@mui/joy/Typography";
  import React, { useEffect, useState, useContext } from "react";
  import Rating from "./Rating";
  import useAuth from "../utils/useAuth";
  import UserContext from "../../context/UserContext"

  const reviews = {
    totalReviews: 53,
    rating: 4.5,
  };

  export default function MachineCard({
    imageUrl,
    model,
    model_id,
    name,
    categories,
    price,
    onClick,
  }) {
    const [loading, setLoading] = React.useState(true);
    const { get, post } = useAuth();
    const { user } = useContext(UserContext);
    const token = user?.access || "";
    const [reviewData, setReviewData] = useState(null);
    console.log("TOKEN: ", token);
    console.log("MODEL: ", model);
    console.log("MODEL_ID: ", model_id);

  useEffect(() => {
    const fetchReviews = async () => {
      try {
        const payload = {
          model_id: model_id,
          order: "recent",         // opcional: default explícito
        };

        const res = await post("reviews/machines/get", payload, {
          withCredentials: true,
        });

        const { average_rating, reviews } = res.data;

        setReviewData({
          rating: average_rating,
          totalReviews: reviews.length,
        });
      } catch (err) {
        console.error("Error al obtener reseñas:", err);
        if (err.response) {
          console.log("Respuesta del servidor:", err.response.data);
        }
      }
    };

    if (model) fetchReviews();
  }, [model]);


    return (
      <Card
        variant={"outlined"}
        onClick={onClick}
        sx={{
          cursor: "pointer",
          "& *": { cursor: "pointer" }, // aplico a todos los hijos si no Joy no aplica el cursor,
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
            {name} {model}
          </Typography>

            {reviewData && <Rating reviews={reviewData} />}

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
