/**
 * Reusable animation helper functions for common UI effects
 */

export const animations = {
  // Fade effects
  fadeIn: "animate-fadeIn",
  fadeOut: "animate-fadeOut",

  // Slide effects
  slideDown: "animate-slideDown",
  slideUp: "animate-slideUp",
  slideInRight: "animate-slideInRight",
  slideOutLeft: "animate-slideOutLeft",

  // Pulse effects (for alerts/notifications)
  pulse: "animate-pulse",
  pulseRed: "animate-pulseRed",
  pulseGreen: "animate-pulseGreen",

  // Scale effects (for importance)
  scaleIn: "animate-scaleIn",
  scaleOut: "animate-scaleOut",

  // Special effects
  bounce: "animate-bounce",
  shimmer: "animate-shimmer",
  glow: "animate-glow",
};

/**
 * Get animation classes for health state changes
 * Used to indicate damage, healing, or critical HP
 */
export function getHealthAnimationClass(
  currentHP: number,
  maxHP: number,
  previousHP?: number,
): string {
  const healthPercent = (currentHP / maxHP) * 100;

  // Critical state
  if (healthPercent <= 25) {
    return "animate-pulseRed";
  }

  // Low health
  if (healthPercent <= 50) {
    return "animate-pulse";
  }

  // Just took damage (flash effect)
  if (previousHP !== undefined && currentHP < previousHP) {
    return "animate-damageFlash";
  }

  // Just healed (green glow)
  if (previousHP !== undefined && currentHP > previousHP) {
    return "animate-healGlow";
  }

  return "";
}

/**
 * Get animation for item pickup/drop
 */
export function getItemAnimationClass(action: "pickup" | "drop"): string {
  if (action === "pickup") {
    return "animate-slideInRight animate-fadeIn";
  }
  return "animate-slideOutLeft animate-fadeOut";
}

/**
 * Compose multiple animation classes with proper spacing
 */
export function composeAnimations(...animations: (string | undefined)[]): string {
  return animations.filter(Boolean).join(" ");
}

/**
 * Calculate animation delay based on list index
 * Useful for staggered animations in lists
 */
export function getStaggerDelay(index: number, baseDelay = 50): string {
  return `${index * baseDelay}ms`;
}
