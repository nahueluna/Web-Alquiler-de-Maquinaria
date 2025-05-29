import { AspectRatio, Option, Select, Skeleton } from "@mui/joy";
import { useEffect, useState } from "react";

function Location({ dispatch, locations }) {
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState(locations[0]);

  function handleChange(event, newValue) {
    const loc = locations.find((x) => x.id === newValue);
    setLoading(true);
    setSelected(loc);
  }

  useEffect(() => {
    dispatch({ type: "setLocation", value: selected });
  }, [selected]);

  return (
    <>
      <Select
        onChange={handleChange}
        defaultValue={locations[0].id}
        sx={{ mb: 1 }}
      >
        {locations.map((l) => (
          <Option value={l.id}>{l.city}</Option>
        ))}
      </Select>
      <AspectRatio ratio={4 / 3}>
        <Skeleton loading={loading}>
          <iframe
            width="100%"
            height="450"
            style={{
              border: 0,
              borderRadius: 5,
            }}
            loading="lazy"
            onLoad={() => setLoading(false)}
            src={`https://www.google.com/maps/embed/v1/place?q=${
              selected.latitude
            },${selected.longitude}&key=${import.meta.env.VITE_MAPS}`}
          ></iframe>
        </Skeleton>
      </AspectRatio>
    </>
  );
}

export default Location;
