import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { ConfigProvider, theme } from 'antd'
const { defaultAlgorithm, darkAlgorithm } = theme

const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ConfigProvider
      theme={{ algorithm: isDark ? darkAlgorithm : defaultAlgorithm }}
    >
      <App />
    </ConfigProvider>
  </StrictMode>,
)
