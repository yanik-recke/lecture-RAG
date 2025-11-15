"use client"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ChevronRight, Folder, FileAudio, ChevronDown } from 'lucide-react'

// Placeholder data structure for when API becomes available
type Module = {
  id: string
  name: string
  lectureCount: number
}

type Lecture = {
  id: string
  name: string
  uploadedAt: string
  summary: string
}

export function CollectionTab() {
  const [selectedModule, setSelectedModule] = useState<string | null>(null)
  const [expandedLectures, setExpandedLectures] = useState<Set<string>>(new Set())

  // Placeholder modules - replace with API call when available
  const modules: Module[] = [
    { id: "1", name: "Introduction to Programming", lectureCount: 5 },
    { id: "2", name: "Data Structures", lectureCount: 8 },
    { id: "3", name: "Algorithms", lectureCount: 6 },
  ]

  const getLecturesForModule = (moduleId: string): Lecture[] => {
    // This will be replaced with actual API call
    if (moduleId === "1") {
      return [
        { 
          id: "1", 
          name: "Lecture 1: Variables", 
          uploadedAt: "2024-01-15",
          summary: "This lecture covers the fundamentals of variables in programming, including variable declaration, initialization, and scope. We explore different data types such as integers, floats, strings, and booleans. The lecture also discusses naming conventions, best practices for variable usage, and common pitfalls to avoid when working with variables in modern programming languages."
        },
        { 
          id: "2", 
          name: "Lecture 2: Functions", 
          uploadedAt: "2024-01-16",
          summary: "An in-depth exploration of functions and their role in code organization and reusability. Topics include function parameters, return values, function overloading, and recursive functions. We also cover higher-order functions, lambda expressions, and the importance of pure functions in functional programming paradigms."
        },
        { 
          id: "3", 
          name: "Lecture 3: Control Flow", 
          uploadedAt: "2024-01-17",
          summary: "Understanding control flow structures including if-else statements, switch cases, and various loop constructs. The lecture demonstrates practical examples of when to use each structure and how to avoid common mistakes like infinite loops and deeply nested conditionals."
        },
      ]
    }
    return []
  }

  const toggleSummary = (lectureId: string) => {
    setExpandedLectures(prev => {
      const newSet = new Set(prev)
      if (newSet.has(lectureId)) {
        newSet.delete(lectureId)
      } else {
        newSet.add(lectureId)
      }
      return newSet
    })
  }

  return (
    <Card className="border-border bg-white shadow-sm">
      <CardHeader>
        <CardTitle>Collection</CardTitle>
        <CardDescription>
          {selectedModule ? "Lectures in selected module" : "View modules and their lectures"}
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!selectedModule ? (
          <div className="space-y-2">
            {modules.length === 0 ? (
              <p className="text-muted-foreground text-sm text-center py-8">
                No modules available yet
              </p>
            ) : (
              modules.map((module) => (
                <button
                  key={module.id}
                  onClick={() => setSelectedModule(module.id)}
                  className="w-full flex items-center justify-between p-4 border border-border rounded-lg hover:bg-muted/50 transition-colors"
                >
                  <div className="flex items-center gap-3">
                    <Folder className="h-5 w-5 text-muted-foreground" />
                    <div className="text-left">
                      <p className="font-medium">{module.name}</p>
                      <p className="text-sm text-muted-foreground">
                        {module.lectureCount} lecture{module.lectureCount !== 1 ? 's' : ''}
                      </p>
                    </div>
                  </div>
                  <ChevronRight className="h-5 w-5 text-muted-foreground" />
                </button>
              ))
            )}
          </div>
        ) : (
          <div className="space-y-4">
            <Button
              variant="ghost"
              onClick={() => setSelectedModule(null)}
              className="mb-2"
            >
              ← Back to Modules
            </Button>
            <div className="space-y-2">
              {getLecturesForModule(selectedModule).map((lecture) => (
                <div
                  key={lecture.id}
                  className="border border-border rounded-lg"
                >
                  <div className="flex items-center gap-3 p-4">
                    <FileAudio className="h-5 w-5 text-muted-foreground" />
                    <div className="flex-1">
                      <p className="font-medium">{lecture.name}</p>
                      <p className="text-sm text-muted-foreground">
                        Uploaded: {new Date(lecture.uploadedAt).toLocaleDateString()}
                      </p>
                    </div>
                  </div>
                  
                  <div className="border-t border-border">
                    <button
                      onClick={() => toggleSummary(lecture.id)}
                      className="w-full flex items-center justify-between p-4 hover:bg-muted/50 transition-colors"
                    >
                      <span className="text-sm font-medium">Summary</span>
                      <ChevronDown 
                        className={`h-4 w-4 text-muted-foreground transition-transform ${
                          expandedLectures.has(lecture.id) ? 'rotate-180' : ''
                        }`}
                      />
                    </button>
                    {expandedLectures.has(lecture.id) && (
                      <div className="px-4 pb-4">
                        <p className="text-sm text-muted-foreground leading-relaxed">
                          {lecture.summary}
                        </p>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
