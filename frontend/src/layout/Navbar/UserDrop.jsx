import {
  Menu,
  MenuButton,
  Dropdown,
  MenuItem,
  Link,
  ListDivider,
} from "@mui/joy";
import { Link as RouterLink } from "react-router-dom";
import KeyboardArrowDownRoundedIcon from "@mui/icons-material/KeyboardArrowDownRounded";
import LogoutRoundedIcon from "@mui/icons-material/LogoutRounded";
import AccountCircleRoundedIcon from "@mui/icons-material/AccountCircleRounded";
import AgricultureRoundedIcon from "@mui/icons-material/AgricultureRounded";

// TODO: Tomar el usuario para mostrar el nombre y lo que haga falta. Probablemente de un context

function UserDrop() {
  return (
    <Dropdown>
      <MenuButton
        endDecorator={<KeyboardArrowDownRoundedIcon />}
        variant="plain"
      >
        User
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
        <MenuItem>
          <AgricultureRoundedIcon />
          <Link
            underline="none"
            textColor={"text.primary"}
            component={RouterLink}
            to="/rentals"
          >
            Mis alquileres
          </Link>
        </MenuItem>
        <ListDivider />
        <MenuItem>
          <LogoutRoundedIcon color="danger" />
          <Link
            underline="none"
            textColor={"text.primary"}
            component={RouterLink}
            onClick={() => {}} // TODO:
          >
            Cerrar sesion
          </Link>
        </MenuItem>
      </Menu>
    </Dropdown>
  );
}

export default UserDrop;
