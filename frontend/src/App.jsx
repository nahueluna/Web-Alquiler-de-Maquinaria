import CssBaseline from "@mui/joy/CssBaseline";
import { CssVarsProvider } from "@mui/joy/styles";
import { BrowserRouter } from "react-router-dom";
import AppRoutes from "./routes/AppRoutes";
import { UserProvider } from "./context/UserContext";
import "@iroomit/react-date-range/dist/styles.css"; // main css file
import "@iroomit/react-date-range/dist/theme/default.css"; // theme css file

const App = () => {
  return (
    <CssVarsProvider>
      <CssBaseline />
      <UserProvider>
        <BrowserRouter>
          <AppRoutes />
        </BrowserRouter>
      </UserProvider>
    </CssVarsProvider>
  );
};

export default App;
