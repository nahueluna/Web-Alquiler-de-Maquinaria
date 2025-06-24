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
  const mesesOrdenados = [
    { en: "january", es: "Enero" },
    { en: "february", es: "Febrero" },
    { en: "march", es: "Marzo" },
    { en: "april", es: "Abril" },
    { en: "may", es: "Mayo" },
    { en: "june", es: "Junio" },
    { en: "july", es: "Julio" },
    { en: "august", es: "Agosto" },
    { en: "september", es: "Septiembre" },
    { en: "october", es: "Octubre" },
    { en: "november", es: "Noviembre" },
    { en: "december", es: "Diciembre" },
  ];

  // Gepeteado por darme datos cirujas
  const labels = mesesOrdenados.map((m) => m.es);
  const values = mesesOrdenados.map((m) => statsData[m.en] ?? 0);

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
