import React from 'react';
import MemoryChart from './MemoryChart'; 
import CyclesConsumedChart from './CyclesConsumedChart';
// import ModuleHashChart from './ModuleHashChart';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>Canister Memory Consumption</h1>
        <p>Graph showing the memory consumption over time.</p>
      </header>
      <MemoryChart />

      <header className="App-header">
        <h1>Canister Cycle Consumption</h1>
        <p>Graph showing the cycle consumption over time.</p>
      </header>
      <CyclesConsumedChart />
    </div>
  );
}

export default App;
