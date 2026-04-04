import { Link, useLocation } from 'react-router-dom'
import { cn } from '@/utils/helpers'
import { ROUTES } from '@/utils/constants'
import {
  LayoutDashboard,
  Search,
  Trash2,
  Bot,
  History,
  Settings,
} from 'lucide-react'

const navItems = [
  { icon: LayoutDashboard, label: 'Dashboard', path: ROUTES.dashboard },
  { icon: Search, label: 'Scan', path: ROUTES.scan },
  { icon: Trash2, label: 'Cleanup', path: ROUTES.cleanup },
  { icon: Bot, label: 'AI Assistant', path: ROUTES.ai },
  { icon: History, label: 'History', path: ROUTES.history },
  { icon: Settings, label: 'Settings', path: ROUTES.settings },
]

export const Sidebar = () => {
  const location = useLocation()
  
  return (
    <aside className="w-64 bg-surface border-r border-border p-4">
      <nav className="space-y-1" role="navigation" aria-label="Main navigation">
        {navItems.map(({ icon: Icon, label, path }) => {
          const isActive = location.pathname === path
          return (
            <Link
              key={path}
              to={path}
              className={cn(
                'flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors',
                isActive
                  ? 'bg-primary/10 text-primary'
                  : 'text-text-secondary hover:bg-background hover:text-text'
              )}
              aria-current={isActive ? 'page' : undefined}
            >
              <Icon className="w-5 h-5" />
              {label}
            </Link>
          )
        })}
      </nav>
    </aside>
  )
}
