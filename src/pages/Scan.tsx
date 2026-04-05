import { useState, useEffect, useCallback } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Progress } from '@/components/ui/Progress'
import { Tooltip } from '@/components/ui/Tooltip'
import { formatDuration } from '@/utils/formatBytes'
import { Play, Pause, Square, Info } from 'lucide-react'
import { startScan, getScanProgress } from '@/hooks/useScanner'
import { toastInfo, toastSuccess, toastError } from '@/components/common/Toast'

const CATEGORIES = [
  { name: 'Browser Cache', icon: '🌐', color: 'text-blue-500', desc: 'Cached web pages and assets from browsers' },
  { name: 'System Cache', icon: '⚙️', color: 'text-gray-500', desc: 'Operating system temporary files' },
  { name: 'Application Cache', icon: '📦', color: 'text-purple-500', desc: 'App-specific caches and temp data' },
  { name: 'Logs', icon: '📝', color: 'text-yellow-500', desc: 'System and application log files' },
  { name: 'Temporary Files', icon: '🗑️', color: 'text-red-500', desc: 'System temp files (.tmp, .bak, .swp)' },
  { name: 'Downloads', icon: '📥', color: 'text-green-500', desc: 'Files in your Downloads folder' },
  { name: 'Thumbnails', icon: '🖼️', color: 'text-pink-500', desc: 'Cached image thumbnails' },
  { name: 'Development Cache', icon: '💻', color: 'text-indigo-500', desc: 'Build artifacts and package caches' },
]

export const Scan = () => {
  const [scanId, setScanId] = useState<string | null>(null)
  const [isScanning, setIsScanning] = useState(false)
  const [isPaused, setIsPaused] = useState(false)
  const [progress, setProgress] = useState(0)
  const [filesScanned, setFilesScanned] = useState(0)
  const [elapsed, setElapsed] = useState(0)

  // Poll scan progress
  useEffect(() => {
    if (!scanId || !isScanning) return

    const interval = setInterval(async () => {
      if (isPaused) return

      try {
        const p = await getScanProgress(scanId)
        setProgress(Math.round(p.progressPercent))
        setFilesScanned(p.filesScanned)
        setElapsed(p.elapsedMs)

        if (p.isComplete) {
          setIsScanning(false)
          toastSuccess('Scan Complete', `Found ${p.filesScanned.toLocaleString()} files`)
        }
      } catch (err) {
        console.error('Failed to get progress:', err)
      }
    }, 1000)

    return () => clearInterval(interval)
  }, [scanId, isScanning, isPaused])

  const handleStartScan = useCallback(async () => {
    try {
      const id = await startScan()
      setScanId(id)
      setIsScanning(true)
      setIsPaused(false)
      setProgress(0)
      setFilesScanned(0)
      setElapsed(0)
      toastInfo('Scanning Started', 'Analyzing your system...')
    } catch (err) {
      toastError('Scan Failed', String(err))
    }
  }, [])

  const handlePause = useCallback(() => {
    setIsPaused((prev) => !prev)
  }, [])

  const handleStop = useCallback(() => {
    setIsScanning(false)
    setIsPaused(false)
    setScanId(null)
    setProgress(0)
    setFilesScanned(0)
    setElapsed(0)
  }, [])

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-text">System Scan</h1>
          <p className="text-text-secondary mt-1">Scan your system for cleanable files</p>
        </div>
        <div className="flex gap-2">
          {!isScanning ? (
            <Tooltip content="Start scanning your cache directories" side="left">
              <span>
                <Button onClick={handleStartScan} leftIcon={<Play className="w-5 h-5" />} aria-label="Start scan">
                  Start Scan
                </Button>
              </span>
            </Tooltip>
          ) : (
            <>
              <Tooltip content={isPaused ? 'Resume scanning' : 'Pause scanning'} side="left">
                <span>
                  <Button variant="secondary" onClick={handlePause} leftIcon={isPaused ? <Play className="w-5 h-5" /> : <Pause className="w-5 h-5" />} aria-label={isPaused ? 'Resume scan' : 'Pause scan'}>
                    {isPaused ? 'Resume' : 'Pause'}
                  </Button>
                </span>
              </Tooltip>
              <Tooltip content="Stop the current scan" side="left">
                <span>
                  <Button variant="danger" onClick={handleStop} leftIcon={<Square className="w-5 h-5" />} aria-label="Stop scan">
                    Stop
                  </Button>
                </span>
              </Tooltip>
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
      {filesScanned > 0 && (
        <Card title="Scan Results" subtitle={`${filesScanned.toLocaleString()} files found so far`}>
          <div className="space-y-3">
            {CATEGORIES.map((cat) => (
              <div key={cat.name} className="flex items-center justify-between p-3 rounded-lg bg-background">
                <div className="flex items-center gap-3">
                  <span className="text-2xl" role="img" aria-hidden="true">{cat.icon}</span>
                  <div>
                    <p className="font-medium">{cat.name}</p>
                    <p className="text-sm text-text-secondary">{cat.desc}</p>
                  </div>
                </div>
                <Tooltip content={cat.desc}>
                  <button className="p-1 rounded hover:bg-border transition-colors" aria-label={`Info about ${cat.name}`}>
                    <Info className="w-4 h-4 text-text-secondary" />
                  </button>
                </Tooltip>
                <span className="text-sm text-text-secondary">Scanning...</span>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* Empty state */}
      {!isScanning && filesScanned === 0 && (
        <Card>
          <div className="text-center py-12">
            <div className="text-6xl mb-4" role="img" aria-label="Magnifying glass">🔍</div>
            <h3 className="text-lg font-semibold mb-2">Ready to Scan</h3>
            <p className="text-text-secondary">Click "Start Scan" to analyze your system</p>
          </div>
        </Card>
      )}
    </div>
  )
}
