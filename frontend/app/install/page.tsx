"use client"

import { useState } from "react"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Copy, Check, Download } from "lucide-react"

export default function InstallPage() {
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    navigator.clipboard.writeText("npm install -g @vaultx-official/vaultx")
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="min-h-screen bg-[#050505] text-white">
      {/* Navigation */}
      <nav className="container mx-auto px-6 py-6 flex justify-between items-center border-b border-white/5">
        <Link href="/" className="flex flex-col">
          <span className="text-3xl font-black tracking-tighter text-white">VAULTX</span>
          <span className="text-[0.6rem] tracking-[0.2em] text-cyan-500 font-bold uppercase">
            Fast. Secure. Modern.
          </span>
        </Link>
        
        <div className="flex items-center gap-8 text-sm font-medium text-zinc-400">
          <Link href="/#features" className="hover:text-cyan-400 transition-colors">Features</Link>
          <Link href="/install" className="text-cyan-400">Install</Link>
          <Link href="/docs" className="hover:text-cyan-400 transition-colors">Docs</Link>
          <Link href="/about" className="hover:text-cyan-400 transition-colors">About</Link>
        </div>
      </nav>

      {/* Content */}
      <div className="container mx-auto px-6 py-16 max-w-4xl">
        <h1 className="text-5xl font-bold mb-4">Install VaultX</h1>
        <p className="text-zinc-400 text-xl mb-12">Get started in seconds with NPM</p>

        {/* NPM Install */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-2xl font-bold mb-4 flex items-center gap-2">
            <Download className="w-6 h-6 text-cyan-400" />
            Quick Install
          </h2>
          
          <div className="flex items-center gap-3 bg-black/50 border border-zinc-700 rounded-lg p-4 mb-4">
            <span className="text-purple-400 font-mono text-lg">$</span>
            <code className="text-zinc-300 font-mono text-base flex-1">
              npm install -g @vaultx-official/vaultx
            </code>
            <button 
              onClick={handleCopy}
              className="text-zinc-500 hover:text-white transition-colors"
            >
              {copied ? <Check size={20} className="text-green-400" /> : <Copy size={20} />}
            </button>
          </div>

          <p className="text-zinc-400 text-sm">
            This will install VaultX globally on your system. Requires Node.js 16+
          </p>
        </div>

        {/* Verify Installation */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-2xl font-bold mb-4">Verify Installation</h2>
          
          <div className="bg-black/50 border border-zinc-700 rounded-lg p-4 mb-2">
            <code className="text-zinc-300 font-mono text-sm">
              <span className="text-purple-400">$</span> vx --version
            </code>
          </div>
          
          <div className="bg-black/50 border border-zinc-700 rounded-lg p-4">
            <code className="text-zinc-300 font-mono text-sm">
              <span className="text-purple-400">$</span> vx --help
            </code>
          </div>
        </div>

        {/* Alternative Methods */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8">
          <h2 className="text-2xl font-bold mb-4">Alternative Installation</h2>
          
          <div className="space-y-4">
            <div>
              <h3 className="text-lg font-semibold mb-2 text-cyan-400">Download Binary</h3>
              <p className="text-zinc-400 mb-2">
                Download pre-built binaries from GitHub Releases:
              </p>
              <a 
                href="https://github.com/skdas20/VaultX/releases" 
                target="_blank"
                className="inline-block bg-white text-black px-6 py-2 rounded-lg font-semibold hover:bg-zinc-200 transition-colors"
              >
                View Releases
              </a>
            </div>

            <div className="pt-4 border-t border-zinc-800">
              <h3 className="text-lg font-semibold mb-2 text-cyan-400">Build from Source</h3>
              <p className="text-zinc-400 mb-2">
                For advanced users who want to build from source:
              </p>
              <Link 
                href="/docs" 
                className="inline-block bg-zinc-800 text-white px-6 py-2 rounded-lg font-semibold hover:bg-zinc-700 transition-colors"
              >
                View Build Guide
              </Link>
            </div>
          </div>
        </div>

        {/* Next Steps */}
        <div className="mt-12 text-center">
          <p className="text-zinc-400 mb-4">Ready to get started?</p>
          <Link href="/docs">
            <Button className="bg-cyan-500 hover:bg-cyan-600 text-white px-8 py-3 rounded-full font-bold">
              Read the Docs →
            </Button>
          </Link>
        </div>
      </div>
    </div>
  )
}
