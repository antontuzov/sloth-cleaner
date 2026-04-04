import { Link } from 'react-router-dom'
import { Snail } from 'lucide-react'
import { APP_NAME, APP_VERSION, ROUTES } from '@/utils/constants'

export const Header = () => {
  return (
    <header className="bg-surface border-b border-border px-6 py-4">
      <div className="flex items-center justify-between">
        <Link to={ROUTES.dashboard} className="flex items-center gap-3">
          <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
            <Snail className="w-6 h-6 text-white" />
          </div>
          <div>
            <h1 className="text-lg font-bold text-text">{APP_NAME}</h1>
            <p className="text-xs text-text-secondary">v{APP_VERSION}</p>
          </div>
        </Link>
        
        <div className="flex items-center gap-4">
          <button className="p-2 rounded-lg hover:bg-background transition-colors" aria-label="Notifications">
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
          </button>
        </div>
      </div>
    </header>
  )
}
