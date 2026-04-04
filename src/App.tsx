import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { Header } from './components/layout/Header'
import { Sidebar } from './components/layout/Sidebar'
import { Dashboard } from './pages/Dashboard'
import { Scan } from './pages/Scan'
import { Cleanup } from './pages/Cleanup'
import { AIAssistant } from './pages/AIAssistant'
import { Settings } from './pages/Settings'
import { History } from './pages/History'

function App() {
  return (
    <BrowserRouter>
      <div className="flex flex-col h-screen bg-background">
        <Header />
        <div className="flex flex-1 overflow-hidden">
          <Sidebar />
          <main className="flex-1 overflow-auto p-6">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/scan" element={<Scan />} />
              <Route path="/cleanup" element={<Cleanup />} />
              <Route path="/ai" element={<AIAssistant />} />
              <Route path="/history" element={<History />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </main>
        </div>
      </div>
    </BrowserRouter>
  )
}

export default App
