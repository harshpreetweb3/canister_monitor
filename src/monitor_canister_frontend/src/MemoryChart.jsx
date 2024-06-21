import React, { useState, useEffect } from 'react';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale } from 'chart.js';
import { Line } from 'react-chartjs-2';
import 'chartjs-adapter-date-fns';
import { monitor_canister_backend } from 'declarations/monitor_canister_backend';
import { Principal } from "@dfinity/principal";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale);

function MemoryChart() {
  const [dataPoints, setDataPoints] = useState([]);

  useEffect(() => {
    const canisterId = Principal.fromText("bkyz2-fmaaa-aaaaa-qaaaq-cai");

    const fetchData = async () => {
      try {
        const timestampsData = await monitor_canister_backend.get_all_timestamps(canisterId);
        const validData = timestampsData.filter(item => item[0] !== BigInt(0));

        console.log("valid data", validData)

        const data = validData.map(item => ({
          x: new Date(Number(item[0] / BigInt(1e6))), 
          y: parseFloat(item[1].memory_consumed.toString()) 
        }));

        setDataPoints(data);

      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    fetchData();

    console.log("here are the dataPoints", dataPoints);

    const intervalId = setInterval(fetchData, 60000);

    return () => {
      clearInterval(intervalId);
    };

  }, []);

  const data = {
    labels: dataPoints.map(dp => dp.x.toLocaleString()),

    datasets: [{
      label: 'Memory Consumed',
      data: dataPoints,
      fill: false,
      backgroundColor: 'rgb(75, 192, 192)',
      borderColor: 'rgba(75, 192, 192, 0.2)',
    }]

  };

  console.log("data object containing label and datasets", data)

  const options = {

    scales: {
      x: {
        type: 'time',

        time: {
          unit: 'minute',
          tooltipFormat: 'MMM d, YYYY h:mm:ss a'
        },

        title: {
          display: true,
          text: 'Timestamp'
        },
        
        ticks: {
          autoSkip: true,
          maxTicksLimit: 20
        },
        min: dataPoints[0] ? dataPoints[0].x : undefined, // Set to the first data point's timestamp
        max: dataPoints[dataPoints.length - 1] ? dataPoints[dataPoints.length - 1].x : undefined // Set to the last data point's timestamp
      },
      
      y: {
        beginAtZero: true,
        title: {
          display: true,
          text: 'Memory Consumed (Units)'
        },
      },
    },
    plugins: {
      legend: {
        display: true,
        position: 'top',
      },
    },
    responsive: true,
    maintainAspectRatio: false
  };

  return <div style={{ height: "400px" }}>
    <Line data={data} options={options} />
  </div>;
}

export default MemoryChart;


//10851068