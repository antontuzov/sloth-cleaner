import { invoke } from '@/utils/tauri'
import type { CleanupResult, Snapshot } from '@/types'

export async function proposeCleanup(scanId: string): Promise<unknown> {
  return await invoke('propose_cleanup', { scanId })
}

export async function executeCleanup(proposalId: string, dryRun: boolean): Promise<CleanupResult> {
  return await invoke<CleanupResult>('execute_cleanup', { proposalId, dryRun })
}

export async function listSnapshots(): Promise<Snapshot[]> {
  return await invoke<Snapshot[]>('list_snapshots')
}

export async function restoreSnapshot(snapshotId: string): Promise<void> {
  return await invoke('restore_snapshot', { snapshotId })
}
