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
} from "@mui/joy";
import { useParams, Link as RouterLink, useNavigate } from "react-router-dom";
import { useEffect, useState, useContext } from "react";
import axios from "axios";
import UserContext from "../../context/UserContext";
import ProductSkeleton from "./ProductSkeleton";
import MachineCard from "../Explore/MachineCard";
import RentalModal from "./Modal/RentalModal";

function Product() {
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [machine, setMachine] = useState(null);
  const [locations, setLocations] = useState(null);
  const [products, setProducts] = useState([]);
  const { user, refresh } = useContext(UserContext);
  const nav = useNavigate();

  const { id } = useParams();

  useEffect(() => {
    window.scrollTo({
      top: 0,
      behavior: "smooth",
    });

    async function fetchMachine() {
      try {
        const { data } = await axios.get(`http://localhost:8000/explore/${id}`);

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
        const { data } = await axios.get(`http://localhost:8000/explore`);

        setProducts(data.items.slice(0, 6));
      } catch (error) {
        console.error(error);
      }
    }

    fetchProducts();
  }, []);

  return (
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
                <Skeleton animation="wave">
                  {/* <img
                style={{
                  width: "100%",
                  maxWidth: 500,
                }}
                // src={Img}
                alt=""
              /> */}
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
                ${machine?.price} x dia
              </Typography>
              <Button
                sx={{ width: "100%" }}
                size="lg"
                color="danger"
                onClick={async () => {
                  // TODO: Cleanup
                  const {
                    data: { locations },
                  } = await axios
                    .post(
                      `http://localhost:8000/explore/${machine.id}/locations`,
                      { access: user.access }
                    )
                    .catch(async () => {
                      const { access } = await refresh();
                      const res = await axios.post(
                        `http://localhost:8000/explore/${machine.id}/locations`,
                        {
                          access,
                        }
                      );

                      return res;
                    });
                  setLocations(locations);
                  setOpen(true);
                }}
                disabled={!user}
              >
                Alquilar
              </Button>
              {!user ? (
                <Typography textAlign="center" level="body-sm" mt={1}>
                  <Link component={RouterLink} to={"/login"}>
                    Inicia sesion
                  </Link>{" "}
                  para empezar a alquilar
                </Typography>
              ) : null}
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
                  <td>Anio</td>
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

          {/* Otros productos */}
          <Sheet
            sx={{
              py: 2,
            }}
          >
            <Typography level="h3">Otros productos</Typography>
            <Sheet
              sx={{
                display: "flex",
                gap: 2,
              }}
            >
              {products.map((machine) => (
                <Grid key={machine.id}>
                  <MachineCard
                    imageUrl={machine.imageUrl}
                    model={machine.model}
                    category={machine.category}
                    price={machine.price}
                    onClick={() => nav(`/explore/${machine.id}`)}
                  />
                </Grid>
              ))}
            </Sheet>
          </Sheet>
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
  );
}

export default Product;
