import { Route, Routes } from "react-router-dom";
import ExplorePage from "../features/Explore/ExplorePage";
import Home from "../features/Home/HomePage";
import Login from "../features/Login/LoginPage";
import NotFoundPage from "../features/NotFoundPage";
import Product from "../features/Product/Product";
import RegisterPage from "../features/Register/RegisterPage";
import Terms from "../features/Terms/TermsPage";
import TwoFactor from "../features/TwoFactorAuth/TwoFactor";
import MainLayout from "../layout/MainLayout";

export default function AppRoutes() {
  return (
    <Routes>
      <Route element={<MainLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route path="/explore" element={<ExplorePage />} />
        <Route path="/maquina/:id" element={<Product />} />
        <Route path="/terms" element={<Terms />} />
      </Route>
      <Route path="/two-factor" element={<TwoFactor />} />

      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
