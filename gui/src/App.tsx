import React from 'react';
import './App.css';

function App() {
  return (
    <div className="app">
      <header>
        <h1>👑 King Hunter</h1>
        <p>Solana Launch Bot v1.0</p>
      </header>

      <main>
        <button onClick={() => alert("Launch Token clicked!")}>Launch New Token</button>
        <button onClick={() => alert("Create Wallets clicked!")}>Create Wallets</button>
        <button onClick={() => alert("Buy Tokens clicked!")}>Buy Tokens</button>
        <button onClick={() => alert("Check for Update clicked!")}>Check for Update</button>
      </main>

      <footer>
        <p>&copy; 2025 King Hunter. All rights reserved.</p>
      </footer>
    </div>
  );
}

export default App;
