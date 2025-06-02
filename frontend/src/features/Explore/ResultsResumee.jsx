import { Divider, Typography } from "@mui/joy";
import Box from "@mui/joy/Box";
import Chip from "@mui/joy/Chip";
import Link from "@mui/joy/Link";
import { useSearchParams } from "react-router-dom";

const ResultsResumee = ({ totalItems, currentFilters, query }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  return (
    <Box>
      {(query ||
        currentFilters.categories.length > 0 ||
        currentFilters.maxPrice ||
        currentFilters.minPrice) && (
        <>
          {query && (
            <>
              <Typography level="title-lg">"{query}"</Typography>
            </>
          )}
          <Typography level="body-xs">
            {!totalItems || totalItems === 0 ? "0" : totalItems}{" "}
            {!totalItems || totalItems != 1 ? "resultados" : "resultado"}
          </Typography>

          {currentFilters.categories.length > 0 && (
            <Box>
              {currentFilters.categories.map((category) => (
                <Chip key={category} variant="soft" color="neutral">
                  {category}
                </Chip>
              ))}
            </Box>
          )}
          {currentFilters.maxPrice && (
            <Chip>Precio máximo: ${currentFilters.maxPrice}</Chip>
          )}
          {currentFilters.minPrice && (
            <Chip>Precio mínimo: ${currentFilters.minPrice}</Chip>
          )}
          <Link
            onClick={() => {
              setSearchParams((prev) => {
                const newParams = new URLSearchParams(prev);
                newParams.delete("search");
                newParams.delete("category");
                newParams.delete("min_price");
                newParams.delete("max_price");
                return newParams;
              });
            }}
            sx={{ mt: 1 }}
          >
            Limpiar filtros
          </Link>
          <Divider sx={{ mt: 2 }} />
        </>
      )}
    </Box>
  );
};

export default ResultsResumee;
