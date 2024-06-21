import React, { useState, useEffect } from 'react';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale } from 'chart.js';
import { Line } from 'react-chartjs-2';
import 'chartjs-adapter-date-fns';
import { monitor_canister_backend } from 'declarations/monitor_canister_backend';
import { Principal } from "@dfinity/principal";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale);

function ModuleHashChart() {
  const [dataPoints, setDataPoints] = useState([]);
  const [error, setError] = useState('');

  useEffect(() => {
    const canisterId = Principal.fromText("bkyz2-fmaaa-aaaaa-qaaaq-cai");

    const fetchData = async () => {
      try {
        const timestampsData = await monitor_canister_backend.get_all_timestamps(canisterId);
        const validData = timestampsData.filter(item => item[0] !== BigInt(0));

        // Processing the module hashes and filtering unique changes
        let lastHash = "";
        const data = validData.map(item => {
          const timestamp = new Date(Number(item[0] / BigInt(1e6))); // Convert nanoseconds to milliseconds
          const moduleHash = item[1].module_hash ? Buffer.from(item[1].module_hash).toString('hex') : null;

          if (moduleHash !== lastHash) {
            lastHash = moduleHash;
            return { x: timestamp, y: 1, moduleHash }; // Arbitrary Y value since we only care about X axis for timestamps
          }
          return null;
        }).filter(item => item !== null);

        setDataPoints(data);

      } catch (error) {
        console.error('Error fetching data:', error);
        setError(error.message);
      }
    };

    fetchData();
    const intervalId = setInterval(fetchData, 60000);

    return () => clearInterval(intervalId);

  }, []);

  const data = {
    datasets: [{
      label: 'Module Hash Changes',
      data: dataPoints,
      backgroundColor: 'rgb(255, 99, 132)',
      borderColor: 'rgba(255, 99, 132, 0.2)',
      pointRadius: 5,
      pointHoverRadius: 7,
    }]
  };

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
        }
      },
      y: {
        display: false // Hide Y axis
      }
    },
    plugins: {
      tooltip: {
        callbacks: {
          label: function(context) {
            return `Module Hash: ${context.raw.moduleHash}`;
          }
        }
      },
      legend: {
        display: false  // No need for a legend
      }
    },
    responsive: true,
    maintainAspectRatio: false
  };

  return (
    <div style={{ height: "400px" }}>
      {error ? <p>Error loading data: {error}</p> : <Line data={data} options={options} />}
    </div>
  );
}

export default ModuleHashChart;
