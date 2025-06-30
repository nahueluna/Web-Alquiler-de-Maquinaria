import React from "react";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { Line } from "react-chartjs-2";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const LineChart = ({ statsData, typeName }) => {
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
        min: 0,
        suggestedMax: Math.max(10, ...values),
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
      <Line data={data} options={options} />
    </div>
  );
};

export default LineChart;
