"use client"

import { useState } from "react"
import Link from "next/link"
import { Monitor, Apple, Terminal, Copy, Check } from "lucide-react"

export default function DocsPage() {
  const [activeTab, setActiveTab] = useState<"windows" | "linux" | "macos">("windows")
  const [copied, setCopied] = useState("")

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text)
    setCopied(id)
    setTimeout(() => setCopied(""), 2000)
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
          <Link href="/install" className="hover:text-cyan-400 transition-colors">Install</Link>
          <Link href="/docs" className="text-cyan-400">Docs</Link>
          <Link href="/about" className="hover:text-cyan-400 transition-colors">About</Link>
        </div>
      </nav>

      {/* Content */}
      <div className="container mx-auto px-6 py-16 max-w-6xl">
        <h1 className="text-5xl font-bold mb-4">Documentation</h1>
        <p className="text-zinc-400 text-xl mb-12">Complete setup guide for all platforms</p>

        {/* Platform Tabs */}
        <div className="flex gap-4 mb-8 border-b border-zinc-800">
          <button
            onClick={() => setActiveTab("windows")}
            className={`flex items-center gap-2 px-6 py-3 font-semibold transition-colors border-b-2 ${
              activeTab === "windows" 
                ? "text-cyan-400 border-cyan-400" 
                : "text-zinc-400 border-transparent hover:text-white"
            }`}
          >
            <Monitor className="w-5 h-5" />
            Windows
          </button>
          <button
            onClick={() => setActiveTab("linux")}
            className={`flex items-center gap-2 px-6 py-3 font-semibold transition-colors border-b-2 ${
              activeTab === "linux" 
                ? "text-cyan-400 border-cyan-400" 
                : "text-zinc-400 border-transparent hover:text-white"
            }`}
          >
            <Terminal className="w-5 h-5" />
            Linux
          </button>
          <button
            onClick={() => setActiveTab("macos")}
            className={`flex items-center gap-2 px-6 py-3 font-semibold transition-colors border-b-2 ${
              activeTab === "macos" 
                ? "text-cyan-400 border-cyan-400" 
                : "text-zinc-400 border-transparent hover:text-white"
            }`}
          >
            <Apple className="w-5 h-5" />
            macOS
          </button>
        </div>

        {/* Windows Guide */}
        {activeTab === "windows" && (
          <div className="space-y-8">
            <Section title="1. Install Node.js">
              <p className="text-zinc-400 mb-4">VaultX requires Node.js 16 or higher.</p>
              <CodeBlock 
                code="winget install OpenJS.NodeJS"
                onCopy={() => handleCopy("winget install OpenJS.NodeJS", "win-node")}
                copied={copied === "win-node"}
              />
              <p className="text-zinc-400 text-sm mt-2">Or download from <a href="https://nodejs.org" target="_blank" className="text-cyan-400 hover:underline">nodejs.org</a></p>
            </Section>

            <Section title="2. Install VaultX">
              <CodeBlock 
                code="npm install -g @vaultx-official/vaultx"
                onCopy={() => handleCopy("npm install -g @vaultx-official/vaultx", "win-install")}
                copied={copied === "win-install"}
              />
            </Section>

            <Section title="3. Verify Installation">
              <CodeBlock 
                code="vx --version"
                onCopy={() => handleCopy("vx --version", "win-verify")}
                copied={copied === "win-verify"}
              />
            </Section>

            <Section title="4. Environment Variables (Optional)">
              <p className="text-zinc-400 mb-4">NPM automatically adds VaultX to your PATH. If needed, manually add:</p>
              <CodeBlock 
                code="%APPDATA%\npm"
                onCopy={() => handleCopy("%APPDATA%\\npm", "win-path")}
                copied={copied === "win-path"}
              />
              <p className="text-zinc-400 text-sm mt-2">Add this to System Environment Variables → Path</p>
            </Section>

            <Section title="5. SSH Setup (Optional)">
              <p className="text-zinc-400 mb-4">For SSH key management:</p>
              <div className="space-y-2">
                <CodeBlock 
                  code="vx ssh init my-server"
                  onCopy={() => handleCopy("vx ssh init my-server", "win-ssh1")}
                  copied={copied === "win-ssh1"}
                />
                <p className="text-zinc-400 text-sm">Copy the public key to your server's <code className="text-cyan-400">~/.ssh/authorized_keys</code></p>
              </div>
            </Section>
          </div>
        )}

        {/* Linux Guide */}
        {activeTab === "linux" && (
          <div className="space-y-8">
            <Section title="1. Install Node.js">
              <p className="text-zinc-400 mb-4">Using package manager:</p>
              <CodeBlock 
                code="curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs"
                onCopy={() => handleCopy("curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -\nsudo apt-get install -y nodejs", "linux-node")}
                copied={copied === "linux-node"}
              />
            </Section>

            <Section title="2. Install VaultX">
              <CodeBlock 
                code="npm install -g @vaultx-official/vaultx"
                onCopy={() => handleCopy("npm install -g @vaultx-official/vaultx", "linux-install")}
                copied={copied === "linux-install"}
              />
            </Section>

            <Section title="3. Verify Installation">
              <CodeBlock 
                code="vx --version"
                onCopy={() => handleCopy("vx --version", "linux-verify")}
                copied={copied === "linux-verify"}
              />
            </Section>

            <Section title="4. File Permissions">
              <p className="text-zinc-400 mb-4">Ensure proper permissions for vault file:</p>
              <CodeBlock 
                code="chmod 600 ~/.vaultx/vault.vx"
                onCopy={() => handleCopy("chmod 600 ~/.vaultx/vault.vx", "linux-chmod")}
                copied={copied === "linux-chmod"}
              />
            </Section>

            <Section title="5. SSH Setup">
              <p className="text-zinc-400 mb-4">Generate and use SSH keys:</p>
              <div className="space-y-2">
                <CodeBlock 
                  code="vx ssh init my-server"
                  onCopy={() => handleCopy("vx ssh init my-server", "linux-ssh1")}
                  copied={copied === "linux-ssh1"}
                />
                <p className="text-zinc-400 text-sm mb-2">Copy public key to server:</p>
                <CodeBlock 
                  code="ssh-copy-id -i ~/.ssh/vaultx_key.pub user@server"
                  onCopy={() => handleCopy("ssh-copy-id -i ~/.ssh/vaultx_key.pub user@server", "linux-ssh2")}
                  copied={copied === "linux-ssh2"}
                />
                <p className="text-zinc-400 text-sm mb-2">Set correct permissions:</p>
                <CodeBlock 
                  code="chmod 600 ~/.ssh/vaultx_key
chmod 644 ~/.ssh/vaultx_key.pub"
                  onCopy={() => handleCopy("chmod 600 ~/.ssh/vaultx_key\nchmod 644 ~/.ssh/vaultx_key.pub", "linux-ssh3")}
                  copied={copied === "linux-ssh3"}
                />
              </div>
            </Section>

            <Section title="6. Passwordless SSH (Optional)">
              <p className="text-zinc-400 mb-4">For passwordless sudo on remote server:</p>
              <CodeBlock 
                code="echo 'username ALL=(ALL) NOPASSWD: ALL' | sudo tee /etc/sudoers.d/username"
                onCopy={() => handleCopy("echo 'username ALL=(ALL) NOPASSWD: ALL' | sudo tee /etc/sudoers.d/username", "linux-nopasswd")}
                copied={copied === "linux-nopasswd"}
              />
              <p className="text-zinc-400 text-sm mt-2">⚠️ Use with caution - only on trusted systems</p>
            </Section>
          </div>
        )}

        {/* macOS Guide */}
        {activeTab === "macos" && (
          <div className="space-y-8">
            <Section title="1. Install Node.js">
              <p className="text-zinc-400 mb-4">Using Homebrew:</p>
              <CodeBlock 
                code="brew install node"
                onCopy={() => handleCopy("brew install node", "mac-node")}
                copied={copied === "mac-node"}
              />
              <p className="text-zinc-400 text-sm mt-2">Don't have Homebrew? Install from <a href="https://brew.sh" target="_blank" className="text-cyan-400 hover:underline">brew.sh</a></p>
            </Section>

            <Section title="2. Install VaultX">
              <CodeBlock 
                code="npm install -g @vaultx-official/vaultx"
                onCopy={() => handleCopy("npm install -g @vaultx-official/vaultx", "mac-install")}
                copied={copied === "mac-install"}
              />
            </Section>

            <Section title="3. Verify Installation">
              <CodeBlock 
                code="vx --version"
                onCopy={() => handleCopy("vx --version", "mac-verify")}
                copied={copied === "mac-verify"}
              />
            </Section>

            <Section title="4. File Permissions">
              <p className="text-zinc-400 mb-4">Secure your vault file:</p>
              <CodeBlock 
                code="chmod 600 ~/.vaultx/vault.vx"
                onCopy={() => handleCopy("chmod 600 ~/.vaultx/vault.vx", "mac-chmod")}
                copied={copied === "mac-chmod"}
              />
            </Section>

            <Section title="5. SSH Setup">
              <p className="text-zinc-400 mb-4">Generate and configure SSH keys:</p>
              <div className="space-y-2">
                <CodeBlock 
                  code="vx ssh init my-server"
                  onCopy={() => handleCopy("vx ssh init my-server", "mac-ssh1")}
                  copied={copied === "mac-ssh1"}
                />
                <p className="text-zinc-400 text-sm mb-2">Add public key to server:</p>
                <CodeBlock 
                  code="cat ~/.ssh/vaultx_key.pub | ssh user@server 'cat >> ~/.ssh/authorized_keys'"
                  onCopy={() => handleCopy("cat ~/.ssh/vaultx_key.pub | ssh user@server 'cat >> ~/.ssh/authorized_keys'", "mac-ssh2")}
                  copied={copied === "mac-ssh2"}
                />
                <p className="text-zinc-400 text-sm mb-2">Set permissions:</p>
                <CodeBlock 
                  code="chmod 600 ~/.ssh/vaultx_key
chmod 644 ~/.ssh/vaultx_key.pub"
                  onCopy={() => handleCopy("chmod 600 ~/.ssh/vaultx_key\nchmod 644 ~/.ssh/vaultx_key.pub", "mac-ssh3")}
                  copied={copied === "mac-ssh3"}
                />
              </div>
            </Section>

            <Section title="6. Keychain Integration (Optional)">
              <p className="text-zinc-400 mb-4">Add SSH key to macOS Keychain:</p>
              <CodeBlock 
                code="ssh-add --apple-use-keychain ~/.ssh/vaultx_key"
                onCopy={() => handleCopy("ssh-add --apple-use-keychain ~/.ssh/vaultx_key", "mac-keychain")}
                copied={copied === "mac-keychain"}
              />
            </Section>
          </div>
        )}

        {/* Quick Start */}
        <div className="mt-16 bg-gradient-to-r from-cyan-500/10 to-purple-500/10 border border-cyan-500/20 rounded-xl p-8">
          <h2 className="text-3xl font-bold mb-4">Quick Start</h2>
          <div className="space-y-4">
            <div>
              <p className="text-zinc-400 mb-2">Initialize a project:</p>
              <CodeBlock 
                code="vx init my-project"
                onCopy={() => handleCopy("vx init my-project", "qs-init")}
                copied={copied === "qs-init"}
              />
            </div>
            <div>
              <p className="text-zinc-400 mb-2">Add a secret:</p>
              <CodeBlock 
                code="vx add my-project API_KEY"
                onCopy={() => handleCopy("vx add my-project API_KEY", "qs-add")}
                copied={copied === "qs-add"}
              />
            </div>
            <div>
              <p className="text-zinc-400 mb-2">Retrieve a secret:</p>
              <CodeBlock 
                code="vx get my-project API_KEY"
                onCopy={() => handleCopy("vx get my-project API_KEY", "qs-get")}
                copied={copied === "qs-get"}
              />
            </div>
            <div>
              <p className="text-zinc-400 mb-2">Run security audit:</p>
              <CodeBlock 
                code="vx audit"
                onCopy={() => handleCopy("vx audit", "qs-audit")}
                copied={copied === "qs-audit"}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-6">
      <h2 className="text-2xl font-bold mb-4 text-cyan-400">{title}</h2>
      {children}
    </div>
  )
}

function CodeBlock({ code, onCopy, copied }: { code: string; onCopy: () => void; copied: boolean }) {
  return (
    <div className="flex items-start gap-3 bg-black/50 border border-zinc-700 rounded-lg p-4 group">
      <pre className="text-zinc-300 font-mono text-sm flex-1 overflow-x-auto">
        <code>{code}</code>
      </pre>
      <button 
        onClick={onCopy}
        className="text-zinc-500 hover:text-white transition-colors flex-shrink-0"
      >
        {copied ? <Check size={18} className="text-green-400" /> : <Copy size={18} />}
      </button>
    </div>
  )
}
