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
  type: _type = "fade",
  duration = 300,
  children,
  onExited,
}: Props) {
  const [mounted, setMounted] = useState(show);
  const [visible, setVisible] = useState(show);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const frameRef = useRef<number | null>(null);
  const onExitedRef = useRef(onExited);
  const firstRenderRef = useRef(true);
  onExitedRef.current = onExited;

  useEffect(() => {
    if (show) {
      setMounted(true);
      if (firstRenderRef.current) {
        // Paint immediately on first load so headless browsers and audits see content.
        setVisible(true);
      } else {
        setVisible(false);
        frameRef.current = requestAnimationFrame(() => {
          frameRef.current = requestAnimationFrame(() => {
            setVisible(true);
          });
        });
      }
    } else {
      setVisible(false);
      timerRef.current = setTimeout(() => {
        setMounted(false);
        onExitedRef.current?.();
      }, duration);
    }
    firstRenderRef.current = false;
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
      if (frameRef.current !== null) cancelAnimationFrame(frameRef.current);
    };
  }, [show, duration]);

  if (!mounted) return null;

  // Use opacity-only transitions to avoid `transform` creating a new
  // containing block, which breaks `position: fixed` in children.
  const baseStyle: React.CSSProperties = {
    transition: `opacity ${duration}ms ease`,
  };

  const enterClass = visible ? "opacity-100" : "opacity-0";

  return (
    <div className={enterClass} style={baseStyle}>
      {children}
    </div>
  );
}
