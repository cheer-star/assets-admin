import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import LoginPage from './pages/user/login/index.tsx'

import { BrowserRouter, Route, Routes } from 'react-router'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter >
      <Routes>
        <Route index element={<App />} />
        <Route path='/login' element={<LoginPage />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
