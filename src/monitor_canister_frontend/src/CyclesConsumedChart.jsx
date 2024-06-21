import React, { useState, useEffect } from 'react';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale } from 'chart.js';
import { Line } from 'react-chartjs-2';
import 'chartjs-adapter-date-fns';
import { monitor_canister_backend } from 'declarations/monitor_canister_backend';
import { Principal } from "@dfinity/principal";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, TimeScale);

function CyclesConsumedChart() {
    const [dataPoints, setDataPoints] = useState([]);
    const [error, setError] = useState('');
  
    useEffect(() => {
      const canisterId = Principal.fromText("bkyz2-fmaaa-aaaaa-qaaaq-cai");
      const fetchData = async () => {
        try {
          const timestampsData = await monitor_canister_backend.get_all_timestamps(canisterId);
          console.log('Raw Fetched Data:', timestampsData);
  
          const referenceTime = BigInt(1718695527961107308); 
          const safeData = timestampsData.map(item => {
            const timeDiff = (item[0] - referenceTime) / BigInt(1e9);
            return [timeDiff, item[1]];
          });
  
          const data = safeData.map(item => ({
            x: new Date(Number(referenceTime / BigInt(1e6)) + Number(item[0]) * 1000),
            y: parseFloat(item[1].cycles.toString())
          }));
  
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
        label: 'Cycles Consumed',
        data: dataPoints,
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1
      }]
    };
  
    const options = {
      scales: {
        x: {
          type: 'time',
          time: {
            unit: 'second'
          },
          title: {
            display: true,
            text: 'Time since reference'
          }
        },
        y: {
          beginAtZero: true
        }
      },
      plugins: {
        legend: {
          display: true
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
  
  export default CyclesConsumedChart;