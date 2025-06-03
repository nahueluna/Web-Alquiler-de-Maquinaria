import { useContext } from "react";
import axios from "axios";
import UserContext from "../../context/UserContext";
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

function useAuth() {
  const { user, refresh } = useContext(UserContext);

  let isRefreshing = false;
  let failedQueue = [];
  let latestAccess = user?.access; // Track the latest access token

  const processQueue = (error, access = null) => {
    failedQueue.forEach((prom) => {
      if (error) {
        prom.reject(error);
      } else {
        prom.resolve(access);
      }
    });
    failedQueue = [];
  };

  const axiosInstance = axios.create({
    baseURL: BACKEND_URL,
    withCredentials: true,
    headers: {
      "Content-Type": "application/json",
    },
    data: {},
  });

  axiosInstance.interceptors.request.use((req) => {
    // Always use the latest access token
    const accessToken = latestAccess || user?.access;
    if (req.method === "post" && accessToken) {
      if (typeof req.data === "string") {
        const data = JSON.parse(req.data);
        data.access = accessToken;
        req.data = JSON.stringify(data);
      } else {
        req.data = {
          ...req.data,
          access: accessToken,
        };
      }
    }
    return req;
  });

  axiosInstance.interceptors.response.use(
    (res) => res,
    async (err) => {
      const originalRequest = err.config;

      if (err?.response?.status === 401 && !originalRequest._retry) {
        if (isRefreshing) {
          return new Promise(function (resolve, reject) {
            failedQueue.push({
              resolve: (access) => {
                // Use the latest access token
                if (typeof originalRequest.data === "string") {
                  const json = JSON.parse(originalRequest.data);
                  json.access = access;
                  originalRequest.data = JSON.stringify(json);
                } else {
                  originalRequest.data = {
                    ...originalRequest.data,
                    access,
                  };
                }
                originalRequest._retry = true;
                resolve(axiosInstance(originalRequest));
              },
              reject: (error) => {
                reject(error);
              },
            });
          });
        }

        originalRequest._retry = true;
        isRefreshing = true;

        try {
          const { access } = await refresh();
          latestAccess = access; // Update the latest access token
          processQueue(null, access);

          // Use the latest access token for the original request
          if (typeof originalRequest.data === "string") {
            const json = JSON.parse(originalRequest.data);
            json.access = access;
            originalRequest.data = JSON.stringify(json);
          } else {
            originalRequest.data = {
              ...originalRequest.data,
              access,
            };
          }

          return axiosInstance(originalRequest);
        } catch (refreshError) {
          processQueue(refreshError, null);
          return Promise.reject(refreshError);
        } finally {
          isRefreshing = false;
        }
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
