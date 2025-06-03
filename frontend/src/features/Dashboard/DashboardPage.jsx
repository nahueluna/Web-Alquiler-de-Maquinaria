import AgricultureIcon from "@mui/icons-material/Agriculture";
import GroupIcon from "@mui/icons-material/Group";
import WorkIcon from "@mui/icons-material/Work";
import {
  Divider,
  List,
  ListItem,
  ListItemButton,
  Sheet,
  Stack,
} from "@mui/joy";
import ListItemDecorator from "@mui/joy/ListItemDecorator";
import { useState } from "react";
import Employees from "./Employees";
import Machines from "./Machines";
import Rentals from "./Rentals";
import { useEffect } from "react";
import useAuth from "../utils/useAuth";

function Dashboard() {
  const [selected, setSelected] = useState(0);
  const [categories, setCategories] = useState([]);
  const { get } = useAuth();

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
      case 0:
        return <Rentals />;
      case 1:
        return <Machines categories={categories} />;
      case 2:
        return <Employees />;
      default:
        return null;
    }
  };

  return (
    <>
      <Stack direction={"row"} sx={{ minHeight: "100%", width: "100%" }}>
        <Sheet sx={{ minWidth: "15%" }}>
          <List>
            <ListItem>
              <ListItemButton
                selected={selected === 0}
                onClick={() => setSelected(0)}
              >
                <ListItemDecorator>
                  <WorkIcon />
                </ListItemDecorator>
                Ver alquileres
              </ListItemButton>
            </ListItem>
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
          </List>
        </Sheet>
        <Divider orientation="vertical" />
        <Sheet sx={{ flex: 1, p: 2 }}>{renderContent()}</Sheet>
      </Stack>
    </>
  );
}

export default Dashboard;
