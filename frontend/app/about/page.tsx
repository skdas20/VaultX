"use client"

import Link from "next/link"
import { Shield, Zap, Lock, Code, Github, Twitter } from "lucide-react"

export default function AboutPage() {
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
          <Link href="/install" className="hover:text-cyan-400 transition-colors">Install</Link>
          <Link href="/docs" className="hover:text-cyan-400 transition-colors">Docs</Link>
          <Link href="/about" className="text-cyan-400">About</Link>
        </div>
      </nav>

      {/* Hero */}
      <div className="container mx-auto px-6 py-16 max-w-4xl">
        <h1 className="text-5xl md:text-6xl font-bold mb-6">
          About <span className="text-cyan-400">VaultX</span>
        </h1>
        <p className="text-zinc-400 text-xl leading-relaxed mb-12">
          A production-grade, CLI-first, zero-trust developer vault built with security and performance in mind.
        </p>

        {/* Mission */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-3xl font-bold mb-4">Our Mission</h2>
          <p className="text-zinc-400 text-lg leading-relaxed">
            VaultX was created to solve a critical problem: developers need a fast, secure, and simple way to manage secrets without compromising on security or usability. We believe that security tools should be developer-friendly, not obstacles to productivity.
          </p>
        </div>

        {/* Core Principles */}
        <div className="grid md:grid-cols-2 gap-6 mb-12">
          <PrincipleCard 
            icon={<Shield className="w-8 h-8 text-cyan-400" />}
            title="Security First"
            description="Military-grade AES-256-GCM encryption with Argon2id key derivation. Your secrets never leave your machine unencrypted."
          />
          <PrincipleCard 
            icon={<Zap className="w-8 h-8 text-yellow-400" />}
            title="Lightning Fast"
            description="Built with Rust for maximum performance. Operations complete in milliseconds, not seconds."
          />
          <PrincipleCard 
            icon={<Lock className="w-8 h-8 text-purple-500" />}
            title="Zero Trust"
            description="Local-first architecture. No cloud dependencies, no backend servers, no data collection. You own your data."
          />
          <PrincipleCard 
            icon={<Code className="w-8 h-8 text-green-400" />}
            title="Developer Focused"
            description="Intuitive CLI designed for developers. Integrates seamlessly into your workflow."
          />
        </div>

        {/* Technology Stack */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-3xl font-bold mb-6">Technology Stack</h2>
          <div className="grid md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-xl font-semibold mb-2 text-cyan-400">Core</h3>
              <ul className="space-y-2 text-zinc-400">
                <li>• Rust (vx-core)</li>
                <li>• WebAssembly (WASM)</li>
                <li>• Node.js (CLI wrapper)</li>
              </ul>
            </div>
            <div>
              <h3 className="text-xl font-semibold mb-2 text-cyan-400">Cryptography</h3>
              <ul className="space-y-2 text-zinc-400">
                <li>• AES-256-GCM encryption</li>
                <li>• Argon2id key derivation</li>
                <li>• ed25519 SSH keys</li>
              </ul>
            </div>
          </div>
        </div>

        {/* Features */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-3xl font-bold mb-6">Key Features</h2>
          <ul className="space-y-3 text-zinc-400 text-lg">
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>Encrypted project-based vault with single file storage</span>
            </li>
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>TTL-based secret expiration for automatic rotation</span>
            </li>
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>Security audit to identify expired and high-risk secrets</span>
            </li>
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>SSH identity management with ed25519 keys</span>
            </li>
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>Cross-platform support (Windows, Linux, macOS)</span>
            </li>
            <li className="flex items-start gap-3">
              <span className="text-cyan-400 mt-1">✓</span>
              <span>Ultra-lightweight binary (~2-3 MB)</span>
            </li>
          </ul>
        </div>

        {/* Open Source */}
        <div className="bg-gradient-to-r from-cyan-500/10 to-purple-500/10 border border-cyan-500/20 rounded-xl p-8 mb-8">
          <h2 className="text-3xl font-bold mb-4">Open Source</h2>
          <p className="text-zinc-400 text-lg mb-6">
            VaultX is open source and available on GitHub. We believe in transparency and community-driven development.
          </p>
          <div className="flex gap-4">
            <a 
              href="https://github.com/skdas20/VaultX" 
              target="_blank"
              className="flex items-center gap-2 bg-white text-black px-6 py-3 rounded-lg font-semibold hover:bg-zinc-200 transition-colors"
            >
              <Github className="w-5 h-5" />
              View on GitHub
            </a>
            <a 
              href="https://github.com/skdas20/VaultX/issues" 
              target="_blank"
              className="flex items-center gap-2 bg-zinc-800 text-white px-6 py-3 rounded-lg font-semibold hover:bg-zinc-700 transition-colors"
            >
              Report Issue
            </a>
          </div>
        </div>

        {/* Team */}
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 mb-8">
          <h2 className="text-3xl font-bold mb-4">Built By Developers, For Developers</h2>
          <p className="text-zinc-400 text-lg leading-relaxed">
            VaultX is maintained by a team of security-conscious developers who understand the challenges of managing secrets in modern development workflows. We're committed to building tools that make developers' lives easier without compromising on security.
          </p>
        </div>

        {/* Contact */}
        <div className="text-center">
          <h2 className="text-2xl font-bold mb-4">Get in Touch</h2>
          <p className="text-zinc-400 mb-6">
            Have questions or feedback? We'd love to hear from you.
          </p>
          <div className="flex justify-center gap-4">
            <a 
              href="https://github.com/skdas20/VaultX" 
              target="_blank"
              className="flex items-center gap-2 text-zinc-400 hover:text-white transition-colors"
            >
              <Github className="w-5 h-5" />
              GitHub
            </a>
            <a 
              href="https://twitter.com/vaultx" 
              target="_blank"
              className="flex items-center gap-2 text-zinc-400 hover:text-white transition-colors"
            >
              <Twitter className="w-5 h-5" />
              Twitter
            </a>
          </div>
        </div>
      </div>
    </div>
  )
}

function PrincipleCard({ icon, title, description }: { icon: React.ReactNode; title: string; description: string }) {
  return (
    <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-6 hover:border-cyan-500/30 transition-colors">
      <div className="mb-4 p-3 rounded-lg bg-black/50 w-fit border border-white/5">
        {icon}
      </div>
      <h3 className="text-xl font-bold mb-2">{title}</h3>
      <p className="text-zinc-400 leading-relaxed">{description}</p>
    </div>
  )
}
