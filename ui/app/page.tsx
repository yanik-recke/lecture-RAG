"use client"

import { useState } from "react"
import { TranscribeTab } from "@/components/transcribe-tab"
import { ChatTab } from "@/components/chat-tab"
import { CollectionTab } from "@/components/collection-tab"

export default function Home() {
  const [activeTab, setActiveTab] = useState<"transcribe" | "ask" | "collection">("transcribe")

  return (
    <main className="min-h-screen bg-white">
      <div className="sticky top-0 z-10 bg-white border-b border-border">
        <div className="w-full max-w-3xl mx-auto px-6 flex h-14">
          <button
            onClick={() => setActiveTab("transcribe")}
            className={`flex-1 text-base font-medium transition-colors hover:text-foreground ${
              activeTab === "transcribe"
                ? "text-foreground border-b-2 border-foreground"
                : "text-muted-foreground border-b-2 border-transparent"
            }`}
          >
            Transcribe
          </button>
          <button
            onClick={() => setActiveTab("ask")}
            className={`flex-1 text-base font-medium transition-colors hover:text-foreground ${
              activeTab === "ask"
                ? "text-foreground border-b-2 border-foreground"
                : "text-muted-foreground border-b-2 border-transparent"
            }`}
          >
            Ask Question
          </button>
          <button
            onClick={() => setActiveTab("collection")}
            className={`flex-1 text-base font-medium transition-colors hover:text-foreground ${
              activeTab === "collection"
                ? "text-foreground border-b-2 border-foreground"
                : "text-muted-foreground border-b-2 border-transparent"
            }`}
          >
            Collection
          </button>
        </div>
      </div>

      <div className={`container mx-auto px-6 ${activeTab === "ask" ? "h-[calc(100vh-4rem)] overflow-hidden" : ""}`}>
        <div className={activeTab === "ask" ? "h-full py-6" : "mt-8"}>
          <div className="max-w-3xl mx-auto">
            {activeTab === "transcribe" && <TranscribeTab />}
          </div>
          {activeTab === "ask" && <ChatTab />}
          <div className="max-w-3xl mx-auto">
            {activeTab === "collection" && <CollectionTab />}
          </div>
        </div>
      </div>
    </main>
  )
}
