import React from "react";
import useAuth from "../utils/useAuth";
import { Bar } from "react-chartjs-2";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

const BarChart = ({ statsData, typeName }) => {
  const labels = Object.keys(statsData);
  const values = Object.values(statsData);

  console.log("BarChart statsData", statsData);
  console.log("BarChart labels", labels);
  console.log("BarChart values", values);

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    indexAxis: "x",
    scales: {
      y: {
        beginAtZero: true,
      },
    },
    plugins: {
      legend: {
        display: true,
      },
    },
  };

  const data = {
    labels,
    datasets: [
      {
        label: typeName,
        data: values,
        backgroundColor: "rgba(212, 88, 72, 0.5)",
        borderColor: "rgb(228, 133, 133)",
        borderWidth: 1,
      },
    ],
  };

  return (
    <div style={{ minWidth: "600px", width: "60%", minHeight: "400px" }}>
      <Bar data={data} options={options} />
    </div>
  );
};

export default BarChart;
