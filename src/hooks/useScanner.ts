import { invoke } from '@/utils/tauri'
import type { ScanResults, ScanProgress, SystemInfo } from '@/types'

export async function startScan(): Promise<string> {
  const result = await invoke<{ id: string }>('start_scan')
  return result.id
}

export async function getScanProgress(scanId: string): Promise<ScanProgress> {
  return await invoke<ScanProgress>('get_scan_progress', { scanId })
}

export async function getScanResults(scanId: string): Promise<ScanResults> {
  return await invoke<ScanResults>('get_scan_results', { scanId })
}

export async function getSystemInfo(): Promise<SystemInfo> {
  return await invoke<SystemInfo>('get_system_info')
}
