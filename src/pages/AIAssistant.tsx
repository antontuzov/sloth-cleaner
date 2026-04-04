import { useState } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Send, Sparkles } from 'lucide-react'
import { aiChat } from '@/hooks/useAI'

const PREMADE_PROMPTS = [
  "What's taking up the most space?",
  'Suggest safe cleanup for 5GB',
  'Find duplicate files',
  'Clean development cache',
]

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
}

export const AIAssistant = () => {
  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  
  const handleSend = async (message: string) => {
    if (!message.trim() || isLoading) return
    
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: message,
      timestamp: new Date(),
    }
    
    setMessages(prev => [...prev, userMessage])
    setInput('')
    setIsLoading(true)
    
    try {
      const response = await aiChat(message)
      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: response.message,
        timestamp: new Date(),
      }
      setMessages(prev => [...prev, assistantMessage])
    } catch (error) {
      console.error('AI chat error:', error)
    } finally {
      setIsLoading(false)
    }
  }
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    handleSend(input)
  }
  
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-text">AI Assistant</h1>
        <p className="text-text-secondary mt-1">Ask questions about your system and get smart recommendations</p>
      </div>
      
      {/* Premade Prompts */}
      {!messages.length && (
        <Card>
          <div className="space-y-4">
            <div className="flex items-center gap-2 mb-4">
              <Sparkles className="w-5 h-5 text-primary" />
              <h3 className="font-semibold">Try asking...</h3>
            </div>
            <div className="grid grid-cols-2 gap-3">
              {PREMADE_PROMPTS.map((prompt) => (
                <Button
                  key={prompt}
                  variant="secondary"
                  onClick={() => handleSend(prompt)}
                  className="justify-start text-left h-auto py-3 px-4"
                >
                  {prompt}
                </Button>
              ))}
            </div>
          </div>
        </Card>
      )}
      
      {/* Chat Messages */}
      {messages.length > 0 && (
        <Card>
          <div className="space-y-4 max-h-96 overflow-y-auto">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
              >
                <div
                  className={`max-w-[80%] rounded-lg p-3 ${
                    msg.role === 'user'
                      ? 'bg-primary text-white'
                      : 'bg-background text-text'
                  }`}
                >
                  <p>{msg.content}</p>
                </div>
              </div>
            ))}
            {isLoading && (
              <div className="flex justify-start">
                <div className="bg-background rounded-lg p-3">
                  <div className="flex gap-1">
                    <div className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                    <div className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                    <div className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
                  </div>
                </div>
              </div>
            )}
          </div>
        </Card>
      )}
      
      {/* Input */}
      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Ask about your system..."
          className="flex-1 input"
          disabled={isLoading}
        />
        <Button type="submit" disabled={isLoading || !input.trim()}>
          <Send className="w-4 h-4" />
        </Button>
      </form>
      
      {/* Learning Indicator */}
      <Card>
        <div className="flex items-center gap-3">
          <div className="w-2 h-2 bg-secondary rounded-full animate-pulse" />
          <div>
            <p className="text-sm font-medium">Learning your preferences</p>
            <p className="text-xs text-text-secondary">I'm getting smarter with every cleanup</p>
          </div>
        </div>
      </Card>
    </div>
  )
}
