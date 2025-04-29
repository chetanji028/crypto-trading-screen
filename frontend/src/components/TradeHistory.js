 
import React from 'react';

function TradeHistory({ trades }) {
  return (
    <div style={{ background: '#2a2e39', padding: '10px', maxHeight: '300px', overflowY: 'auto' }}>
      <h3>Trade History</h3>
      <table>
        <thead>
          <tr>
            <th>Price</th>
            <th>Quantity</th>
            <th>Time</th>
          </tr>
        </thead>
        <tbody>
          {trades.map((trade, index) => (
            <tr key={index}>
              <td>{trade.price.toFixed(2)}</td>
              <td>{trade.quantity.toFixed(4)}</td>
              <td>{new Date(trade.time).toLocaleTimeString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default TradeHistory;