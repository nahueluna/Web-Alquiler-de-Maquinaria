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

// Por si quieren que sean diferentes los colores
/*function getRandomColor() {
  var letters = "0123456789ABCDEF";
  var color = "#";
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}*/

const HorizontalBarChart = ({ statsData, typeName }) => {
  console.log(statsData);

  const labels = statsData.map((item) => item.name);
  const values = statsData.map((item) => item.value);

  console.log("HorizontalBarChart statsData", statsData);
  console.log("HorizontalBarChart labels", labels);
  console.log("HorizontalBarChart values", values);

  // No encontre una opcion simple en Chart.js, pero basicamente establezco un alto segun
  // la cantidad de labels y que se adapte
  const minBarHeight = 50;
  const chartHeight = Math.max(labels.length * minBarHeight, 200);

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    indexAxis: "y",
    scales: {
      x: {
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
    <div style={{ minWidth: "600px", width: "60%", height: chartHeight }}>
      <Bar data={data} options={options} />
    </div>
  );
};

export default HorizontalBarChart;
