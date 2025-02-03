import type { Metadata } from 'next';
import { GeistSans } from 'geist/font/sans';
import './globals.css';

export const metadata: Metadata = {
  title: 'Blazing Fast URL Shortener',
  description: 'Transform your long URLs into short links',
  icons: {
    icon: '/favicon.ico',
  },
  other: {
    'format-detection': 'telephone=no, date=no, email=no, address=no',
  },
};

function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${GeistSans.className} min-h-screen bg-white text-gray-900 antialiased`}
      >
        {children}
      </body>
    </html>
  );
}

export default RootLayout;
