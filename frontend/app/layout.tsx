import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "VaultX - Secure SSH Client & Developer Vault | Free Secret Manager",
  description: "VaultX is a free, high-performance SSH client, SSH identity manager, and secure developer vault with military-grade AES-256-GCM encryption. Manage secrets, multiple shells, and easy SCP transfers with zero-trust security.",
  keywords: [
    "security vault",
    "vaultx",
    "ssh client",
    "free developer vault",
    "ssh identity manager",
    "multiple shell manager",
    "easy scp",
    "secrets management",
    "environment variables",
    "AES-256 encryption",
    "CLI tool",
    "developer tools",
    "secure vault",
    "password manager",
    "ssh manager",
    "terminal manager",
    "secure file transfer",
    "encrypted storage",
    "zero trust security"
  ],
  authors: [{ name: "Sumit Kumar Das", url: "https://www.linkedin.com/in/sumitkumardas-ai" }],
  creator: "Sumit Kumar Das",
  publisher: "VaultX",
  applicationName: "VaultX",
  category: "Developer Tools",
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    url: "https://vaultx.developersumit.me",
    siteName: "VaultX",
    title: "VaultX - Secure SSH Client & Developer Vault | Free Secret Manager",
    description: "Free SSH client, SSH identity manager, and secure developer vault with military-grade encryption. Manage multiple shells, secrets, and easy SCP transfers.",
    images: [
      {
        url: "https://vaultx.developersumit.me/og-image.png",
        width: 1200,
        height: 630,
        alt: "VaultX - Secure SSH Client & Developer Vault",
        type: "image/png",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "VaultX - Secure SSH Client & Developer Vault",
    description: "Free SSH client, SSH identity manager, and secure developer vault with military-grade AES-256-GCM encryption.",
    images: ["https://vaultx.developersumit.me/og-image.png"],
    creator: "@developersumit",
  },
  icons: {
    icon: [
      { url: "/favicon.ico", sizes: "any" },
      { url: "/icon.svg", type: "image/svg+xml" },
    ],
    apple: [
      { url: "/apple-icon.png", sizes: "180x180", type: "image/png" },
    ],
  },
  manifest: "/manifest.json",
  alternates: {
    canonical: "https://vaultx.developersumit.me",
  },
  verification: {
    google: "google-site-verification-code",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        {children}
      </body>
    </html>
  );
}
