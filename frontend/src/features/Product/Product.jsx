import {
  AspectRatio,
  Button,
  Divider,
  Grid,
  Link,
  Sheet,
  Skeleton,
  Table,
  Typography,
  Box,
} from "@mui/joy";
import { useContext, useEffect, useState } from "react";
import { Link as RouterLink, useNavigate, useParams } from "react-router-dom";
import UserContext from "../../context/UserContext";
import MachineCard from "../Explore/MachineCard";
import RentalModal from "./Modal/RentalModal";
import ProductSkeleton from "./ProductSkeleton";
import useAuth from "../utils/useAuth";
import QASection from "./QASection";
import Rating from "../Explore/Rating"

function Product() {
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [loadingImg, setLoadingImg] = useState(true);
  const [loadingButton, setLoadingButton] = useState(false);
  const [machine, setMachine] = useState(null);
  const [locations, setLocations] = useState(null);
  const [products, setProducts] = useState([]);
  const { user } = useContext(UserContext);
  const nav = useNavigate();
  const { get, post } = useAuth();

  const { id } = useParams();
  const [reviewData, setReviewData] = useState(null);
  const [allReviews, setAllReviews] = useState([]);
  const [showReviews, setShowReviews] = useState(false);

  useEffect(() => {
    window.scrollTo({
      top: 0,
      behavior: "smooth",
    });

    async function fetchMachine() {
      try {
        const { data } = await get(`/explore/${id}`);

        console.log(data.machine);
        setMachine(data.machine);
      } catch (error) {
        console.error(error);
      } finally {
        setLoading(false);
      }
    }

    fetchMachine();
  }, [id]);

  useEffect(() => {
    async function fetchProducts() {
      try {
        const { data } = await get("/explore");

        setProducts(data.items.slice(0, 6));
      } catch (error) {
        console.error(error);
      }
    }

    fetchProducts();
  }, []);

  useEffect(() => {
    async function fetchSummary() {
      if (!machine?.id) return;
      try {
        const res = await post("reviews/machines/get", { model_id: machine.id, order: "recent" });
        const { average_rating, reviews } = res.data;
        setReviewData({
          rating: average_rating,
          totalReviews: reviews.length,
        });
      } catch (e) {
        console.error(e);
      }
    }
    fetchSummary();
  }, [machine]);

  const handleShowReviews = async () => {
    if (!machine?.id) return;
    try {
      const res = await post("reviews/machines/get", { model_id: machine.id });
      setAllReviews(res.data.reviews);
      setShowReviews(true);
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <>
    <Sheet
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        width: "100%",
        padding: 1,
      }}
    >
      {loading ? (
        <ProductSkeleton />
      ) : machine !== null ? (
        <>
          {/* Product info */}
          <Sheet
            sx={{
              display: "flex",
              flexDirection: {
                xs: "column",
                lg: "row",
              },
              alignItems: "center",
              justifyContent: "center",
              pt: 20,
            }}
          >
            {/* TODO: multiple images */}
            <Sheet>
              <AspectRatio ratio="4/3" sx={{ width: 500, mr: 2 }}>
                <Skeleton loading={loadingImg} animation="wave">
                  <img
                    style={{
                      width: "100%",
                      maxWidth: 500,
                    }}
                    src={machine?.main_image}
                    alt=""
                    onLoad={() => setLoadingImg(false)}
                  />
                </Skeleton>
              </AspectRatio>
            </Sheet>

            <Sheet>
              <Typography level="h2" maxWidth={500}>
                <Skeleton loading={loading}>
                  {machine?.name} {machine?.model}
                </Skeleton>
              </Typography>
              <Typography textColor={"neutral.500"} level="body-md" width={500}>
                <Skeleton loading={loading}>{machine?.description}</Skeleton>
              </Typography>
              <Typography my={5} level="h3">

              {reviewData && (
                <Box
                  sx={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 2, // espacio entre Rating y el botón
                    paddingBottom: '8px',
                    fontSize: '1.25rem',
                    width: '100%',
                    maxWidth: 500,
                  }}
                >
                  <Rating reviews={reviewData} />

                  <Button
                    variant="outlined"
                    size="small"
                    color="danger"
                    onClick={handleShowReviews}
                  >
                    Ver valoraciones
                  </Button>
                </Box>
              )}
                ${machine?.price} x dia
              </Typography>

      {/* Modal de reviews */}
      {showReviews && (
        <Sheet
          component="div"
          sx={{
            position: "fixed",
            inset: 0,
            bgcolor: "rgba(0,0,0,0.4)",
            backdropFilter: "blur(5px)",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            zIndex: 1300,
            p: 2,
          }}
          onClick={() => setShowReviews(false)} // Cerrar modal al click fuera
        >
          <Sheet
            onClick={e => e.stopPropagation()} // Evita cerrar modal al click dentro
            sx={{
              bgcolor: "background.surface",
              borderRadius: 2,
              maxWidth: 600,
              width: "100%",
              maxHeight: "80vh",
              overflowY: "auto",
              p: 3,
              boxShadow: "lg",
            }}
          >
            <Typography level="h4" mb={2}>
              Valoraciones
            </Typography>

            {allReviews.length === 0 ? (
              <Typography>No hay valoraciones.</Typography>
            ) : (
              allReviews.map((review, index) => (
                <Box key={index} sx={{ mb: 3, borderBottom: "1px solid #eee", pb: 2 }}>
                  <Typography fontWeight="bold">{review.user_name}</Typography>
                  <Typography>{review.content}</Typography>
                  <Typography fontSize="xs" color="text.secondary">
                    ⭐ {review.rating} - {new Date(review.created_at).toLocaleDateString()}
                  </Typography>
                </Box>
              ))
            )}

            <Button
              variant="outlined"
              color="danger"
              sx={{ mt: 2 }}
              onClick={() => setShowReviews(false)}
            >
              Cerrar
            </Button>
          </Sheet>
        </Sheet>
      )}
              
              {user ? (
                user.pub_user.role === 1 ? (
                  <Button
                    sx={{ width: "100%" }}
                    size="lg"
                    color="danger"
                    loading={loadingButton}
                  >
                    Registrar alquiler presencial
                  </Button>
                ) : (
                  <Button
                    sx={{ width: "100%" }}
                    size="lg"
                    color="danger"
                    onClick={async () => {
                      setLoadingButton(true);
                      const {
                        data: { locations },
                      } = await post(`/explore/${machine.id}/locations`);
                      setLoadingButton(false);
                      setLocations(locations);
                      setOpen(true);
                    }}
                    disabled={user.pub_user.role === 0}
                    loading={loadingButton}
                  >
                    Alquilar
                  </Button>
                )
              ) : (
                <>
                  <Button
                    sx={{ width: "100%" }}
                    size="lg"
                    color="danger"
                    disabled
                  >
                    Alquilar
                  </Button>
                  <Typography textAlign="center" level="body-sm" mt={1}>
                    <Link component={RouterLink} to={"/login"}>
                      Inicia sesión
                    </Link>{" "}
                    para empezar a alquilar
                  </Typography>
                </>
              )}
              {/* Modal */}
              <RentalModal
                open={open}
                setOpen={setOpen}
                machine={machine}
                locations={locations}
              />
            </Sheet>
          </Sheet>

          <Divider sx={{ mt: 20, mb: 5 }} />
              
          <Sheet
            sx={{
              alignSelf: "center",
            }}
          >
            <Typography level="h4">Caracteristicas generales</Typography>
            <Table
              sx={{
                maxWidth: "500px",
              }}
              stripe={"odd"}
              borderAxis="none"
            >
              <tbody>
                <tr>
                  <td>Marca</td>
                  <td>{machine?.brand}</td>
                </tr>
                <tr>
                  <td>Modelo</td>
                  <td>{machine?.model}</td>
                </tr>
                <tr>
                  <td>Año</td>
                  <td>{machine?.year}</td>
                </tr>
                <tr>
                  <td>Politica de cancelacion</td>
                  <td>{machine?.policy}</td>
                </tr>
              </tbody>
            </Table>
          </Sheet>

          <Divider sx={{ my: 5 }} />

          <QASection id={id} />
        </>
      ) : (
        <Sheet>
          <Typography level="h2">No se encontro la maquina</Typography>
          <Typography level="body-lg">
            <Link component={RouterLink} to={"/explore"} textAlign="center">
              Catalogo
            </Link>
          </Typography>
        </Sheet>
      )}
    </Sheet>

    </>
  );
}

export default Product;
