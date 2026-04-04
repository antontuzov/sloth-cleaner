import { useState } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Toggle } from '@/components/ui/Toggle'
import { useSettingsStore, type Theme } from '@/stores/settingsStore'
import { resetLearningData, exportLogs } from '@/hooks/useSystemInfo'
import { Plus, X } from 'lucide-react'

type TabType = 'general' | 'safety' | 'ai' | 'advanced'

export const Settings = () => {
  const [activeTab, setActiveTab] = useState<TabType>('general')
  const [newPath, setNewPath] = useState('')
  
  const {
    theme,
    setTheme,
    language,
    setLanguage,
    autoStart,
    setAutoStart,
    enableRollback,
    setEnableRollback,
    dryRunAlways,
    setDryRunAlways,
    minFileAgeDays,
    setMinFileAgeDays,
    enableLearning,
    setEnableLearning,
    aggressiveness,
    setAggressiveness,
    protectedPaths,
    addProtectedPath,
    removeProtectedPath,
  } = useSettingsStore()
  
  const tabs: { id: TabType; label: string }[] = [
    { id: 'general', label: 'General' },
    { id: 'safety', label: 'Safety' },
    { id: 'ai', label: 'AI' },
    { id: 'advanced', label: 'Advanced' },
  ]
  
  const handleAddPath = () => {
    if (newPath.trim()) {
      addProtectedPath(newPath.trim())
      setNewPath('')
    }
  }
  
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-text">Settings</h1>
        <p className="text-text-secondary mt-1">Customize SlothCleaner to your preferences</p>
      </div>
      
      {/* Tabs */}
      <Card>
        <div className="flex gap-2 mb-6 border-b border-border">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-primary text-primary'
                  : 'border-transparent text-text-secondary hover:text-text'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
        
        {/* General Tab */}
        {activeTab === 'general' && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium mb-2">Theme</label>
              <div className="flex gap-2">
                {(['light', 'dark', 'system'] as Theme[]).map((t) => (
                  <Button
                    key={t}
                    variant={theme === t ? 'primary' : 'secondary'}
                    onClick={() => setTheme(t)}
                  >
                    {t.charAt(0).toUpperCase() + t.slice(1)}
                  </Button>
                ))}
              </div>
            </div>
            
            <div>
              <label className="block text-sm font-medium mb-2">Language</label>
              <select
                value={language}
                onChange={(e) => setLanguage(e.target.value)}
                className="input max-w-xs"
              >
                <option value="en">English</option>
                <option value="es">Español</option>
                <option value="fr">Français</option>
                <option value="de">Deutsch</option>
                <option value="zh">中文</option>
              </select>
            </div>
            
            <Toggle
              label="Auto-start on boot"
              description="Launch SlothCleaner when your computer starts"
              checked={autoStart}
              onCheckedChange={setAutoStart}
            />
          </div>
        )}
        
        {/* Safety Tab */}
        {activeTab === 'safety' && (
          <div className="space-y-6">
            <Toggle
              label="Enable rollback"
              description="Create snapshots before cleanup for instant rollback"
              checked={enableRollback}
              onCheckedChange={setEnableRollback}
            />
            
            <Toggle
              label="Dry-run always"
              description="Always show preview before actual deletion"
              checked={dryRunAlways}
              onCheckedChange={setDryRunAlways}
            />
            
            <div>
              <label className="block text-sm font-medium mb-2">Minimum file age</label>
              <select
                value={minFileAgeDays}
                onChange={(e) => setMinFileAgeDays(Number(e.target.value))}
                className="input max-w-xs"
              >
                <option value={1}>1 day</option>
                <option value={7}>1 week</option>
                <option value={30}>1 month</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium mb-2">Protected folders</label>
              <div className="space-y-2">
                {protectedPaths.map((path) => (
                  <div key={path} className="flex items-center gap-2 p-2 bg-background rounded">
                    <span className="flex-1 text-sm">{path}</span>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => removeProtectedPath(path)}
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                ))}
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newPath}
                    onChange={(e) => setNewPath(e.target.value)}
                    placeholder="/path/to/protect"
                    className="input flex-1"
                    onKeyDown={(e) => e.key === 'Enter' && handleAddPath()}
                  />
                  <Button onClick={handleAddPath} leftIcon={<Plus className="w-4 h-4" />}>
                    Add
                  </Button>
                </div>
              </div>
            </div>
          </div>
        )}
        
        {/* AI Tab */}
        {activeTab === 'ai' && (
          <div className="space-y-6">
            <Toggle
              label="Enable learning"
              description="Allow SlothCleaner to learn from your cleanup decisions"
              checked={enableLearning}
              onCheckedChange={setEnableLearning}
            />
            
            <div>
              <label className="block text-sm font-medium mb-2">
                Aggressiveness: {aggressiveness}/5
              </label>
              <input
                type="range"
                min="1"
                max="5"
                value={aggressiveness}
                onChange={(e) => setAggressiveness(Number(e.target.value))}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-text-secondary mt-1">
                <span>Conservative</span>
                <span>Aggressive</span>
              </div>
            </div>
            
            <Button
              variant="danger"
              onClick={() => {
                resetLearningData()
              }}
            >
              Reset Learning Data
            </Button>
          </div>
        )}
        
        {/* Advanced Tab */}
        {activeTab === 'advanced' && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium mb-2">Custom scan paths</label>
              <textarea
                className="input h-24"
                placeholder="/path/to/scan&#10;/another/path"
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium mb-2">Exclude patterns</label>
              <textarea
                className="input h-24"
                defaultValue="*.git*&#10;*node_modules*&#10;*.cache*"
              />
            </div>
            
            <div className="space-y-3">
              <Button variant="secondary" onClick={() => exportLogs()}>
                Export Logs
              </Button>
              <Button variant="danger">Reset All Settings</Button>
            </div>
          </div>
        )}
      </Card>
    </div>
  )
}
