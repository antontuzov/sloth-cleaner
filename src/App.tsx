import { Suspense, lazy } from 'react'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { Header } from './components/layout/Header'
import { Sidebar } from './components/layout/Sidebar'
import { ErrorBoundary } from './components/common/ErrorBoundary'
import { ToastContainer } from './components/common/ToastContainer'
import { PageSkeleton } from './components/common/LoadingSkeleton'
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts'

// Lazy-loaded pages for code splitting
const Dashboard = lazy(() => import('./pages/Dashboard').then(m => ({ default: m.Dashboard })))
const Scan = lazy(() => import('./pages/Scan').then(m => ({ default: m.Scan })))
const Cleanup = lazy(() => import('./pages/Cleanup').then(m => ({ default: m.Cleanup })))
const AIAssistant = lazy(() => import('./pages/AIAssistant').then(m => ({ default: m.AIAssistant })))
const Settings = lazy(() => import('./pages/Settings').then(m => ({ default: m.Settings })))
const History = lazy(() => import('./pages/History').then(m => ({ default: m.History })))

function App() {
  // Register global keyboard shortcuts
  useKeyboardShortcuts()

  return (
    <BrowserRouter future={{ v7_startTransition: true, v7_relativeSplatPath: true }}>
      <div className="flex flex-col h-screen bg-background">
        <Header />
        <div className="flex flex-1 overflow-hidden">
          <Sidebar />
          <main className="flex-1 overflow-auto p-6" role="main">
            <ErrorBoundary>
              <Suspense fallback={<PageSkeleton />}>
                <Routes>
                  <Route path="/" element={<Dashboard />} />
                  <Route path="/scan" element={<Scan />} />
                  <Route path="/cleanup" element={<Cleanup />} />
                  <Route path="/ai" element={<AIAssistant />} />
                  <Route path="/history" element={<History />} />
                  <Route path="/settings" element={<Settings />} />
                </Routes>
              </Suspense>
            </ErrorBoundary>
          </main>
        </div>
        <ToastContainer />
      </div>
    </BrowserRouter>
  )
}

export default App
