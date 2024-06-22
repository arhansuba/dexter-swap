import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter } from 'react-router-dom';
import './index.css'; // Optional: Import your global styles
import App from './App'; // Assuming your main App component is located in App.js
import reportWebVitals from './reportWebVitals'; // Optional: Performance monitoring

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById('root')
);

// Optional: Report web vitals to analyze performance
reportWebVitals();
