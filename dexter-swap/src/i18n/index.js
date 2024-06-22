import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import enTranslation from './locales/en.json';
import esTranslation from './locales/es.json';

// Configure i18next
i18n
  .use(LanguageDetector) // Enable language detection
  .use(initReactI18next) // Initialize react-i18next
  .init({
    resources: {
      en: {
        translation: enTranslation, // English translations
      },
      es: {
        translation: esTranslation, // Spanish translations
      },
    },
    fallbackLng: 'en', // Fallback language if translation is missing
    debug: true, // Enable debug mode for development
    interpolation: {
      escapeValue: false, // React already escapes by default
    },
  });

// Render the application
ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);

// Measure performance with web vitals (optional)
reportWebVitals();
