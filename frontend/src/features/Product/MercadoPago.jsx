import { initMercadoPago, Wallet } from "@mercadopago/sdk-react";
import { useEffect } from "react";

const MercadoPago = () => {
  useEffect(() => {
    initMercadoPago("YOUR_PUBLIC_KEY", { locale: "es-AR" });
  }, []);

  return (
    <div>
      <Wallet initialization={{ preferenceId: "<PREFERENCE_ID>" }} />
    </div>
  );
};

export default MercadoPago;
