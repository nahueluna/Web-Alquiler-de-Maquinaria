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
    const { data } = await axios.post("http://localhost:8000/login", loginInfo);

    saveLocalStorage("user", data);

    return data;
  }

  return (
    <UserContext.Provider
      value={{
        user,
        setUser,
        login,
      }}
    >
      {children}
    </UserContext.Provider>
  );
}

function saveLocalStorage(key, object) {
  window.localStorage.setItem(key, JSON.stringify(object));
}
