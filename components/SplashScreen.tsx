import React, { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Hexagon } from 'lucide-react';

const SPLASH_KEY = 'worpen_splash_last';
const SPLASH_INTERVAL_MS = 20 * 60 * 1000; // at most once per 20 minutes
const SPLASH_DURATION_MS = 1900;

/** Returns true when the splash should be shown (20-min gate via localStorage). */
function shouldShow(): boolean {
  try {
    const last = Number(localStorage.getItem(SPLASH_KEY) || 0);
    if (Date.now() - last < SPLASH_INTERVAL_MS) return false;
    localStorage.setItem(SPLASH_KEY, String(Date.now()));
    return true;
  } catch {
    return true;
  }
}

export const SplashScreen: React.FC = () => {
  const [visible, setVisible] = useState(() => shouldShow());

  useEffect(() => {
    if (!visible) return;
    const t = setTimeout(() => setVisible(false), SPLASH_DURATION_MS);
    return () => clearTimeout(t);
  }, [visible]);

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-[#1e1f22]"
          exit={{ opacity: 0, filter: 'blur(8px)' }}
          transition={{ duration: 0.45, ease: 'easeOut' }}
        >
          <motion.div
            initial={{ scale: 0.85, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
            className="flex flex-col items-center"
          >
            <div className="relative w-20 h-20 flex items-center justify-center rounded-2xl bg-gradient-to-br from-[#e85d26] to-[#f2c55c] shadow-[0_0_60px_rgba(232,93,38,0.35)]">
              <Hexagon size={40} className="text-white" strokeWidth={2} />
            </div>
            <div className="mt-6 text-3xl font-bold tracking-tight text-[#dfe1e5]">
              Worpen
            </div>
            <div className="mt-2 text-[11px] font-bold uppercase tracking-[0.3em] text-[#e85d26]">
              Dynamic API Engine
            </div>
            <motion.div
              className="mt-8 h-[3px] w-48 overflow-hidden rounded-full bg-[#43454a]"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
            >
              <motion.div
                className="h-full rounded-full bg-gradient-to-r from-[#e85d26] to-[#f2c55c]"
                initial={{ x: '-100%' }}
                animate={{ x: '250%' }}
                transition={{ duration: 1.1, ease: 'easeInOut', repeat: 1 }}
              />
            </motion.div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
