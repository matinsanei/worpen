
import React from 'react';
import { motion } from 'framer-motion';

interface PageTransitionProps {
    children: React.ReactNode;
    className?: string;
}

const variants = {
    initial: { opacity: 0, x: 20, filter: 'blur(10px)' },
    enter: { opacity: 1, x: 0, filter: 'blur(0px)' },
    exit: { opacity: 0, x: -20, filter: 'blur(10px)' },
};

export const PageTransition: React.FC<PageTransitionProps> = ({ children, className = "" }) => {
    return (
        <motion.div
            initial="initial"
            animate="enter"
            exit="exit"
            variants={variants}
            transition={{ type: "spring", stiffness: 300, damping: 30 }}
            className={`h-full w-full ${className}`}
        >
            {children}
        </motion.div>
    );
};
