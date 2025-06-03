import { Table, Typography } from "@mui/joy";

function Summary({ info }) {
  const { machine, selectedLocation, dates, days } = info;
  const { name, model, price } = machine;

  return (
    <>
      <Table
        sx={{
          width: "100%",
        }}
        stripe={"odd"}
        borderAxis="none"
      >
        <tbody>
          <tr>
            <td>Maquina</td>
            <td>
              {name} {model}
            </td>
          </tr>
          <tr>
            <td>Ubicacion</td> <td>{selectedLocation.city}</td>
          </tr>
          <tr>
            <td>Fecha de inicio</td> <td>{dates[0]}</td>
          </tr>
          <tr>
            <td>Fecha de fin</td> <td>{dates[1]}</td>
          </tr>
          <tr>
            <td>Duracion total</td> <td>{days} dias</td>
          </tr>
        </tbody>
      </Table>
      <Typography sx={{ textAlign: "right", mt: 2 }} level="h3">
        Precio total: ${price * days}
      </Typography>
    </>
  );
}

export default Summary;
