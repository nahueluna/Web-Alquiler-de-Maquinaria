import { useLocation } from "react-router-dom";
import useAuth from "../utils/useAuth";
import { useEffect } from "react";
import { Link, Sheet, Typography } from "@mui/joy";
import CircularProgress from "@mui/joy/CircularProgress";
import { useState } from "react";
import UserContext from "../../context/UserContext";
import { useContext } from "react";
import FailImage from "../../assets/PaymentFailed.png";
import SuccessImage from "../../assets/PaymentSuccessful.png";
import Stack from "@mui/joy/Stack";
import { Link as RouterLink } from "react-router-dom";

/* URL PRUEBA
  /payment?payment_id=123456789&status=approved&merchant_order_id=987654321&preference_id=abc123
*/

function PaymentPage() {
  const { post } = useAuth();
  const { search } = useLocation();
  const params = new URLSearchParams(search);
  const paymentId = params.get("payment_id");
  const status = params.get("status");
  const { user } = useContext(UserContext);

  const [loading, setLoading] = useState(true);
  const [result, setResult] = useState({
    isError: false,
    message: "",
  });

  // Para cuando tenga la rental id
  const rentalId = 21;

  useEffect(() => {
    const handlePayment = async () => {
      if (paymentId && status) {
        try {
          await post(
            `/payment/check?payment_id=${paymentId}&status=${status}`,
            {
              rental_id: rentalId,
            }
          );
          setResult({
            isError: false,
            message: "Se aprobo tu alquiler y se te envio un mail.",
          });
        } catch (error) {
          let errorMessage;
          switch (error.response?.status) {
            case 500:
              errorMessage =
                "Hubo un error al procesar la operacion. Lamentamos las molestias.";
              break;
            case 502:
              errorMessage =
                "Tu pago fue rechazado. El alquiler se marcó como fallido.";
              break;
            case 401:
              errorMessage =
                "No podemos verificar el pago porque tu sesion expiro. Por favor, inicia sesión nuevamente.";
              break;
            case 403:
              errorMessage = "No tenes permiso para realizar esta operación.";
              break;
            case 404:
              errorMessage =
                "No se encontró el alquiler asociado a este pago. Lamentamos las molestias";
              break;
            case 409:
              errorMessage =
                "Se recibio un estado de pago distinto al esperado.";
              break;
            default:
              errorMessage =
                "Hubo un error al procesar tu pago. Lamentamos las molestias.";
              break;
          }
          setResult({
            isError: true,
            message: errorMessage,
          });
        }
      } else {
        setResult({
          isError: true,
          message:
            "Los datos del pago enviados estan incompletos o son invalidos.",
        });
      }
      setLoading(false);
    };

    handlePayment();
  }, [user]);

  return (
    <Sheet
      sx={{
        width: "100%",
        minHeight: "100vh",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        minHeight: "100vh",
        backgroundColor: {
          sm: "#f4f4f4",
        },
      }}
    >
      <Sheet
        sx={{
          p: 2,
          borderRadius: "md",
          maxWidth: 800,
          height: "50%",
          minHeight: 300,
          maxHeight: 600,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 2,
          boxShadow: {
            xs: "none",
            sm: "xl",
          },
          px: {
            xs: 2,
            sm: 5,
          },
          py: 5,
          borderRadius: "md",
        }}
      >
        {loading ? (
          <CircularProgress color="danger" size="md" variant="plain" />
        ) : (
          <Stack spacing={1} alignItems="center" direction="row">
            <Stack spacing={3}>
              <Typography level="h1">
                {result.isError ? "Operacion fallida" : "Operacion exitosa!"}
              </Typography>
              <Typography level="h5">{result.message}</Typography>
              <Typography level="body-sm">
                Para volver al inicio, hace click{" "}
                <Link component={RouterLink} to={"/"}>
                  aca
                </Link>
                .
              </Typography>
            </Stack>
            {result.isError ? (
              <img
                src={FailImage}
                alt="Payment Failed"
                style={{ width: "100%", minWidth: 150, maxWidth: 300 }}
              />
            ) : (
              <img
                src={SuccessImage}
                alt="Payment Successful"
                style={{ width: "100%", maxWidth: 300 }}
              />
            )}
          </Stack>
        )}
      </Sheet>
    </Sheet>
  );
}

export default PaymentPage;
