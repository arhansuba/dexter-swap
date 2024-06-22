import React, { useState, useEffect } from 'react';
import { createRoot } from 'react-dom';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import './index.css'; // Optional: Import your global styles
import Header from './components/Header';
import Footer from './components/Footer';
import SwapComponent from './components/SwapComponent';
import LiquidityComponent from './components/LiquidityComponent';

// Main App component
function App() {
    // Example state variables
    const [userAddress, setUserAddress] = useState('');
    const [connected, setConnected] = useState(false);

    // Example useEffect to fetch user data on component mount
    useEffect(() => {
        const fetchUserData = async () => {
            // Replace with actual logic to fetch user data (e.g., address, balance)
            try {
                // Example: const address = await fetchUserAddress();
                // setUserAddress(address);
                // setConnected(true);
            } catch (error) {
                console.error('Error fetching user data:', error);
            }
        };

        fetchUserData();
    }, []);

    return (
        <div className="App">
            <Header userAddress={userAddress} connected={connected} />
            <div className="container">
                <Switch>
                    {/* Define routes for different components */}
                    <Route exact path="/">
                        <SwapComponent />
                    </Route>
                    <Route path="/liquidity">
                        <LiquidityComponent />
                    </Route>
                    {/* Add more routes as needed */}
                </Switch>
            </div>
            <Footer />
        </div>
    );
}

// Use createRoot instead of ReactDOM.render
createRoot(document.getElementById('root')).render(
    <React.StrictMode>
        <Router>
            <App />
        </Router>
    </React.StrictMode>
);

export default App;
