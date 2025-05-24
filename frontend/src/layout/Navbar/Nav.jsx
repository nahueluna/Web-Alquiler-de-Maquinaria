import { Link as RouterLink, useLocation } from "react-router-dom";
import { Link, Button } from "@mui/joy";

function Nav() {
  const location = useLocation();
  const navItems = [{ label: "Catalogo", path: "/explore" }];

  return (
    <nav>
      <ul
        style={{
          display: "flex",
          gap: 1,
          listStyle: "none",
        }}
      >
        {navItems.map((x) => (
          <li>
            <Link
              textColor={"text.primary"}
              component={RouterLink}
              to={x.path}
              underline="none"
            >
              <Button
                color={x.path === location.pathname ? "danger" : "neutral"}
                variant={x.path === location.pathname ? "soft" : "outlined"}
              >
                {x.label}
              </Button>
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}

export default Nav;
