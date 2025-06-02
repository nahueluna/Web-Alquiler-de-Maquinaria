import { Sheet } from "@mui/joy";
import { Divider } from "@mui/material";
import React, { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import useAuth from "../utils/useAuth";
import Filters from "./Filters";
import Results from "./Results";
import SortBy from "./SortBy";

const ExplorePage = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [results, setResults] = useState({
    all_categories: [],
    items: [],
    categories: [],
    page: 0,
    page_size: 0,
    total_items: 0,
  });
  const [loading, setLoading] = useState(true);
  const { get } = useAuth();

  const [currentFilters, setCurrentFilters] = React.useState({
    categories: [],
    minPrice: null,
    maxPrice: null,
  });

  useEffect(() => {
    setCurrentFilters({
      categories: searchParams.getAll("category"),
      minPrice: searchParams.get("min_price")
        ? Number(searchParams.get("min_price"))
        : null,
      maxPrice: searchParams.get("max_price")
        ? Number(searchParams.get("max_price"))
        : null,
    });
  }, [searchParams]);

  const query = searchParams.get("search") || "";

  useEffect(() => {
    async function fetchResults() {
      try {
        const queryString = searchParams.toString();
        console.log("Fetching results with params:", queryString);
        const { data } = await get(`/explore?${queryString}`);
        setResults(data);
        setLoading(false);
      } catch (error) {
        setResults({
          all_categories: [],
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

    fetchResults();
  }, [searchParams]);

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
