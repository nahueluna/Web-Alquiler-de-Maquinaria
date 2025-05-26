import axios from "axios";
import { createContext, useState } from "react";

const UserContext = createContext();

export default UserContext;

export function UserProvider({ children }) {
  const [user, setUser] = useState(null);

  async function login(loginInfo) {
    const { data } = await axios.post("http://localhost:8000/login", loginInfo);

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
