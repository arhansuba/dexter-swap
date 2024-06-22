// src/app.js

import React, { useState, useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom';
import './index.css'; // Optional: Import your global styles
import Header from './components/Header';
import Footer from './components/Footer';
import SwapComponent from './components/Swap/SwapComponent';
import LiquidityComponent from './components/Liquidity/AddLiquidity';
import {
  Abstraxion,
  useAbstraxionAccount,
  useAbstraxionSigningClient,
  useModal,
} from "@burnt-labs/abstraxion";
import { Button } from "@burnt-labs/ui";
import "@burnt-labs/ui/dist/index.css";

const App = () => {
  const [address, setAddress] = useState("");
  const { account } = useAbstraxionAccount();
  const { client, connect } = useAbstraxionSigningClient();
  const { isOpen, openModal, closeModal } = useModal();

  useEffect(() => {
    if (account) {
      setAddress(account.address);
    }
  }, [account]);

  const handleConnect = async () => {
    try {
      await connect();
      if (account) {
        setAddress(account.address);
      }
    } catch (error) {
      console.error("Connection failed", error);
    }
  };

  return (
    <div className="App">
      <Header userAddress={address} connected={!!address} />
      <div className="container">
        <nav>
          <Link to="/swap">Swap</Link>
          <Link to="/liquidity">Liquidity</Link>
        </nav>
        <Button onClick={handleConnect}>
          {address ? `Connected as: ${address}` : "Connect Wallet"}
        </Button>
        <Routes>
          <Route path="/" element={<SwapComponent />} />
          <Route path="/swap" element={<SwapComponent />} />
          <Route path="/liquidity" element={<LiquidityComponent />} />
          {/* Add more routes as needed */}
        </Routes>
      </div>
      <Footer />
    </div>
  );
};

// Use createRoot instead of ReactDOM.render
createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <Router>
      <App />
    </Router>
  </React.StrictMode>
);

export default App;
