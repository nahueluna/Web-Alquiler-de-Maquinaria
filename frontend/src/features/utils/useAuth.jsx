import { useContext } from "react";
import axios from "axios";
import UserContext from "../../context/UserContext";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

function useAuth() {
  const { user, refresh } = useContext(UserContext);
  const axiosInstance = axios.create({
    baseURL: BACKEND_URL,
    withCredentials: true,
    headers: {
      "Content-Type": "application/json",
    },
    data: {},
  });

  axiosInstance.interceptors.request.use((req) => {
    if (req.method === "post" && user?.access) {
      console.log(req.data);
      if (typeof req.data === "string") {
        const data = JSON.parse(req.data);
        data.access = user.access;
      } else {
        req.data = {
          ...req.data,
          access: user.access,
        };
      }
    }

    return req;
  });

  axiosInstance.interceptors.response.use(
    (res) => res,
    async (err) => {
      console.log(err);
      if (err?.response?.status === 401) {
        const { access } = await refresh();
        const { config } = err;
        const json = JSON.parse(config.data);
        json.access = access;
        config.data = JSON.stringify(json);

        return axiosInstance(config);
      }

      return Promise.reject(err);
    }
  );

  return {
    get: (...args) => axiosInstance.get(...args),
    post: (...args) => axiosInstance.post(...args),
  };
}

export default useAuth;
