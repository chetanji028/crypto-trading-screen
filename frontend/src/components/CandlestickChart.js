 
import React from 'react';
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip } from 'recharts';

function CandlestickChart({ trades }) {
  const chartData = trades.map((trade, index) => ({
    time: new Date(trade.time).toLocaleTimeString(),
    price: trade.price,
  }));

  return (
    <div style={{ background: '#2a2e39', padding: '10px' }}>
      <h3>Candlestick Chart</h3>
      <AreaChart width={600} height={400} data={chartData}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey="time" />
        <YAxis domain={['auto', 'auto']} />
        <Tooltip />
        <Area type="monotone" dataKey="price" stroke="#00c087" fill="#00c087" />
      </AreaChart>
    </div>
  );
}

export default CandlestickChart;