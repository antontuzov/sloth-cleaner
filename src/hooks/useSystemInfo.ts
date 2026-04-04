import { invoke } from '@/utils/tauri'
import type { AnalyticsData } from '@/types'

export async function getAnalytics(days = 30): Promise<AnalyticsData> {
  return await invoke<AnalyticsData>('get_analytics', { days })
}

export async function exportLogs(): Promise<string> {
  return await invoke('export_logs')
}

export async function resetLearningData(): Promise<void> {
  return await invoke('reset_learning_data')
}
