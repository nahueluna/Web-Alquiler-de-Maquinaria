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

function Dashboard() {
  const [selected, setSelected] = useState(0);

  const renderContent = () => {
    switch (selected) {
      case 0:
        return <Rentals />;
      case 1:
        return <Machines />;
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
