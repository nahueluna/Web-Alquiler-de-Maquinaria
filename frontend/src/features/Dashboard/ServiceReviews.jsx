import { useState, useEffect, useContext } from "react";
import {
  Box,
  Button,
  Sheet,
  Typography,
} from "@mui/joy";
import useAuth from "../utils/useAuth";
import UserContext from "../../context/UserContext";

const DEFAULT_ORDER = "recent";
const DEFAULT_RATING = null;

export default function ServiceReviews() {
  const { post } = useAuth();
  const { user } = useContext(UserContext);

  const [allReviews, setAllReviews] = useState([]);
  const [order, setOrder] = useState(DEFAULT_ORDER);
  const [ratingFilter, setRatingFilter] = useState(DEFAULT_RATING);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const loadReviews = async (orderParam = order, ratingParam = ratingFilter) => {
    if (!user || !user.access) {
      setError("No autorizado");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const body = {
        access: user.access,
      };
      if (orderParam) body.order = orderParam;
      if (ratingParam) body.rating = Number(ratingParam);

      const res = await post("reviews/service/get", body);

      setAllReviews(res.data.reviews || []);
    } catch (e) {
      setError("Error al cargar reseñas");
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadReviews(order, ratingFilter);
  }, [order, ratingFilter]);

  return (
    <Sheet
      sx={{
        bgcolor: "background.surface",
        borderRadius: 2,
        maxWidth: 800,
        width: "100%",
        p: 3,
        boxShadow: "lg",
      }}
    >
      <Typography level="h4" mb={2}>
        Valoraciones de servicio
      </Typography>

      {/* Controles de filtro y orden */}
      <Box sx={{ display: "flex", gap: 2, mb: 3, flexWrap: "wrap" }}>
        <select
          value={order}
          onChange={(e) => setOrder(e.target.value)}
          style={{ minWidth: 150, height: 32, fontSize: 14 }}
        >
          <option value="recent">Más recientes</option>
          <option value="more_rating">Más estrellas</option>
          <option value="less_rating">Menos estrellas</option>
        </select>

        <select
          value={ratingFilter || ""}
          onChange={(e) => setRatingFilter(e.target.value || null)}
          style={{ minWidth: 150, height: 32, fontSize: 14 }}
        >
          <option value="">Todas</option>
          <option value="5">5 estrellas</option>
          <option value="4">4 estrellas</option>
          <option value="3">3 estrellas</option>
          <option value="2">2 estrellas</option>
          <option value="1">1 estrella</option>
        </select>

        {(order !== DEFAULT_ORDER || ratingFilter !== DEFAULT_RATING) && (
          <Button
            variant="outlined"
            color="neutral"
            size="sm"
            onClick={() => {
              setOrder(DEFAULT_ORDER);
              setRatingFilter(DEFAULT_RATING);
            }}
            sx={{ alignSelf: "center", mt: 0 }}
          >
            Limpiar filtros
          </Button>
        )}
      </Box>

      {loading ? (
        <Typography>Cargando valoraciones...</Typography>
      ) : error ? (
        <Typography color="danger">{error}</Typography>
      ) : allReviews.length === 0 ? (
        <Typography>No hay valoraciones.</Typography>
      ) : (
        allReviews.map((review, index) => (
          <Box
            key={index}
            sx={{ mb: 3, borderBottom: "1px solid #eee", pb: 2 }}
          >
            <Typography fontWeight="bold">{review.user_name}</Typography>
            <Typography>{review.content}</Typography>
            <Typography fontSize="xs" color="text.secondary">
              ⭐ {review.rating} -{" "}
              {new Date(review.created_at).toLocaleDateString()}
            </Typography>

            <Typography fontSize="xs" color="text.secondary">
              Modelo: {review.model_brand} {review.model_model} - {review.model_name}
            </Typography>
            <Typography fontSize="xs" color="text.secondary">
            Empleado retiro: {review.retirement_employee_name?.trim() ? review.retirement_employee_name : "-"}
            </Typography>
            <Typography fontSize="xs" color="text.secondary">
            Empleado devolución: {review.return_employee_name?.trim() ? review.return_employee_name : "-"}
            </Typography>
          </Box>
        ))
      )}
    </Sheet>
  );
}
