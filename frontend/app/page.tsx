"use client"

import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Shield, Terminal, Settings, Lock, Copy, Check } from "lucide-react"
import { motion } from "framer-motion"
import Link from "next/link"
import Image from "next/image"
import { useState } from "react"

export default function Home() {
  const [copied, setCopied] = useState(false)

  const copyCommand = () => {
    navigator.clipboard.writeText("npm install -g @vaultx-official/vaultx")
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="min-h-screen bg-[#020617] text-white overflow-x-hidden font-sans selection:bg-cyan-500/30">
      
      {/* Background Effects */}
      <div className="fixed inset-0 pointer-events-none">
        <div className="absolute top-[-20%] right-[-10%] w-[800px] h-[800px] bg-cyan-500/10 rounded-full blur-[120px]" />
        <div className="absolute bottom-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-600/10 rounded-full blur-[100px]" />
      </div>

      {/* Navigation */}
      <nav className="relative z-50 container mx-auto px-6 py-6 flex justify-between items-center">
        <div className="flex flex-col">
          <span className="text-3xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-500">
            VAULTX
          </span>
          <span className="text-[0.6rem] tracking-[0.2em] text-cyan-500/80 font-bold uppercase">
            Fast. Secure. Modern.
          </span>
        </div>
        
        <div className="hidden md:flex items-center gap-8 text-sm font-medium text-gray-400">
          <Link href="#features" className="hover:text-cyan-400 transition-colors">Features</Link>
          <Link href="#install" className="hover:text-cyan-400 transition-colors">Install</Link>
          <Link href="#docs" className="hover:text-cyan-400 transition-colors">Docs</Link>
          <Link href="#about" className="hover:text-cyan-400 transition-colors">About</Link>
        </div>

        <Button className="hidden md:flex bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-400 border border-cyan-500/50 rounded-full px-6 font-bold shadow-[0_0_15px_rgba(6,182,212,0.3)] hover:shadow-[0_0_25px_rgba(6,182,212,0.5)] transition-all">
          INSTALL NOW
        </Button>
      </nav>

      {/* Hero Section */}
      <section className="relative z-10 container mx-auto px-6 py-12 lg:py-20">
        <div className="grid lg:grid-cols-2 gap-16 items-center">
          
          {/* Left Column: Text & Features */}
          <motion.div 
            initial={{ opacity: 0, x: -50 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8 }}
          >
            <h1 className="text-5xl lg:text-7xl font-black leading-[1.1] mb-8">
              SECURE YOUR <br />
              <span className="text-cyan-400">SECRETS</span> WITH <br />
              EASE.
            </h1>

            {/* Feature Grid (2x2) */}
            <div className="grid sm:grid-cols-2 gap-4 mb-12">
              <FeatureBox icon={<Shield className="w-6 h-6 text-cyan-400" />} text="End-to-End Encryption" />
              <FeatureBox icon={<Settings className="w-6 h-6 text-cyan-400" />} text="Cross-Platform Support" />
              <FeatureBox icon={<Terminal className="w-6 h-6 text-cyan-400" />} text="Simple CLI Interface" />
              <FeatureBox icon={<Lock className="w-6 h-6 text-cyan-400" />} text="Rust-Powered Performance" />
            </div>

            {/* Install Bar */}
            <div 
              className="group relative flex items-center bg-black/40 border border-cyan-500/30 rounded-lg p-4 backdrop-blur-sm cursor-pointer hover:border-cyan-500/60 transition-colors"
              onClick={copyCommand}
            >
              <div className="mr-4 text-cyan-500 font-bold text-lg">{">"}</div>
              <code className="flex-1 font-mono text-lg text-gray-200">
                npm install -g @vaultx-official/vaultx
              </code>
              <div className="p-2 rounded-md hover:bg-cyan-500/20 transition-colors">
                {copied ? <Check className="w-5 h-5 text-green-400" /> : <Copy className="w-5 h-5 text-cyan-500" />}
              </div>
              
              {/* Glow Effect behind bar */}
              <div className="absolute -inset-1 bg-gradient-to-r from-cyan-500/20 to-blue-500/20 rounded-lg blur opacity-50 group-hover:opacity-100 transition-opacity -z-10" />
            </div>
          </motion.div>

          {/* Right Column: 3D Visuals */}
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="relative h-[600px] w-full flex items-center justify-center"
          >
            {/* 
                NOTE: Using the mockup image as the primary visual since we want to match the "website.png" exactly.
                In a real scenario without the image, we would build this with CSS 3D transforms.
            */}
            <div className="relative w-full h-full">
              {/* Glowing Circle Backgrounds */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] border border-cyan-500/10 rounded-full animate-[spin_10s_linear_infinite]" />
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[350px] h-[350px] border border-cyan-500/20 rounded-full animate-[spin_15s_linear_infinite_reverse]" />
              
              {/* Main Image Layer */}
              <Image 
                src="/hero-mockup.png" 
                alt="VaultX Interface" 
                fill 
                className="object-contain drop-shadow-[0_0_50px_rgba(6,182,212,0.3)]"
              />
            </div>
          </motion.div>
        </div>
      </section>

      {/* Footer / Credits */}
      <footer className="container mx-auto px-6 py-8 mt-12 border-t border-white/5 flex justify-between items-center text-sm text-gray-500">
        <div className="flex gap-4">
          <span>Twitter</span>
          <span>GitHub</span>
          <span>Instagram</span>
        </div>
        <div>
           © 2025 VaultX. All Rights Reserved.
        </div>
      </footer>
    </div>
  )
}

function FeatureBox({ icon, text }: { icon: React.ReactNode, text: string }) {
  return (
    <div className="flex items-center gap-3 p-4 rounded-xl bg-white/5 border border-white/10 hover:border-cyan-500/30 hover:bg-white/10 transition-all group">
      <div className="p-2 rounded-lg bg-black/50 border border-white/5 group-hover:border-cyan-500/50 group-hover:shadow-[0_0_10px_rgba(6,182,212,0.2)] transition-all">
        {icon}
      </div>
      <span className="font-medium text-gray-200">{text}</span>
    </div>
  )
}
