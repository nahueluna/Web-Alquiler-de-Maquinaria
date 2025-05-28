import axios from "axios";
import { createContext, useEffect, useState } from "react";

const UserContext = createContext();

export default UserContext;

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

  async function login(loginInfo) {
    const { data } = await axios.post(
      "http://localhost:8000/login",
      loginInfo,
      {
        withCredentials: true,
      }
    );

    console.log("User logged in", data);

    saveLocalStorage("user", data);

    return data;
  }

  async function refresh() {
    const { data } = await axios.post("http://localhost:8000/refresh", null, {
      withCredentials: true,
    });
    const user = JSON.parse(window.localStorage.getItem("user"));
    user.access = data.access;

    saveLocalStorage("user", user);
    console.log("User refreshed", user);
    return data;
  }

  // TODO: Blacklist token

  async function logout() {
    try {
      const user = JSON.parse(window.localStorage.getItem("user"));
      const access = user.access;
      console.log("user", user);
      console.log("user.access", user.access);

      if (access) {
        const response = await axios.post(
          "http://localhost:8000/logout",
          { access },
          {
            withCredentials: true,
          }
        );
        console.log("exitoso", response.status);
      }

      window.localStorage.removeItem("user");
      setUser(null);
    } catch (error) {
      console.error("Error al cerrar sesion:", error);
      if (error.response) {
        console.log(error.response.status);
      }
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
