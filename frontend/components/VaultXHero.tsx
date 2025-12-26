"use client"

import React, { useState, useEffect } from "react";
import { motion } from "framer-motion";
import { Shield, Key, Settings, Copy, Check } from "lucide-react";
import { cn } from "@/lib/utils";
import AuroraBackground from "@/components/aurora-background";

export default function VaultXHero() {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText("npm install -g @vaultx-official/vaultx");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative w-full min-h-screen bg-[#050505] overflow-hidden flex items-center justify-center font-sans selection:bg-cyan-500/30">
      
      {/* --- Aurora Background --- */}
      <div className="absolute inset-0 z-0">
        <AuroraBackground />
      </div>

      <div className="container mx-auto px-4 grid lg:grid-cols-2 gap-8 lg:gap-12 items-center relative z-10">

        {/* --- LEFT SIDE: Copy & CTA --- */}
        <div className="text-left space-y-6 lg:space-y-8 order-2 lg:order-1">
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <span className="inline-block py-2 px-4 rounded-full bg-cyan-950/40 border border-cyan-500/40 text-cyan-300 text-xs font-semibold tracking-widest mb-4 md:mb-6 backdrop-blur-sm hover:bg-cyan-950/60 transition-colors">
              v0.3.6 RELEASE
            </span>
            <h1 className="text-3xl sm:text-4xl md:text-6xl lg:text-7xl font-black tracking-tighter text-white mb-4 md:mb-8 leading-tight">
              Secure secrets. <br />
              <span className="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-purple-400 to-pink-500">
                Instantly.
              </span>
            </h1>
            <p className="text-zinc-300 text-base sm:text-lg md:text-xl max-w-lg leading-relaxed font-medium">
              A high-performance CLI for managing environment variables and secrets with military-grade encryption.
            </p>
          </motion.div>

          {/* Installation Command Bar */}
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2, duration: 0.5 }}
            className="group relative flex flex-col sm:flex-row items-start sm:items-center gap-3 bg-zinc-950/60 border border-purple-500/20 rounded-xl p-3 sm:p-5 max-w-full sm:max-w-md hover:border-purple-500/40 transition-all duration-300 backdrop-blur-sm shadow-lg shadow-purple-500/5"
          >
            <span className="text-cyan-400 font-mono text-lg font-bold flex-shrink-0">$</span>
            <code className="text-zinc-200 font-mono text-xs sm:text-sm flex-1 font-medium break-all">
              npm install -g @vaultx-official/vaultx
            </code>
            <button
              onClick={handleCopy}
              className="text-zinc-400 hover:text-cyan-400 transition-colors p-2 hover:bg-cyan-500/10 rounded-lg flex-shrink-0"
            >
              {copied ? <Check size={18} className="text-green-400" /> : <Copy size={18} />}
            </button>
          </motion.div>
        </div>

        {/* --- RIGHT SIDE: The 3D Animation --- */}
        <div className="relative h-[300px] sm:h-[400px] md:h-[500px] w-full flex items-center justify-center [perspective:1000px] order-1 lg:order-2">
            {/* This div creates the 3D space */}
            <motion.div
              className="relative w-full sm:w-[380px] md:w-[420px] lg:w-[520px] aspect-[4/3] max-w-[95vw] sm:max-w-none"
              initial={{ rotateX: 20, rotateY: -20, rotateZ: 5, scale: 0.9 }}
              animate={{
                rotateX: [20, 25, 20],
                rotateY: [-20, -15, -20],
                y: [0, -20, 0] // Floating effect for the whole container
              }}
              transition={{
                duration: 6,
                repeat: Infinity,
                ease: "easeInOut"
              }}
              style={{ transformStyle: "preserve-3d" }}
            >
                {/* --- The Terminal Card --- */}
                <div className="absolute inset-0 bg-black/80 backdrop-blur-xl border border-zinc-700 rounded-xl shadow-[0_0_80px_-10px_rgba(6,182,212,0.4),0_0_60px_-5px_rgba(59,130,246,0.3)] overflow-hidden flex flex-col">
                    {/* Terminal Header */}
                    <div className="h-10 bg-zinc-900/80 border-b border-zinc-800 flex items-center px-4 gap-2">
                        <div className="w-3 h-3 rounded-full bg-red-500/80"></div>
                        <div className="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                        <div className="w-3 h-3 rounded-full bg-green-500/80"></div>
                        <div className="ml-auto text-xs text-zinc-500 font-mono">bash — 80x24</div>
                    </div>
                    
                    {/* Terminal Body */}
                    <div className="flex-1 p-6 font-mono text-sm md:text-base text-green-400/90 shadow-inner bg-black/40">
                        <div className="flex flex-col gap-2">
                            <TypingLine text="vx init" delay={0} />
                            <TypingLine text="> Initializing safe vault..." delay={1500} color="text-zinc-400" />
                            <TypingLine text="> Vault created successfully." delay={2500} color="text-cyan-400" />
                            <div className="h-4"></div>
                            {/* The Main Command from your prompt */}
                            <div className="flex flex-wrap">
                                <span className="text-purple-400 mr-2">$</span>
                                <TypingLine 
                                    text="vx add my-project API_KEY --ttl 6h" 
                                    delay={3500} 
                                    cursor 
                                />
                            </div>
                        </div>
                    </div>

                    {/* Glossy Reflection overlay */}
                    <div className="absolute inset-0 bg-gradient-to-tr from-white/5 to-transparent pointer-events-none"></div>
                </div>

                {/* --- Floating 3D Elements --- */}

                {/* 1. Rust Gear (Settings Icon proxy) - Hidden on mobile */}
                <div className="hidden sm:block">
                  <FloatingIcon
                      icon={<Settings size={40} />}
                      color="text-orange-500"
                      glow="shadow-[0_0_30px_-5px_rgba(249,115,22,0.6)]"
                      position="-top-12 -right-12"
                      delay={0}
                  />
                </div>

                {/* 2. Security Shield - Hidden on mobile */}
                <div className="hidden sm:block">
                  <FloatingIcon
                      icon={<Shield size={48} />}
                      color="text-cyan-400"
                      glow="shadow-[0_0_30px_-5px_rgba(34,211,238,0.6)]"
                      position="top-1/2 -left-16"
                      delay={1}
                  />
                </div>

                {/* 3. Golden Key - Hidden on mobile */}
                <div className="hidden sm:block">
                  <FloatingIcon
                      icon={<Key size={40} />}
                      color="text-yellow-400"
                      glow="shadow-[0_0_30px_-5px_rgba(250,204,21,0.6)]"
                      position="-bottom-10 -right-8"
                      delay={2}
                  />
                </div>
            </motion.div>
        </div>
      </div>
    </div>
  );
}

// Sub-component for individual typing lines
function TypingLine({ text, delay, color = "text-green-400", cursor = false }: { text: string, delay: number, color?: string, cursor?: boolean }) {
    const [displayedText, setDisplayedText] = useState("");

    useEffect(() => {
        const timeout = setTimeout(() => {
            let i = 0;
            const typingInterval = setInterval(() => {
                if (i < text.length) {
                    setDisplayedText(text.substring(0, i + 1));
                    i++;
                } else {
                    clearInterval(typingInterval);
                }
            }, 50); // Typing speed
            return () => clearInterval(typingInterval);
        }, delay);
        return () => clearTimeout(timeout);
    }, [text, delay]);

    return (
        <span className={color}>
            {displayedText}
            {cursor && displayedText.length === text.length && (
                <motion.span 
                    animate={{ opacity: [0, 1, 0] }}
                    transition={{ repeat: Infinity, duration: 0.8 }}
                    className="inline-block w-2 h-4 bg-green-400 ml-1 align-middle"
                />
            )}
        </span>
    );
}

// Sub-component for the floating icons
function FloatingIcon({ icon, color, glow, position, delay }: { icon: React.ReactNode, color: string, glow: string, position: string, delay: number }) {
    return (
        <motion.div
            className={cn(
                "absolute bg-zinc-900/90 border border-zinc-700 p-4 rounded-xl backdrop-blur-md z-20", 
                color, glow, position
            )}
            initial={{ y: 0 }}
            animate={{ 
                y: [-10, 10, -10],
                rotate: [0, 5, -5, 0]
            }}
            transition={{ 
                duration: 4, 
                repeat: Infinity, 
                ease: "easeInOut",
                delay: delay 
            }}
            style={{ transform: "translateZ(50px)" }} // Pushes it "closer" to camera in 3D space
        >
            {icon}
        </motion.div>
    );
}
