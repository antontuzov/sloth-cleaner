import { useState } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Progress } from '@/components/ui/Progress'
import { formatBytes } from '@/utils/formatBytes'
import { getSafetyColor, getSafetyLabel } from '@/utils/formatBytes'
import { Check, AlertTriangle, X } from 'lucide-react'

const MOCK_CATEGORIES = [
  { name: 'Browser Cache', size: 1234567890, count: 5432, safety: 0.95, icon: '🌐' },
  { name: 'System Cache', size: 987654321, count: 2345, safety: 0.85, icon: '⚙️' },
  { name: 'Temporary Files', size: 456789012, count: 1234, safety: 0.98, icon: '🗑️' },
  { name: 'Logs', size: 234567890, count: 876, safety: 0.90, icon: '📝' },
  { name: 'Thumbnails', size: 123456789, count: 4567, safety: 0.92, icon: '🖼️' },
  { name: 'Downloads', size: 2345678901, count: 234, safety: 0.60, icon: '📥' },
  { name: 'Development Cache', size: 876543210, count: 1567, safety: 0.75, icon: '💻' },
]

export const Cleanup = () => {
  const [selectedCategories, setSelectedCategories] = useState<Set<string>>(new Set())
  const [isCleaning, setIsCleaning] = useState(false)
  const [cleaningProgress, setCleaningProgress] = useState(0)
  
  const toggleCategory = (name: string) => {
    const next = new Set(selectedCategories)
    if (next.has(name)) {
      next.delete(name)
    } else {
      next.add(name)
    }
    setSelectedCategories(next)
  }
  
  const selectAllSafe = () => {
    const safe = MOCK_CATEGORIES.filter(c => c.safety >= 0.8).map(c => c.name)
    setSelectedCategories(new Set(safe))
  }
  
  const handleClean = () => {
    setIsCleaning(true)
    
    let progress = 0
    const interval = setInterval(() => {
      progress += 10
      setCleaningProgress(progress)
      
      if (progress >= 100) {
        clearInterval(interval)
        setIsCleaning(false)
        setCleaningProgress(0)
        setSelectedCategories(new Set())
      }
    }, 300)
  }
  
  const selectedSize = MOCK_CATEGORIES
    .filter(c => selectedCategories.has(c.name))
    .reduce((sum, c) => sum + c.size, 0)
  
  const SafetyIcon = ({ safety }: { safety: number }) => {
    if (safety >= 0.8) return <Check className="w-4 h-4 text-secondary" />
    if (safety >= 0.5) return <AlertTriangle className="w-4 h-4 text-accent" />
    return <X className="w-4 h-4 text-red-500" />
  }
  
  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-text">Cleanup</h1>
          <p className="text-text-secondary mt-1">Select what you want to clean</p>
        </div>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={selectAllSafe} disabled={isCleaning}>
            Select All Safe
          </Button>
        </div>
      </div>
      
      {/* Categories */}
      <Card>
        <div className="space-y-2">
          {MOCK_CATEGORIES.map((cat) => (
            <div
              key={cat.name}
              className={`flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-colors ${
                selectedCategories.has(cat.name)
                  ? 'border-primary bg-primary/5'
                  : 'border-border hover:border-primary/50'
              }`}
              onClick={() => toggleCategory(cat.name)}
            >
              <div className="flex items-center gap-4">
                <span className="text-2xl">{cat.icon}</span>
                <div>
                  <p className="font-medium">{cat.name}</p>
                  <p className="text-sm text-text-secondary">{cat.count.toLocaleString()} files</p>
                </div>
              </div>
              <div className="flex items-center gap-4">
                <div className="flex items-center gap-1">
                  <SafetyIcon safety={cat.safety} />
                  <span className={`text-sm ${getSafetyColor(cat.safety)}`}>
                    {getSafetyLabel(cat.safety)}
                  </span>
                </div>
                <span className="font-semibold text-primary w-24 text-right">
                  {formatBytes(cat.size)}
                </span>
                <input
                  type="checkbox"
                  checked={selectedCategories.has(cat.name)}
                  onChange={() => {}}
                  className="w-4 h-4"
                />
              </div>
            </div>
          ))}
        </div>
      </Card>
      
      {/* Cleaning Progress */}
      {isCleaning && (
        <Card>
          <div className="space-y-3">
            <Progress value={cleaningProgress} size="lg" showLabel />
            <p className="text-sm text-text-secondary text-center">Cleaning selected files...</p>
          </div>
        </Card>
      )}
      
      {/* Footer */}
      <div className="sticky bottom-0 bg-surface border-t border-border p-4 flex items-center justify-between">
        <div>
          <p className="text-sm text-text-secondary">{selectedCategories.size} categories selected</p>
          <p className="text-lg font-bold text-primary">{formatBytes(selectedSize)} can be freed</p>
        </div>
        <Button
          onClick={handleClean}
          disabled={selectedCategories.size === 0 || isCleaning}
          size="lg"
        >
          {isCleaning ? 'Cleaning...' : 'Clean Selected'}
        </Button>
      </div>
    </div>
  )
}
