import { Route, Routes } from "react-router-dom";
import Home from "../features/Home/HomePage";
import Login from "../features/Login/LoginPage";
import NotFoundPage from "../features/NotFoundPage";
import RegisterPage from "../features/Register/RegisterPage";
import MainLayout from "../layout/MainLayout";
import TwoFactor from "../features/TwoFactorAuth/TwoFactor";

export default function AppRoutes() {
  return (
    <Routes>
      <Route element={<MainLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<RegisterPage />} />
      </Route>
      <Route path="/two-factor" element={<TwoFactor />} />

      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
