import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import * as React from "react";
import { useSearchParams } from "react-router-dom";

export default function SortBy() {
  const [open, setOpen] = React.useState(false);
  const [searchParams, setSearchParams] = useSearchParams();

  const handleOpenChange = React.useCallback((event, isOpen) => {
    setOpen(isOpen);
  }, []);

  const handleSortByPrice = (e) => {
    const sortBy = e.currentTarget.getAttribute("name");
    const newParams = new URLSearchParams(searchParams);
    newParams.set("order_by", "price");
    newParams.set("order_dir", sortBy);
    setSearchParams(newParams);
    setOpen(false);
  };

  return (
    <Dropdown open={open} onOpenChange={handleOpenChange}>
      <MenuButton endDecorator={<ArrowDropDown />}>Ordenar por...</MenuButton>
      <Menu>
        {/*
        <MenuItem>Mejor valoracion</MenuItem>
        <MenuItem>Peor valoracion</MenuItem>
        PARA LA DEMO 2
        */}
        <MenuItem name="asc" onClick={handleSortByPrice}>
          Menor precio
        </MenuItem>
        <MenuItem name="desc" onClick={handleSortByPrice}>
          Mayor precio
        </MenuItem>
      </Menu>
    </Dropdown>
  );
}
