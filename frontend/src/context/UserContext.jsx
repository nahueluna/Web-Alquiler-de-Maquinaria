import axios from "axios";
import { createContext, useEffect, useState } from "react";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

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
      `${BACKEND_URL}/login`,
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
    const { data } = await axios.post(`${BACKEND_URL}/refresh`, null, {
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
        `${BACKEND_URL}/logout`,
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
          `${BACKEND_URL}/logout`,
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
