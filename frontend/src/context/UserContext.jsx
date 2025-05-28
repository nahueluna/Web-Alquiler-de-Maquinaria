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

  async function login(loginInfo, code = 0) {
    const { data } = await axios.post(
      "http://localhost:8000/login",
      code !== 0 ? { ...loginInfo, code } : loginInfo,
      {
        withCredentials: true,
      }
    );

    if (!data?.message) {
      saveLocalStorage("user", data);
    }

    return data;
  }

  async function refresh() {
    const { data } = await axios.post("http://localhost:8000/refresh", null, {
      withCredentials: true,
    });
    const user = JSON.parse(window.localStorage.getItem("user"));
    user.access = data.access;

    saveLocalStorage("user", user);
    setUser(user);
    return data;
  }

  async function logout() {
    try {
      const { data } = await axios.post(
        "http://localhost:8000/logout",
        {
          access: user.access,
        },
        {
          withCredentials: true,
        }
      );

      console.log(data);
    } catch (error) {
      if (error.status === 401) {
        // Not authorized
        const { access } = await refresh(); // If the refresh token expires this will break
        console.log(access);

        await axios.post(
          "http://localhost:8000/logout",
          {
            access,
          },
          {
            withCredentials: true,
          }
        );
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
