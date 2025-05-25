import { Typography } from "@mui/joy";
import Link from "@mui/joy/Link";
import Sheet from "@mui/joy/Sheet";
import { useState } from "react";
import { Link as RouterLink } from "react-router-dom";

function Home() {
  const [count, setCount] = useState(0);

  return (
    <Sheet>
      <Typography level="h1" sx={{ textAlign: "center", marginTop: 4 }}>
        En construcción!
      </Typography>
      <Typography level="body1" sx={{ textAlign: "center", marginTop: 2 }}>
        Para ver el catalogo de maquinas, por favor dirigite a la sección de{" "}
        <Link component={RouterLink} to={"/explore"}>
          Catalogo
        </Link>
      </Typography>
    </Sheet>
  );
}

export default Home;
