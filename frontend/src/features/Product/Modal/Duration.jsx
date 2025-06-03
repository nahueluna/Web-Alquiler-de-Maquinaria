import { Box, List, ListItem, ListItemButton, Sheet } from "@mui/joy";
import {
  areIntervalsOverlapping,
  formatISO,
  interval,
  isSameDay,
} from "date-fns";
import { addDays, intervalToDuration, eachDayOfInterval } from "date-fns";
import { useEffect, useState } from "react";
import { DateRange } from "@iroomit/react-date-range";
import ArrowRightRoundedIcon from "@mui/icons-material/ArrowRightRounded";

function Duration({ availability, setDisable, dispatch }) {
  const [selectedRange, setSelectedRange] = useState([
    {
      startDate: new Date(),
      endDate: new Date(),
      key: "selection",
    },
  ]);
  const [clickCount, setClickCount] = useState(1);
  const [selected, setSelected] = useState(null);
  const [disabledDates, setDisabledDates] = useState([]);

  useEffect(() => {
    setDisable(true);
    return () => setDisable(false);
  }, []);

  useEffect(() => {
    const [range] = selectedRange;

    // If start and end have been selected
    if (clickCount === 0) {
      // Make intervals
      const selectedInterval = interval(range.startDate, range.endDate);
      const maintentanceInterval = interval(
        addDays(selectedInterval.end, 1),
        addDays(range.endDate, 7)
      );
      const disabledIntervals = selected.periods.map((x) =>
        interval(x.start_date, x.end_date)
      );

      // Check if the selected interval overlaps any disabled ones and update it
      for (let d of disabledIntervals) {
        if (
          areIntervalsOverlapping(d, selectedInterval, {
            inclusive: true,
          }) ||
          areIntervalsOverlapping(d, maintentanceInterval, {
            inclusive: true,
          })
        ) {
          selectedInterval.start = addDays(d.end, 1);
          selectedInterval.end = addDays(selectedInterval.start, 7);

          maintentanceInterval.start = addDays(selectedInterval.end, 1);
          maintentanceInterval.end = addDays(selectedInterval.end, 7);
        }
      }

      setSelectedRange([
        {
          startDate: selectedInterval.start,
          endDate: selectedInterval.end,
          key: "selection",
        },
        {
          startDate: maintentanceInterval.start,
          endDate: maintentanceInterval.end,
          key: "maintenance",
        },
      ]);

      setDisable(false);
      dispatch({
        type: "setDates",
        value: [
          formatISO(selectedInterval.start, {
            representation: "date",
          }),
          formatISO(selectedInterval.end, {
            representation: "date",
          }),
        ],
      });
    }
  }, [clickCount]);

  return (
    <Sheet
      sx={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        gap: 2,
      }}
    >
      {selected !== null ? (
        <Box>
          <Box
            sx={{
              display: "flex",
              justifyContent: "space-around",
              mb: 1,
            }}
          >
            <Box
              sx={{
                display: "flex",
                alignItems: "center",
              }}
            >
              <Box
                sx={{
                  width: "15px",
                  aspectRatio: "1/1",
                  background: "#c41c1c",
                  mr: 1,
                }}
              ></Box>{" "}
              <span>Periodo seleccionado</span>
            </Box>

            <Box
              sx={{
                display: "flex",
                alignItems: "center",
              }}
            >
              <Box
                sx={{
                  width: "15px",
                  aspectRatio: "1/1",
                  background: "#ed8a26",
                  mr: 1,
                }}
              ></Box>{" "}
              <span>Mantenimiento planificado</span>
            </Box>
          </Box>
          <DateRange
            onChange={(item) => {
              let { selection } = item;
              const { startDate, endDate } = selection;
              // Set the hour to 21, this is because the date-fns intervals are set to that
              startDate.setHours(21);
              endDate.setHours(21);
              const inter = interval(startDate, endDate);
              const duration = intervalToDuration(inter);

              // Only the start date was selected
              if (clickCount === 0) {
                setClickCount(1);
              } else {
                // End date was selected
                // If the selected range is only one day, or less than 7 days, make it 7 days
                if (isSameDay(startDate, endDate)) {
                  selection.endDate = addDays(endDate, 7);
                } else if (duration.days < 7) {
                  selection.endDate = addDays(endDate, 7 - duration.days);
                }
                setClickCount(0);
              }

              setSelectedRange([selection]);
            }}
            showSelectionPreview={true}
            months={2}
            minDate={new Date()}
            ranges={selectedRange}
            direction="horizontal"
            showDateDisplay={false}
            disabledDates={disabledDates}
            rangeColors={["#c41c1c", "#ed8a26"]}
            fixedHeight
            dragSelectionEnabled={false}
          />
        </Box>
      ) : (
        <List variant="outlined">
          {availability.map((m, i) => (
            <ListItem>
              <ListItemButton
                onClick={() => {
                  dispatch({ type: "setUnitId", value: m.unit_id });
                  setDisabledDates(
                    m.periods
                      .map((x) =>
                        eachDayOfInterval(interval(x.start_date, x.end_date))
                      )
                      .flat()
                  );
                  setSelected(m);
                }}
                sx={{
                  display: "flex",
                  justifyContent: "space-between",
                }}
              >
                Ejemplar {i + 1}
                <ArrowRightRoundedIcon />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      )}
    </Sheet>
  );
}

export default Duration;
