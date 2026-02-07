import { useEffect, useRef, useState } from "react";

export function useTypewriter(text: string, speed: number) {
  const [displayedText, setDisplayedText] = useState("");
  const [isTyping, setIsTyping] = useState(false);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (speed === 0 || !text) {
      setDisplayedText(text);
      setIsTyping(false);
      return;
    }

    setDisplayedText("");
    setIsTyping(true);
    let idx = 0;

    intervalRef.current = setInterval(() => {
      idx++;
      if (idx >= text.length) {
        setDisplayedText(text);
        setIsTyping(false);
        if (intervalRef.current) clearInterval(intervalRef.current);
      } else {
        setDisplayedText(text.slice(0, idx));
      }
    }, speed);

    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [text, speed]);

  const skip = () => {
    if (intervalRef.current) clearInterval(intervalRef.current);
    setDisplayedText(text);
    setIsTyping(false);
  };

  return { displayedText, isTyping, skip };
}
