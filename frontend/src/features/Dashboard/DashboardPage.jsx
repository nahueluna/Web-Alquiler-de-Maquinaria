import AgricultureIcon from "@mui/icons-material/Agriculture";
import GroupIcon from "@mui/icons-material/Group";
import WorkIcon from "@mui/icons-material/Work";
import InventoryIcon from "@mui/icons-material/Inventory";
import BarChartIcon from "@mui/icons-material/BarChart";
import {
  Divider,
  List,
  ListItem,
  ListItemButton,
  Sheet,
  Stack,
} from "@mui/joy";
import ListItemDecorator from "@mui/joy/ListItemDecorator";
import RateReviewIcon from "@mui/icons-material/RateReview";
import { useState } from "react";
import Employees from "./Employees";
import Machines from "./Machines";
import Rentals from "./Rentals";
import MachineCopies from "./MachineCopies";
import { useEffect } from "react";
import useAuth from "../utils/useAuth";
import UserContext from "../../context/UserContext";
import { useContext } from "react";
import SupportAgentIcon from "@mui/icons-material/SupportAgent";
import QAPanel from "./QAPanel";
import Statistics from "./Statistics";
import ServiceReviews from "./ServiceReviews";

function Dashboard() {
  const [selected, setSelected] = useState(-1);
  const [categories, setCategories] = useState([]);
  const { get } = useAuth();
  const { user } = useContext(UserContext);

  useEffect(() => {
    const fetchCategories = async () => {
      try {
        const response = await get("/explore?search=ZZZZZZZZZZZZZZZZZZ");
        setCategories(response.data.all_categories);
        console.log("Categories fetched:", response.data.all_categories);
      } catch (error) {
        console.error("Error fetching categories:", error);
      }
    };

    fetchCategories();
  }, []);

  const renderContent = () => {
    switch (selected) {
      case -1:
        return <Rentals />;
      case 0:
        return <Statistics />;
      case 1:
        return <Machines categories={categories} />;
      case 2:
        return <Employees />;
      case 3:
        return <MachineCopies />;
      case 4:
        return <QAPanel />;
      case 5:
          return <ServiceReviews />;
      default:
        return null;
    }
  };

  return (
    <>
      <Stack direction={"row"} sx={{ minHeight: "100%", width: "100%" }}>
        <Sheet sx={{ minWidth: "15%" }}>
          <List>
            {user.pub_user.role === 0 && (
              <>
                <ListItem>
                  <ListItemButton
                    selected={selected === 0}
                    onClick={() => setSelected(0)}
                  >
                    <ListItemDecorator>
                      <BarChartIcon />
                    </ListItemDecorator>
                    Ver estadisticas
                  </ListItemButton>
                </ListItem>
              </>
            )}
            {user.pub_user.role === 0 && (
              <>
                <ListItem>
                  <ListItemButton
                    selected={selected === 1}
                    onClick={() => setSelected(1)}
                  >
                    <ListItemDecorator>
                      <AgricultureIcon />
                    </ListItemDecorator>
                    Administrar maquinas
                  </ListItemButton>
                </ListItem>
                <ListItem>
                  <ListItemButton
                    selected={selected === 2}
                    onClick={() => setSelected(2)}
                  >
                    <ListItemDecorator>
                      <GroupIcon />
                    </ListItemDecorator>
                    Administrar empleados
                  </ListItemButton>
                </ListItem>
                <ListItem>
                  <ListItemButton
                    selected={selected === 5}
                    onClick={() => setSelected(5)}
                  >
                    <ListItemDecorator>
                      <RateReviewIcon />
                    </ListItemDecorator>
                    Ver valoraciones de servicio
                  </ListItemButton>
                </ListItem>
              </>
            )}

            {(user.pub_user.role === 0 || user.pub_user.role === 1) && (
              <>
                <ListItem>
                  <ListItemButton
                    selected={selected === -1}
                    onClick={() => setSelected(-1)}
                  >
                    <ListItemDecorator>
                      <WorkIcon />
                    </ListItemDecorator>
                    Ver alquileres
                  </ListItemButton>
                </ListItem>
                <ListItem>
                  <ListItemButton
                    selected={selected === 3}
                    onClick={() => setSelected(3)}
                  >
                    <ListItemDecorator>
                      <InventoryIcon />
                    </ListItemDecorator>
                    Administrar ejemplares
                  </ListItemButton>
                </ListItem>
                <ListItem>
                  <ListItemButton
                    selected={selected === 4}
                    onClick={() => setSelected(4)}
                  >
                    <ListItemDecorator>
                      <SupportAgentIcon />
                    </ListItemDecorator>
                    Responder preguntas
                  </ListItemButton>
                </ListItem>
              </>
            )}
          </List>
        </Sheet>
        <Divider orientation="vertical" />
        <Sheet sx={{ flex: 1, p: 2 }}>{renderContent()}</Sheet>
      </Stack>
    </>
  );
}

export default Dashboard;
