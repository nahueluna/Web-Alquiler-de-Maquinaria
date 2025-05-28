import AccountCircleRoundedIcon from "@mui/icons-material/AccountCircleRounded";
import AgricultureRoundedIcon from "@mui/icons-material/AgricultureRounded";
import DashboardIcon from "@mui/icons-material/Dashboard";
import KeyboardArrowDownRoundedIcon from "@mui/icons-material/KeyboardArrowDownRounded";
import LogoutRoundedIcon from "@mui/icons-material/LogoutRounded";

import {
  Dropdown,
  Link,
  ListDivider,
  Menu,
  MenuButton,
  MenuItem,
} from "@mui/joy";
import { useContext } from "react";
import { Link as RouterLink } from "react-router-dom";
import UserContext from "../../context/UserContext";

function UserDrop() {
  const { user, logout } = useContext(UserContext);
  const {
    pub_user: { name },
  } = user;

  return (
    <Dropdown>
      <MenuButton
        endDecorator={<KeyboardArrowDownRoundedIcon />}
        variant="plain"
      >
        {name}
      </MenuButton>
      <Menu placement="bottom-end">
        <MenuItem>
          <AccountCircleRoundedIcon />
          <Link
            underline="none"
            textColor={"text.primary"}
            component={RouterLink}
            to="/profile"
          >
            Mis datos
          </Link>
        </MenuItem>
        {user.pub_user.role == 2 && (
          <MenuItem>
            <AgricultureRoundedIcon />
            <Link
              underline="none"
              textColor={"text.primary"}
              component={RouterLink}
              to="/myrentals"
            >
              Mis alquileres
            </Link>
          </MenuItem>
        )}
        {(user.pub_user.role == 0 || user.pub_user.role == 1) && (
          <MenuItem>
            <DashboardIcon />
            <Link
              underline="none"
              textColor={"text.primary"}
              component={RouterLink}
              to="/dashboard"
            >
              Dashboard
            </Link>
          </MenuItem>
        )}
        <ListDivider />
        <MenuItem>
          <LogoutRoundedIcon color="danger" />
          <Link
            underline="none"
            textColor={"text.primary"}
            component={RouterLink}
            onClick={logout}
          >
            Cerrar sesion
          </Link>
        </MenuItem>
      </Menu>
    </Dropdown>
  );
}

export default UserDrop;
