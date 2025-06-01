import { Sheet } from "@mui/joy";
import { Divider } from "@mui/material";
import axios from "axios";
import React, { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import Filters from "./Filters";
import Results from "./Results";
import SortBy from "./SortBy";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

const ExplorePage = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [results, setResults] = useState({
    items: [],
    categories: [],
    page: 0,
    page_size: 0,
    total_items: 0,
  });
  const [loading, setLoading] = useState(true);

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

  const query = searchParams.get("search") || "";

  useEffect(() => {
    async function fetchResults(query) {
      console.log("Fetching results for query:", query);
      try {
        const params = new URLSearchParams();
        params.append("search", query);
        selectedFilters.categories.forEach((cat) =>
          params.append("category", cat)
        );
        if (selectedFilters.minPrice !== null)
          params.append("min_price", selectedFilters.minPrice);
        if (selectedFilters.maxPrice !== null)
          params.append("max_price", selectedFilters.maxPrice);

        setSearchParams(params, { replace: true });

        const { data } = await axios.get(
          `${BACKEND_URL}/explore?${params.toString()}`
        );
        setResults(data);
        setLoading(false);
      } catch (error) {
        setResults({
          items: [],
          categories: [],
          page: 0,
          page_size: 0,
          total_items: 0,
        });
        setLoading(false);
        console.error(error);
      }
    }

    fetchResults(query); // Fetch results whenever the query changes to update them
  }, [query, selectedFilters]);

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
          query={query}
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
