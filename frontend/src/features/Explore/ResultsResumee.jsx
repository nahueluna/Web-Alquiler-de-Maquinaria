import { Divider, Typography } from "@mui/joy";
import Box from "@mui/joy/Box";
import Chip from "@mui/joy/Chip";
import { useSearchParams } from "react-router-dom";

const ResultsResumee = ({ totalItems, currentFilters }) => {
  const [searchParams] = useSearchParams();
  const query = searchParams.get("q");

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
              <Typography level="body-xs">
                {totalItems === 0 ? "0" : totalItems} resultados
              </Typography>
            </>
          )}
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

          <Divider sx={{ mt: 2 }} />
        </>
      )}
    </Box>
  );
};

export default ResultsResumee;
