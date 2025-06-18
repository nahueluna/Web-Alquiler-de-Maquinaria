import axios from "axios";
import { createContext, useEffect, useState } from "react";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;
import { navigate } from "react-router-dom";

const UserContext = createContext();

export default UserContext;

const axiosInstance = axios.create({
  baseURL: BACKEND_URL,
  withCredentials: true,
  headers: {
    "Content-Type": "application/json",
  },
  data: {},
});

export function UserProvider({ children }) {
  const [user, setUser] = useState(null);
  const [loadingUser, setLoadingUser] = useState(true);

  useEffect(() => {
    const user = window.localStorage.getItem("user");

    if (user) {
      setUser(JSON.parse(user));
    }
    setLoadingUser(false);
  }, []);

  async function login(loginInfo, code = 0) {
    const { data } = await axiosInstance.post(
      "/login",
      code !== 0 ? { ...loginInfo, code } : loginInfo
    );

    if (!data?.message) {
      saveLocalStorage("user", data);
    }

    return data;
  }

  async function refresh() {
    try {
      const { data } = await axiosInstance.post("/refresh");
      const user = JSON.parse(window.localStorage.getItem("user"));
      user.access = data.access;

      saveLocalStorage("user", user);
      setUser(user);
      return data;
    } catch (err) {
      // Refresh expired
      if (err.code === 401) {
        setUser(null);
        window.localStorage.removeItem("user");
      }
      return err; // Should show a snackbar and redirect to / instead?
    }
  }

  async function logout() {
    try {
      const { data } = await axiosInstance.post("/logout", {
        access: user.access,
      });
      navigate("/");
      console.log(data);
    } catch (error) {
      if (error.status === 401) {
        // Not authorized
        const { access } = await refresh(); // If the refresh token expires this will break
        console.log(access);

        await axiosInstance.post("/logout", {
          access,
        });
      }
    } finally {
      window.localStorage.removeItem("user");
      setUser(null);
    }
  }

  return (
    <UserContext.Provider
      value={{
        user,
        setUser,
        login,
        logout,
        refresh,
        loadingUser,
      }}
    >
      {children}
    </UserContext.Provider>
  );
}

function saveLocalStorage(key, object) {
  window.localStorage.setItem(key, JSON.stringify(object));
}
