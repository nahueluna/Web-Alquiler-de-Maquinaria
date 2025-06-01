import { Sheet } from "@mui/joy";
import { Divider } from "@mui/material";
import React, { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import Filters from "./Filters";
import Results from "./Results";
import SortBy from "./SortBy";
import useAuth from "../utils/useAuth";

const ExplorePage = () => {
  const [searchParams] = useSearchParams();
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(true);
  const { get } = useAuth();

  const [selectedFilters, setSelectedFilters] = React.useState({
    categories: [],
    maxPrice: null,
    minPrice: null,
  });

  const categories = searchParams.getAll("category");
  const maxPrice = searchParams.get("max_price");
  const minPrice = searchParams.get("min_price");

  const currentFilters = {
    categories: categories,
    maxPrice: maxPrice ? Number(maxPrice) : null,
    minPrice: minPrice ? Number(minPrice) : null,
  };

  const query = searchParams.get("q")?.toLowerCase() || "";

  useEffect(() => {
    async function fetchResults(query) {
      try {
        const params = new URLSearchParams();
        selectedFilters.categories.forEach((cat) =>
          params.append("category", cat)
        );
        if (selectedFilters.minPrice !== null)
          params.append("min_price", selectedFilters.minPrice);
        if (selectedFilters.maxPrice !== null)
          params.append("max_price", selectedFilters.maxPrice);
        const { data } = await get(`/explore?${params.toString()}`);
        setResults(data);
        setLoading(false);
      } catch (error) {
        console.error(error);
      }
    }

    fetchResults(query); // Fetch results whenever the query changes to update them
  }, [query]);

  return (
    <Sheet
      sx={{
        minWidth: "80%",
        maxWidth: "995px",
        minHeight: "100%",
        backgroundColor: "white",
        display: "flex",
        flexDirection: "row",
        boxShadow: "0 1px 4px rgba(0, 0, 0, 0.1)",
      }}
    >
      <Sheet sx={{ minWidth: 250, maxWidth: 350, flex: "0 0 20%" }}>
        <Filters
          results={results}
          currentFilters={currentFilters}
          setSelectedFilters={setSelectedFilters}
          selectedFilters={selectedFilters}
        />
      </Sheet>
      <Divider orientation="vertical" />
      <Sheet sx={{ flex: 1, minWidth: 0 }}>
        <Sheet
          sx={{
            display: "flex",
            justifyContent: "flex-end",
            px: 5,
            py: 1,
          }}
        >
          <SortBy />
        </Sheet>
        <Results results={results.items} loading={loading} />
      </Sheet>
    </Sheet>
  );
};

export default ExplorePage;
