"use client"

import { Button } from "@/components/ui/button"
import Link from "next/link"
import VaultXHero from "@/components/VaultXHero"

export default function Home() {
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
      <VaultXHero />

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