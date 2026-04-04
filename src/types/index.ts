export interface FileInfo {
  path: string
  name: string
  size: number
  category: string
  modified: string
  safetyScore: number
}

export interface CategoryResults {
  category: string
  size: number
  fileCount: number
  files: FileInfo[]
}

export interface ScanResults {
  scanId: string
  timestamp: string
  totalSize: number
  fileCount: number
  durationMs: number
  categories: CategoryResults[]
}

export interface ScanProgress {
  scanId: string
  filesScanned: number
  totalSizeScanned: number
  categoriesFound: number
  progressPercent: number
  isComplete: boolean
  elapsedMs: number
}

export interface CleanupResult {
  proposalId: string
  success: boolean
  freedSpace: number
  filesDeleted: number
  errors: string[]
  snapshotId?: string
}

export interface Snapshot {
  id: string
  timestamp: string
  filesCount: number
  totalSize: number
  restoreCount: number
}

export interface SystemInfo {
  osName: string
  osVersion: string
  hostname: string
  cpuBrand: string
  cpuCores: number
  totalMemoryGb: number
  availableMemoryGb: number
  diskTotalGb: number
  diskAvailableGb: number
}

export interface AnalyticsData {
  days: DayAnalytics[]
  totalSpaceFreedGb: number
  totalScans: number
  totalCleanups: number
  averageSavingsPerCleanupGb: number
}

export interface DayAnalytics {
  date: string
  spaceFreedGb: number
  scansCount: number
  cleanupsCount: number
}

export interface AIResponse {
  message: string
  actions: AIAction[]
  confidence: number
}

export interface AIAction {
  label: string
  actionType: string
  payload: Record<string, unknown>
}

export interface Recommendation {
  id: string
  title: string
  description: string
  category: string
  potentialSavingsGb: number
  safetyScore: number
  actionLabel: string
}
