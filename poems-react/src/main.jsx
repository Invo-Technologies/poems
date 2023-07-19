import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.jsx';
import './index.css';
import { PoemsContextProvider } from './context/poemsContext.jsx';

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <PoemsContextProvider>
      <App />
    </PoemsContextProvider>
  </React.StrictMode>
);
