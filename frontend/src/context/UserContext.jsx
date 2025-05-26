import axios from "axios";
import { createContext, useEffect, useState } from "react";

const UserContext = createContext();

export default UserContext;

export function UserProvider({ children }) {
  const [user, setUser] = useState(null);

  useEffect(() => {
    const user = window.localStorage.getItem("user");

    if (user) setUser(JSON.parse(user));
  }, []);

  async function login(loginInfo) {
    const { data } = await axios.post(
      "http://localhost:8000/login",
      loginInfo,
      {
        withCredentials: true,
      }
    );

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
    return data;
  }

  function logout() {
    window.localStorage.removeItem("user");
    setUser(null);
    // TODO: Blacklist token
  }

  return (
    <UserContext.Provider
      value={{
        user,
        setUser,
        login,
        logout,
        refresh,
      }}
    >
      {children}
    </UserContext.Provider>
  );
}

function saveLocalStorage(key, object) {
  window.localStorage.setItem(key, JSON.stringify(object));
}
