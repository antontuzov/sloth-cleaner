import { useEffect, useCallback } from 'react'

interface ShortcutHandler {
  key: string
  meta?: boolean
  shift?: boolean
  alt?: boolean
  ctrl?: boolean
  callback: () => void
  description?: string
}

const shortcuts: ShortcutHandler[] = [
  { key: 's', meta: true, callback: () => window.location.hash = '/scan', description: 'Start scan' },
  { key: 'w', meta: true, callback: () => window.close(), description: 'Close window' },
  { key: '1', meta: true, callback: () => window.location.hash = '/', description: 'Go to Dashboard' },
  { key: '2', meta: true, callback: () => window.location.hash = '/scan', description: 'Go to Scan' },
  { key: '3', meta: true, callback: () => window.location.hash = '/cleanup', description: 'Go to Cleanup' },
  { key: '4', meta: true, callback: () => window.location.hash = '/ai', description: 'Go to AI Assistant' },
  { key: '5', meta: true, callback: () => window.location.hash = '/history', description: 'Go to History' },
  { key: ',', meta: true, callback: () => window.location.hash = '/settings', description: 'Go to Settings' },
]

export function useKeyboardShortcuts() {
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    for (const shortcut of shortcuts) {
      const metaMatch = shortcut.meta === (e.metaKey || e.ctrlKey)
      const shiftMatch = shortcut.shift === e.shiftKey
      const altMatch = shortcut.alt === e.altKey
      const keyMatch = e.key.toLowerCase() === shortcut.key.toLowerCase()

      if (metaMatch && shiftMatch && altMatch && keyMatch) {
        e.preventDefault()
        shortcut.callback()
        break
      }
    }
  }, [])

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [handleKeyDown])
}

export function getShortcutDescription(key: string, meta = true): string {
  const metaKey = navigator.platform.includes('Mac') ? '⌘' : 'Ctrl'
  return meta ? `${metaKey}+${key.toUpperCase()}` : key.toUpperCase()
}
