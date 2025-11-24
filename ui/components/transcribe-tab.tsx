"use client"

import { useState, useEffect } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Upload, FileAudio, Loader2 } from 'lucide-react'
import { useToast } from "@/hooks/use-toast"

export function TranscribeTab() {
  const [file, setFile] = useState<File | null>(null)
  const [lectureName, setLectureName] = useState("")
  const [module, setModule] = useState("")
  const [loading, setLoading] = useState(false)
  const [result, setResult] = useState<string | null>(null)
  const [apiUrl, setApiUrl] = useState<string>('')
  const { toast } = useToast()

  // Fetch API URL on mount
  useEffect(() => {
    fetch('/api/config')
      .then((res) => res.json())
      .then((data) => setApiUrl(data.apiUrl))
      .catch((err) => {
        console.error('Failed to fetch config:', err);
        setApiUrl('http://localhost:40999');
      });
  }, []);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0]
    if (selectedFile) {
      // Check file size (max 100 MB)
      const maxSize = 100 * 1024 * 1024 // 100 MB in bytes
      if (selectedFile.size > maxSize) {
        toast({
          title: "File too large",
          description: "Maximum file size is 100 MB",
          variant: "destructive",
        })
        return
      }
      
      // Check file type
      if (!selectedFile.type.includes("audio") && !selectedFile.name.endsWith(".mp3")) {
        toast({
          title: "Invalid file type",
          description: "Please upload an MP3 file",
          variant: "destructive",
        })
        return
      }
      
      setFile(selectedFile)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!file || !lectureName || !module) {
      toast({
        title: "Missing information",
        description: "Please fill in all fields and upload a file",
        variant: "destructive",
      })
      return
    }

    setLoading(true)
    setResult(null)

    try {
      const formData = new FormData()
      formData.append("file", file)

      if (!apiUrl) {
        throw new Error('API URL not configured');
      }
      const response = await fetch(
        `/api/v1/transcription?lectureName=${encodeURIComponent(lectureName)}&module=${encodeURIComponent(module)}`,
        {
          method: "POST",
          body: formData,
        }
      )

      if (!response.ok) {
        throw new Error("Transcription failed")
      }

      const data = await response.text()
      setResult(data)
      toast({
        title: "Success",
        description: "File added to transcription queue",
      })
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to transcribe the audio file",
        variant: "destructive",
      })
      console.error("Transcription error:", error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Card className="border-border bg-white shadow-sm">
      <CardHeader>
        <CardTitle>Transcribe Audio</CardTitle>
        <CardDescription>Upload an MP3 file to transcribe (max 100 MB)</CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="module">Module</Label>
            <Input
              id="module"
              value={module}
              onChange={(e) => setModule(e.target.value)}
              placeholder="Enter module name"
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="lectureName">Lecture Name</Label>
            <Input
              id="lectureName"
              value={lectureName}
              onChange={(e) => setLectureName(e.target.value)}
              placeholder="Enter lecture name"
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="audio-upload">Audio File</Label>
            <div className="flex items-center gap-4">
              <Input
                id="audio-upload"
                type="file"
                accept="audio/mp3,.mp3"
                onChange={handleFileChange}
                className="hidden"
              />
              <Button
                type="button"
                variant="outline"
                onClick={() => document.getElementById("audio-upload")?.click()}
                className="w-full"
              >
                <Upload className="mr-2 h-4 w-4" />
                {file ? "Change File" : "Upload MP3"}
              </Button>
            </div>
            {file && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground mt-2">
                <FileAudio className="h-4 w-4" />
                <span>{file.name}</span>
                <span className="text-xs">
                  ({(file.size / (1024 * 1024)).toFixed(2)} MB)
                </span>
              </div>
            )}
          </div>

          <Button type="submit" disabled={loading} className="w-full">
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Transcribing...
              </>
            ) : (
              "Transcribe"
            )}
          </Button>

          {result && (
            <div className="mt-4 p-4 bg-muted rounded-lg">
              <h3 className="text-sm font-semibold mb-2">Transcription Result:</h3>
              <p className="text-sm whitespace-pre-wrap">{result}</p>
            </div>
          )}
        </form>
      </CardContent>
    </Card>
  )
}
