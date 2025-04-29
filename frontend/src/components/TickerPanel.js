 
import React from 'react';

function TickerPanel() {
  // Mock data for other coins (bonus feature)
  const tickers = [
    { pair: 'ETH/USDT', price: 3500, change: 2.5 },
    { pair: 'BNB/USDT', price: 600, change: -1.2 },
  ];

  return (
    <div style={{ background: '#2a2e39', padding: '10px' }}>
      <h3>Tickers</h3>
      {tickers.map((ticker, index) => (
        <div key={index}>
          <p>{ticker.pair}: ${ticker.price} ({ticker.change}%)</p>
        </div>
      ))}
    </div>
  );
}

export default TickerPanel;