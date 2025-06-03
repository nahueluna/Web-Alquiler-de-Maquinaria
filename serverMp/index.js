// SDK de Mercado Pago
import dotenv from "dotenv";
import express from "express";
import cors from "cors";
dotenv.config();

import { MercadoPagoConfig, Preference } from "mercadopago";
// Agrega credenciales
const client = new MercadoPagoConfig({
  accessToken: process.env.ACCESS_TOKEN,
});

const app = express();
const port = 3000;

app.use(
  cors({
    origin: process.env.NGROK,
  })
);
app.use(express.json());

app.post("/pago", (req, res) => {
  const preference = new Preference(client);
  const {
    body: {
      machine: { name, model, price },
      days,
    },
  } = req;

  preference
    .create({
      body: {
        items: [
          {
            title: `${name} ${model}`,
            quantity: 1,
            unit_price: price * days,
          },
        ],
        back_urls: {
          success: `${process.env.NGROK}/payment`,
          failure: `${process.env.NGROK}/payment`,
        },
      },
    })
    .then((p) => res.send(p))
    .catch((e) => res.status(e.status).send(e));
});

app.listen(port, () => {
  console.log(`Listening on port: ${port}`);
});
