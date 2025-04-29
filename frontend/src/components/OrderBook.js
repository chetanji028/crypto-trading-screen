 
import React from 'react';

function OrderBook({ bids, asks }) {
  return (
    <div style={{ background: '#2a2e39', padding: '10px' }}>
      <h3>Order Book</h3>
      <div style={{ display: 'flex', gap: '20px' }}>
        <div>
          <h4>Bids</h4>
          <table>
            <thead>
              <tr>
                <th>Price</th>
                <th>Quantity</th>
              </tr>
            </thead>
            <tbody>
              {bids.map((bid, index) => (
                <tr key={index}>
                  <td>{bid[0].toFixed(2)}</td>
                  <td>{bid[1].toFixed(4)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div>
          <h4>Asks</h4>
          <table>
            <thead>
              <tr>
                <th>Price</th>
                <th>Quantity</th>
              </tr>
            </thead>
            <tbody>
              {asks.map((ask, index) => (
                <tr key={index}>
                  <td>{ask[0].toFixed(2)}</td>
                  <td>{ask[1].toFixed(4)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

export default OrderBook;