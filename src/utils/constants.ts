export const ROUTES = {
  dashboard: '/',
  scan: '/scan',
  cleanup: '/cleanup',
  ai: '/ai',
  history: '/history',
  settings: '/settings',
} as const

export const APP_NAME = 'SlothCleaner'
export const APP_VERSION = '0.1.0'

export const SAFE_CATEGORIES = [
  'Browser Cache',
  'Temporary Files',
  'Thumbnails',
  'Logs',
]

export const WARNING_CATEGORIES = [
  'Downloads',
  'Application Cache',
  'Development Cache',
]

export const RISKY_CATEGORIES = [
  'Large Files',
  'Duplicates',
  'Other',
]
