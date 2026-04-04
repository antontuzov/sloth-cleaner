import { invoke } from '@/utils/tauri'
import type { AIResponse, Recommendation } from '@/types'

export async function aiChat(message: string, context: Record<string, unknown> = {}): Promise<AIResponse> {
  return await invoke<AIResponse>('ai_chat', { message, context })
}

export async function getAIRecommendations(): Promise<Recommendation[]> {
  return await invoke<Recommendation[]>('get_ai_recommendations')
}

export async function updateUserFeedback(filePath: string, decision: string): Promise<void> {
  return await invoke('update_user_feedback', { filePath, decision })
}
