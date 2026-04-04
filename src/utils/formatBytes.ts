export const FORMAT_BYTES_UNITS = ['B', 'KB', 'MB', 'GB', 'TB'] as const

export function formatBytes(bytes: number, decimals = 2): string {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  const unitIndex = Math.min(i, FORMAT_BYTES_UNITS.length - 1)
  
  const value = parseFloat((bytes / Math.pow(k, unitIndex)).toFixed(dm))
  const unit = FORMAT_BYTES_UNITS[unitIndex]
  
  return `${value} ${unit}`
}

export function formatDuration(ms: number): string {
  const seconds = Math.floor(ms / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  
  if (hours > 0) {
    return `${hours}h ${minutes % 60}m ${seconds % 60}s`
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`
  }
  return `${seconds}s`
}

export function formatDate(date: string): string {
  const d = new Date(date)
  const now = new Date()
  const diffMs = now.getTime() - d.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  
  if (diffDays === 0) return 'Today'
  if (diffDays === 1) return 'Yesterday'
  if (diffDays < 7) return `${diffDays} days ago`
  
  return d.toLocaleDateString()
}

export function getSafetyColor(score: number): string {
  if (score >= 0.8) return 'text-secondary'
  if (score >= 0.5) return 'text-accent'
  return 'text-red-500'
}

export function getSafetyLabel(score: number): string {
  if (score >= 0.8) return 'Safe'
  if (score >= 0.5) return 'Caution'
  return 'Risky'
}
