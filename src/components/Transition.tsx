import { useEffect, useRef, useState } from "react";

interface Props {
  show: boolean;
  type?: "fade" | "slideUp";
  duration?: number;
  children: React.ReactNode;
  onExited?: () => void;
}

export function Transition({
  show,
  type = "fade",
  duration = 300,
  children,
  onExited,
}: Props) {
  const [mounted, setMounted] = useState(show);
  const [visible, setVisible] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (show) {
      setMounted(true);
      // Force a reflow before setting visible to trigger CSS transition
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          setVisible(true);
        });
      });
    } else {
      setVisible(false);
      timerRef.current = setTimeout(() => {
        setMounted(false);
        onExited?.();
      }, duration);
    }
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [show, duration, onExited]);

  if (!mounted) return null;

  const baseStyle: React.CSSProperties = {
    transition: `opacity ${duration}ms ease, transform ${duration}ms ease`,
  };

  const enterClass = visible
    ? "opacity-100 translate-y-0"
    : type === "slideUp"
      ? "opacity-0 translate-y-4"
      : "opacity-0";

  return (
    <div className={`transition-transform ${enterClass}`} style={baseStyle}>
      {children}
    </div>
  );
}
