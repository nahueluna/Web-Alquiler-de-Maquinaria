import {
  AspectRatio,
  Box,
  Button,
  Divider,
  Sheet,
  Skeleton,
  Table,
  Typography,
} from "@mui/joy";

function ProductSkeleton() {
  return (
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
        <Sheet>
          <AspectRatio ratio="4/3" sx={{ width: 500, mr: 2 }}>
            <Skeleton animation="wave"></Skeleton>
          </AspectRatio>
        </Sheet>

        <Sheet>
          <Typography level="h2" maxWidth={500}>
            <Skeleton>
              Lorem ipsum is placeholder text commonly used in the graphic,
              print, and publishing industries.
            </Skeleton>
          </Typography>
          <Typography textColor={"neutral.500"} level="body-md" width={500}>
            <Skeleton>Lorem ipsum is placeholder text</Skeleton>
          </Typography>
          <Typography my={5} level="h3"></Typography>
          <Button disabled sx={{ width: "100%" }} size="lg" color="danger">
            Alquilar
          </Button>
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
              <td>
                <Typography level="body-sm">
                  <Skeleton>Lorem ipsum</Skeleton>
                </Typography>
              </td>
            </tr>
            <tr>
              <td>Modelo</td>
              <td>
                <Typography level="body-sm">
                  <Skeleton>Lorem ipsum</Skeleton>
                </Typography>
              </td>
            </tr>
            <tr>
              <td>Anio</td>
              <td>
                <Typography level="body-sm">
                  <Skeleton>Lorem ipsum</Skeleton>
                </Typography>
              </td>
            </tr>
            <tr>
              <td>Politica de cancelacion</td>
              <td>
                <Typography level="body-sm">
                  <Skeleton>Lorem ipsum</Skeleton>
                </Typography>
              </td>
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
        ></Sheet>
      </Sheet>
    </>
  );
}

export default ProductSkeleton;
