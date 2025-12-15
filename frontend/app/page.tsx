"use client"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { ArrowRight, Download, Github, Lock, Shield, Terminal, Zap } from "lucide-react"
import { motion } from "framer-motion"
import Link from "next/link"

export default function Home() {
  return (
    <div className="min-h-screen bg-background text-foreground overflow-x-hidden">
      {/* Navigation */}
      <nav className="border-b">
        <div className="container mx-auto flex items-center justify-between px-4 py-4">
          <div className="flex items-center gap-2 font-bold text-xl">
            <Shield className="h-6 w-6 text-primary" />
            <span>VaultX</span>
          </div>
          <div className="flex items-center gap-4">
            <Link href="#features" className="text-sm font-medium hover:text-primary transition-colors">
              Features
            </Link>
            <Link href="#download" className="text-sm font-medium hover:text-primary transition-colors">
              Download
            </Link>
            <Link href="https://github.com/skdas20/VaultX" target="_blank">
              <Button variant="ghost" size="icon">
                <Github className="h-5 w-5" />
              </Button>
            </Link>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="container mx-auto px-4 py-24 md:py-32 flex flex-col items-center text-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
        >
          <div className="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80 mb-4">
            v0.2.0 Available Now
          </div>
          <h1 className="text-4xl md:text-6xl font-extrabold tracking-tight mb-6">
            Zero-Trust Secrets Management <br className="hidden md:block" />
            <span className="text-primary">For Developers</span>
          </h1>
          <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto mb-8">
            Securely store secrets, manage SSH identities, and share credentials without leaving your terminal. 
            Local-first, encrypted, and built with Rust.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link href="#download">
              <Button size="lg" className="w-full sm:w-auto gap-2">
                <Download className="h-4 w-4" /> Download CLI
              </Button>
            </Link>
            <Link href="https://github.com/skdas20/VaultX" target="_blank">
              <Button size="lg" variant="outline" className="w-full sm:w-auto gap-2">
                <Github className="h-4 w-4" /> View Source
              </Button>
            </Link>
          </div>
        </motion.div>
      </section>

      {/* Features Grid */}
      <section id="features" className="bg-muted/50 py-24">
        <div className="container mx-auto px-4">
          <div className="text-center mb-16">
            <h2 className="text-3xl font-bold mb-4">Why VaultX?</h2>
            <p className="text-muted-foreground max-w-2xl mx-auto">
              Built for speed, security, and simplicity. No complex servers to manage.
            </p>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <FeatureCard 
              icon={<Lock className="h-10 w-10 text-primary" />}
              title="AES-256 Encryption"
              description="Bank-grade encryption for all your secrets. Only you hold the keys."
            />
            <FeatureCard 
              icon={<Terminal className="h-10 w-10 text-primary" />}
              title="CLI First"
              description="Designed for your terminal. Fast, scriptable, and intuitive commands."
            />
            <FeatureCard 
              icon={<Zap className="h-10 w-10 text-primary" />}
              title="TTL Expiration"
              description="Set secrets to auto-expire. Perfect for temporary tokens and sharing."
            />
          </div>
        </div>
      </section>

      {/* Download Section */}
      <section id="download" className="py-24">
        <div className="container mx-auto px-4">
          <div className="max-w-4xl mx-auto">
            <Card className="overflow-hidden">
              <CardHeader className="bg-primary text-primary-foreground p-8 text-center">
                <CardTitle className="text-3xl">Get Started in Seconds</CardTitle>
                <CardDescription className="text-primary-foreground/80 text-lg">
                  Install via npm or download the binary directly.
                </CardDescription>
              </CardHeader>
              <CardContent className="p-8 space-y-8">
                <div>
                  <h3 className="text-xl font-semibold mb-4 flex items-center gap-2">
                    <Terminal className="h-5 w-5" /> Install with NPM (Recommended)
                  </h3>
                  <div className="bg-muted p-4 rounded-md font-mono text-sm flex items-center justify-between">
                    <code>npm install -g @vaultx-official/vaultx</code>
                    <Button variant="ghost" size="sm" onClick={() => navigator.clipboard.writeText('npm install -g @vaultx-official/vaultx')}>
                      Copy
                    </Button>
                  </div>
                </div>

                <div className="grid md:grid-cols-2 gap-8">
                  <div>
                    <h3 className="text-xl font-semibold mb-4">Manual Download</h3>
                    <div className="space-y-2">
                      <DownloadButton platform="Windows (x64)" href="https://github.com/skdas20/VaultX/releases/latest/download/vx-x86_64-pc-windows-msvc.exe" />
                      <DownloadButton platform="Linux (x64)" href="https://github.com/skdas20/VaultX/releases/latest/download/vx-x86_64-unknown-linux-gnu" />
                      <DownloadButton platform="macOS (Intel)" href="https://github.com/skdas20/VaultX/releases/latest/download/vx-x86_64-apple-darwin" />
                      <DownloadButton platform="macOS (Apple Silicon)" href="https://github.com/skdas20/VaultX/releases/latest/download/vx-aarch64-apple-darwin" />
                    </div>
                  </div>
                  <div className="bg-muted/30 p-6 rounded-lg border">
                    <h3 className="font-semibold mb-2">After Installation</h3>
                    <p className="text-sm text-muted-foreground mb-4">
                      Initialize your first vault project to get started.
                    </p>
                    <div className="bg-background border p-3 rounded font-mono text-xs">
                      $ vx init my-project<br/>
                      $ vx add my-project DB_PASS<br/>
                      $ vx get my-project DB_PASS
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* Contribute Section */}
      <section className="py-24 bg-secondary/30">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-3xl font-bold mb-6">Join the Community</h2>
          <p className="text-lg text-muted-foreground max-w-2xl mx-auto mb-8">
            VaultX is open source. We welcome contributions, bug reports, and feature requests.
          </p>
          <div className="flex justify-center gap-4">
            <Link href="https://github.com/skdas20/VaultX" target="_blank">
              <Button size="lg" className="gap-2">
                <Github className="h-5 w-5" /> Contribute on GitHub
              </Button>
            </Link>
            <Link href="mailto:skdas5405@gmail.com">
               <Button size="lg" variant="outline" className="gap-2">
                Contact Maintainer
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t py-12">
        <div className="container mx-auto px-4 flex flex-col md:flex-row justify-between items-center gap-6">
          <div className="flex items-center gap-2 font-bold">
            <Shield className="h-5 w-5" />
            <span>VaultX</span>
          </div>
          <p className="text-sm text-muted-foreground">
            © {new Date().getFullYear()} VaultX. Released under MIT License.
          </p>
        </div>
      </footer>
    </div>
  )
}

function FeatureCard({ icon, title, description }: { icon: React.ReactNode, title: string, description: string }) {
  return (
    <Card className="border-none shadow-none bg-background/60">
      <CardHeader>
        <div className="mb-4">{icon}</div>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <CardDescription className="text-base">{description}</CardDescription>
      </CardContent>
    </Card>
  )
}

function DownloadButton({ platform, href }: { platform: string, href: string }) {
  return (
    <Link href={href} className="flex items-center justify-between p-3 rounded border hover:bg-muted transition-colors group">
      <span className="font-medium">{platform}</span>
      <Download className="h-4 w-4 text-muted-foreground group-hover:text-primary" />
    </Link>
  )
}