import React from "react";
import { Outlet } from "react-router-dom";
import Footer from "./Footer";
import Navbar from "./Navbar";
import { Box } from "@mui/joy";

const MainLayout = () => (
  <>
    <Box
      sx={{
        minHeight: "100vh",
        display: "grid",
        gridTemplateRows: "auto 1fr auto", // navbar | page content | footer
      }}
    >
      <Navbar />
      <Box
        sx={{
          display: "grid",
          placeItems: "center",
        }}
      >
        <Outlet />
      </Box>
      <Footer />
    </Box>
  </>
);

export default MainLayout;
