"use client"

import { useEffect, useRef } from "react";

export default function AuroraBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let animationFrameId: number;
    let phase = 0;

    const resize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };

    resize();
    window.addEventListener("resize", resize);

    interface WaveLine {
      y: number;
      amplitude: number;
      frequency: number;
      phase: number;
      speed: number;
      color: string;
    }

    const lines: WaveLine[] = [];

    const initLines = () => {
      lines.length = 0;
      const config = {
        lineCount: 3,
        amplitude: 150,
        frequency: 0.002,
        speed: 0.002,
        colors: [
          "#7c3aed", // Electric Purple
          "#db2777", // Neon Magenta
          "#4f46e5"  // Indigo
        ]
      };

      const width = canvas.width;
      const height = canvas.height;

      for (let i = 0; i < config.lineCount; i++) {
        lines.push({
          y: height / 2 + (i * 80 - 150),
          amplitude: config.amplitude + Math.random() * 50,
          frequency: config.frequency + Math.random() * 0.001,
          phase: Math.random() * Math.PI * 2,
          speed: config.speed + Math.random() * 0.001,
          color: config.colors[i % config.colors.length]
        });
      }
    };

    initLines();

    const drawAurora = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Dark background
      ctx.fillStyle = "#050505";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const width = canvas.width;
      const height = canvas.height;

      // Draw each wave line
      lines.forEach((line) => {
        ctx.save();
        ctx.filter = "blur(40px)";

        ctx.beginPath();

        for (let x = 0; x <= width; x += 5) {
          const y = line.y +
            Math.sin(x * line.frequency + line.phase) * line.amplitude *
            Math.sin(line.phase * 0.1);

          if (x === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }

        // Style the line
        ctx.strokeStyle = line.color;
        ctx.lineWidth = 80;
        ctx.lineCap = "round";
        ctx.stroke();

        // Update phase for animation
        line.phase += line.speed;

        ctx.restore();
      });

      phase += 0.01;
      animationFrameId = requestAnimationFrame(drawAurora);
    };

    drawAurora();

    window.addEventListener("resize", () => {
      resize();
      initLines();
    });

    return () => {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return (
    <div className="absolute inset-0 -z-10 overflow-hidden pointer-events-none">
      <canvas
        ref={canvasRef}
        className="absolute inset-0 w-full h-full"
        style={{ opacity: 0.8 }}
      />
      {/* Gradient overlay for depth */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-[#050505]" />
      {/* Subtle vignette */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,transparent_0%,#050505_70%)] opacity-60" />
    </div>
  );
}
