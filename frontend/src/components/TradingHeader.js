 
import React from 'react';

function TradingHeader({ pair, price }) {
  return (
    <div style={{ padding: '10px', background: '#2a2e39' }}>
      <h2>{pair}</h2>
      <p>Last Price: ${price.toFixed(2)}</p>
    </div>
  );
}

export default TradingHeader;