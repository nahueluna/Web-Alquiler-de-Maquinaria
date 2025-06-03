import { Route, Routes } from "react-router-dom";
import ChangePassword from "../features/ChangePassword/ChangePassword";
import AddEmployee from "../features/Dashboard/AddEmployee";
import Dashboard from "../features/Dashboard/DashboardPage";
import ExplorePage from "../features/Explore/ExplorePage";
import Home from "../features/Home/HomePage";
import Login from "../features/Login/LoginPage";
import MyRentalsPage from "../features/MyRentals/MyRentalsPage";
import NotFoundPage from "../features/NotFoundPage";
import Product from "../features/Product/Product";
import Profile from "../features/Profile/Profile";
import RecoverPassword from "../features/RecoverPassword/RecoverPassword";
import RegisterPage from "../features/Register/RegisterPage";
import Terms from "../features/Terms/TermsPage";
import TwoFactor from "../features/TwoFactorAuth/TwoFactor";
import MainLayout from "../layout/MainLayout";
import ProtectedRoute from "./ProtectedRoutes";
import PaymentPage from "../features/Payment/PaymentPage";

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
        <Route
          path="/profile"
          element={
            <ProtectedRoute>
              {" "}
              <Profile />{" "}
            </ProtectedRoute>
          }
        />
        <Route
          path="/dashboard"
          element={
            <ProtectedRoute>
              {" "}
              <Dashboard />{" "}
            </ProtectedRoute>
          }
        />
        <Route path="/changepsw/:code" element={<ChangePassword />} />
        <Route
          path="/add-employee"
          element={
            <ProtectedRoute>
              {" "}
              <AddEmployee />{" "}
            </ProtectedRoute>
          }
        />
        <Route path="*" element={<NotFoundPage />} />
        <Route
          path="/myrentals"
          element={
            <ProtectedRoute>
              <MyRentalsPage />
            </ProtectedRoute>
          }
        />
      </Route>
      <Route path="/payment" element={<PaymentPage />} />
      <Route path="/two-factor" element={<TwoFactor />} />
    </Routes>
  );
}
