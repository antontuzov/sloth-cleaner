import { useState } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Progress } from '@/components/ui/Progress'
import { formatBytes, formatDuration } from '@/utils/formatBytes'
import { Play, Pause, Square } from 'lucide-react'

const CATEGORIES = [
  { name: 'Browser Cache', icon: '🌐', color: 'text-blue-500' },
  { name: 'System Cache', icon: '⚙️', color: 'text-gray-500' },
  { name: 'Application Cache', icon: '📦', color: 'text-purple-500' },
  { name: 'Logs', icon: '📝', color: 'text-yellow-500' },
  { name: 'Temporary Files', icon: '🗑️', color: 'text-red-500' },
  { name: 'Downloads', icon: '📥', color: 'text-green-500' },
  { name: 'Thumbnails', icon: '🖼️', color: 'text-pink-500' },
  { name: 'Development Cache', icon: '💻', color: 'text-indigo-500' },
]

type CategoryWithStats = typeof CATEGORIES[number] & { size: number; count: number }

export const Scan = () => {
  const [isScanning, setIsScanning] = useState(false)
  const [isPaused, setIsPaused] = useState(false)
  const [progress, setProgress] = useState(0)
  const [filesScanned, setFilesScanned] = useState(0)
  const [elapsed, setElapsed] = useState(0)
  const [categories, setCategories] = useState<CategoryWithStats[]>([])
  
  const handleStartScan = () => {
    setIsScanning(true)
    setIsPaused(false)
    
    // Simulate scanning progress
    let currentProgress = 0
    let currentFiles = 0
    const startTime = Date.now()
    
    const interval = setInterval(() => {
      if (!isPaused) {
        currentProgress += Math.random() * 5
        currentFiles += Math.floor(Math.random() * 1000)
        
        if (currentProgress >= 100) {
          currentProgress = 100
          clearInterval(interval)
          setIsScanning(false)
          
          // Set simulated categories
          setCategories(CATEGORIES.map(cat => ({
            ...cat,
            size: Math.random() * 2 * 1024 * 1024 * 1024,
            count: Math.floor(Math.random() * 10000),
          })))
        }
        
        setProgress(currentProgress)
        setFilesScanned(currentFiles)
        setElapsed(Date.now() - startTime)
      }
    }, 200)
  }
  
  const handlePause = () => {
    setIsPaused(!isPaused)
  }
  
  const handleStop = () => {
    setIsScanning(false)
    setIsPaused(false)
    setProgress(0)
    setFilesScanned(0)
    setElapsed(0)
    setCategories([])
  }
  
  const totalSize = categories.reduce((sum, cat) => sum + cat.size, 0)
  
  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-text">System Scan</h1>
          <p className="text-text-secondary mt-1">Scan your system for cleanable files</p>
        </div>
        <div className="flex gap-2">
          {!isScanning ? (
            <Button onClick={handleStartScan} leftIcon={<Play className="w-5 h-5" />}>
              Start Scan
            </Button>
          ) : (
            <>
              <Button variant="secondary" onClick={handlePause} leftIcon={isPaused ? <Play className="w-5 h-5" /> : <Pause className="w-5 h-5" />}>
                {isPaused ? 'Resume' : 'Pause'}
              </Button>
              <Button variant="danger" onClick={handleStop} leftIcon={<Square className="w-5 h-5" />}>
                Stop
              </Button>
            </>
          )}
        </div>
      </div>
      
      {/* Progress */}
      {isScanning && (
        <Card>
          <div className="space-y-4">
            <Progress value={progress} size="lg" showLabel />
            <div className="flex justify-between text-sm text-text-secondary">
              <span>{filesScanned.toLocaleString()} files scanned</span>
              <span>{formatDuration(elapsed)} elapsed</span>
            </div>
          </div>
        </Card>
      )}
      
      {/* Categories */}
      {categories.length > 0 && (
        <Card title="Scan Results" subtitle={`${formatBytes(totalSize)} can be freed`}>
          <div className="space-y-3">
            {categories.map((cat) => (
              <div key={cat.name} className="flex items-center justify-between p-3 rounded-lg bg-background">
                <div className="flex items-center gap-3">
                  <span className="text-2xl">{cat.icon}</span>
                  <div>
                    <p className="font-medium">{cat.name}</p>
                    <p className="text-sm text-text-secondary">{cat.count.toLocaleString()} files</p>
                  </div>
                </div>
                <span className="font-semibold text-primary">{formatBytes(cat.size)}</span>
              </div>
            ))}
          </div>
        </Card>
      )}
      
      {/* Empty state */}
      {!isScanning && categories.length === 0 && (
        <Card>
          <div className="text-center py-12">
            <div className="text-6xl mb-4">🔍</div>
            <h3 className="text-lg font-semibold mb-2">Ready to Scan</h3>
            <p className="text-text-secondary">Click "Start Scan" to analyze your system</p>
          </div>
        </Card>
      )}
    </div>
  )
}
