import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

// Component loading
import App from './App.jsx'
import Login from './components/Login'


import { BrowserRouter, Routes, Route } from "react-router";


createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<App />} />
        <Route path="login" element={<Login />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>
)
