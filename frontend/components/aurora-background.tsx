export default function AuroraBackground() {
  return (
    <div className="absolute inset-0 -z-10 overflow-hidden pointer-events-none">
      <svg
        className="absolute inset-0 h-full w-full opacity-60"
        xmlns="http://www.w3.org/2000/svg"
        preserveAspectRatio="none"
      >
        <defs>
          <filter id="blur">
            <feGaussianBlur stdDeviation="80" />
          </filter>
        </defs>

        <g filter="url(#blur)">
          <path
            d="M -200 300 Q 300 100 800 300 T 1600 300 V 0 H -200 Z"
            fill="url(#grad1)"
            className="animate-wave1"
          />
          <path
            d="M -200 500 Q 300 300 800 500 T 1600 500 V 0 H -200 Z"
            fill="url(#grad2)"
            className="animate-wave2"
          />
        </g>

        <defs>
          <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stopColor="#7c7cff" />
            <stop offset="100%" stopColor="#22d3ee" />
          </linearGradient>

          <linearGradient id="grad2" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stopColor="#9333ea" />
            <stop offset="100%" stopColor="#0ea5e9" />
          </linearGradient>
        </defs>
      </svg>
      {/* Optional subtle noise overlay */}
      <div className="absolute inset-0 bg-transparent opacity-[0.03] mix-blend-overlay" style={{backgroundImage: 'url("data:image/svg+xml,%3Csvg viewBox=\'0 0 200 200\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cfilter id=\'noiseFilter\'%3E%3CfeTurbulence type=\'fractalNoise\' baseFrequency=\'0.65\' numOctaves=\'3\' stitchTiles=\'stitch\'%2F%3E%3C/filter%3E%3Crect width=\'100%25\' height=\'100%25\' filter=\'url(%23noiseFilter)\'/%3E%3C/svg%3E")'}}></div>
    </div>
  );
}
