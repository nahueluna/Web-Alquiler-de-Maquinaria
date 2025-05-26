import { Route, Routes } from "react-router-dom";
import ChangePassword from "../features/ChangePassword/ChangePassword";
import Dashboard from "../features/Dashboard/DashboardPage";
import ExplorePage from "../features/Explore/ExplorePage";
import Home from "../features/Home/HomePage";
import Login from "../features/Login/LoginPage";
import NotFoundPage from "../features/NotFoundPage";
import Product from "../features/Product/Product";
import Profile from "../features/Profile/Profile";
import RecoverPassword from "../features/RecoverPassword/RecoverPassword";
import RegisterPage from "../features/Register/RegisterPage";
import Terms from "../features/Terms/TermsPage";
import TwoFactor from "../features/TwoFactorAuth/TwoFactor";
import MainLayout from "../layout/MainLayout";
import AddEmployee from "../features/AddEmployee/AddEmployee"

export default function AppRoutes() {
  return (
    <Routes>
      <Route element={<MainLayout />}>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route path="/explore" element={<ExplorePage />} />
        <Route path="/explore/:id" element={<Product />} />
        <Route path="/terms" element={<Terms />} />
        <Route path="/recover-password" element={<RecoverPassword />} />
        <Route path="/profile" element={<Profile />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/change-password" element={<ChangePassword />} />
        <Route path="/add-employee" element={<AddEmployee />} />
        <Route path="*" element={<NotFoundPage />} />
      </Route>
      <Route path="/two-factor" element={<TwoFactor />} />
    </Routes>
  );
}
