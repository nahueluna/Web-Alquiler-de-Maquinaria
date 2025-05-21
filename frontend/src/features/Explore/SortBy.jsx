import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import * as React from "react";

export default function SortBy() {
  const [open, setOpen] = React.useState(false);

  const handleOpenChange = React.useCallback((event, isOpen) => {
    setOpen(isOpen);
  }, []);

  return (
    <Dropdown open={open} onOpenChange={handleOpenChange}>
      <MenuButton endDecorator={<ArrowDropDown />}>Ordenar por...</MenuButton>
      <Menu>
        <MenuItem>Ascendente</MenuItem>
        <MenuItem>Descendente</MenuItem>
        <MenuItem>Menor precio</MenuItem>
        <MenuItem>Mayor precio</MenuItem>
      </Menu>
    </Dropdown>
  );
}
