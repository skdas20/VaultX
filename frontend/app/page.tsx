"use client"

import { Button } from "@/components/ui/button"
import Link from "next/link"
import VaultXHero from "@/components/VaultXHero"
import { Shield, Terminal, Settings, Lock, Menu, X, Server } from "lucide-react"
import { useState } from "react"

export default function Home() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const structuredData = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "name": "VaultX",
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": "Windows, macOS, Linux",
    "offers": {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "USD"
    },
    "description": "Free SSH client, SSH identity manager, and secure developer vault with military-grade AES-256-GCM encryption. Manage secrets, multiple shells, and easy SCP transfers.",
    "softwareVersion": "0.3.7",
    "author": {
      "@type": "Person",
      "name": "Sumit Kumar Das",
      "url": "https://www.linkedin.com/in/sumitkumardas-ai"
    },
    "publisher": {
      "@type": "Organization",
      "name": "VaultX"
    },
    "aggregateRating": {
      "@type": "AggregateRating",
      "ratingValue": "5",
      "ratingCount": "1"
    },
    "featureList": [
      "Military-grade AES-256-GCM encryption",
      "SSH client and identity manager",
      "Multiple shell manager",
      "Easy SCP file transfer",
      "Cross-platform support",
      "Zero-trust security",
      "CLI developer tool"
    ]
  };

  return (
    <div className="min-h-screen bg-[#050505] text-white overflow-x-hidden font-sans selection:bg-cyan-500/30">
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(structuredData) }}
      />

      {/* Navigation */}
      <nav className="relative z-50 container mx-auto px-4 md:px-6 py-4 md:py-6 flex justify-between items-center bg-[#050505]/80 backdrop-blur-md sticky top-0 border-b border-white/5">
        <Link href="/" className="flex flex-col flex-shrink-0">
          <span className="text-2xl md:text-3xl font-black tracking-tighter text-cyan-400">
            VAULTX
          </span>
          <span className="text-[0.5rem] md:text-[0.6rem] tracking-[0.2em] text-cyan-500 font-bold uppercase">
            Fast. Secure. Modern.
          </span>
        </Link>

        {/* Desktop Menu */}
        <div className="hidden md:flex items-center gap-8 text-sm font-medium text-zinc-400">
          <Link href="#features" className="hover:text-cyan-400 transition-colors">Features</Link>
          <Link href="/install" className="hover:text-cyan-400 transition-colors">Install</Link>
          <Link href="/docs" className="hover:text-cyan-400 transition-colors">Docs</Link>
          <Link href="/about" className="hover:text-cyan-400 transition-colors">About</Link>
        </div>

        {/* Mobile Menu Button */}
        <button
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          className="md:hidden text-cyan-400 hover:text-cyan-300 transition-colors"
        >
          {mobileMenuOpen ? <X size={24} /> : <Menu size={24} />}
        </button>
      </nav>

      {/* Mobile Menu Dropdown */}
      {mobileMenuOpen && (
        <div className="md:hidden fixed top-16 left-0 right-0 bg-[#050505]/95 backdrop-blur-md border-b border-white/5 z-40 animate-in fade-in slide-in-from-top-2">
          <div className="container mx-auto px-4 py-4 flex flex-col gap-3 text-sm font-medium text-zinc-400">
            <Link href="#features" onClick={() => setMobileMenuOpen(false)} className="hover:text-cyan-400 transition-colors py-2">Features</Link>
            <Link href="/install" onClick={() => setMobileMenuOpen(false)} className="hover:text-cyan-400 transition-colors py-2">Install</Link>
            <Link href="/docs" onClick={() => setMobileMenuOpen(false)} className="hover:text-cyan-400 transition-colors py-2">Docs</Link>
            <Link href="/about" onClick={() => setMobileMenuOpen(false)} className="hover:text-cyan-400 transition-colors py-2">About</Link>
          </div>
        </div>
      )}

      {/* Hero Section */}
      <VaultXHero />

      {/* Features Section */}
      <section id="features" className="container mx-auto px-6 py-24 relative z-10">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-5xl font-bold mb-4 tracking-tight">
            Why <span className="text-cyan-400">VaultX</span>?
          </h2>
          <p className="text-zinc-400 max-w-2xl mx-auto text-lg">
            Built for developers who demand security without compromising speed.
          </p>
        </div>
        
        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          <FeatureBox
            icon={<Shield className="w-8 h-8 text-cyan-400" />}
            title="Military-Grade"
            desc="AES-256-GCM encryption ensures your secrets are uncrackable."
          />
          <FeatureBox
            icon={<Server className="w-8 h-8 text-purple-500" />}
            title="SSH Manager"
            desc="Built-in SSH client with identity management and seamless SCP transfers."
          />
          <FeatureBox
            icon={<Terminal className="w-8 h-8 text-green-400" />}
            title="Developer First"
            desc="Intuitive CLI designed for rapid development workflows."
          />
          <FeatureBox
            icon={<Lock className="w-8 h-8 text-yellow-400" />}
            title="Cross-Platform"
            desc="Works seamlessly on Linux, macOS, and Windows with session caching."
          />
        </div>
      </section>

      {/* Footer / Credits */}
      <footer className="container mx-auto px-6 py-12 mt-12 border-t border-white/5 flex flex-col md:flex-row justify-between items-center text-sm text-zinc-600">
        <div className="mb-4 md:mb-0 font-medium">
          By <Link href="https://www.linkedin.com/in/sumitkumardas-ai" target="_blank" rel="noopener noreferrer" className="text-cyan-400 hover:text-cyan-300 transition-colors">Sumit Kumar Das</Link>
        </div>
        <div>
           © 2025 VaultX. All Rights Reserved.
        </div>
      </footer>
    </div>
  )
}

function FeatureBox({ icon, title, desc }: { icon: React.ReactNode, title: string, desc: string }) {
  return (
    <div className="p-6 rounded-2xl bg-zinc-900/50 border border-white/5 hover:border-cyan-500/30 transition-colors group">
      <div className="mb-4 p-3 rounded-lg bg-black/50 w-fit border border-white/5 group-hover:border-cyan-500/20 transition-colors">
        {icon}
      </div>
      <h3 className="text-xl font-bold mb-2 text-white">{title}</h3>
      <p className="text-zinc-400 leading-relaxed text-sm">
        {desc}
      </p>
    </div>
  )
}
