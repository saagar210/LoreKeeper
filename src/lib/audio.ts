import type { SoundCue } from "../store/types";

let audioCtx: AudioContext | null = null;
let masterGain: GainNode | null = null;

function getContext(): AudioContext {
  if (!audioCtx) {
    audioCtx = new AudioContext();
    masterGain = audioCtx.createGain();
    masterGain.connect(audioCtx.destination);
  }
  if (audioCtx.state === "suspended") {
    audioCtx.resume();
  }
  return audioCtx;
}

function getGain(): GainNode {
  getContext();
  return masterGain!;
}

export function setVolume(volume: number) {
  getGain().gain.setValueAtTime(Math.max(0, Math.min(1, volume)), getContext().currentTime);
}

function playTone(freq: number, duration: number, type: OscillatorType = "square") {
  const ctx = getContext();
  const osc = ctx.createOscillator();
  const gain = ctx.createGain();
  osc.type = type;
  osc.frequency.setValueAtTime(freq, ctx.currentTime);
  gain.gain.setValueAtTime(0.3, ctx.currentTime);
  gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + duration);
  osc.connect(gain);
  gain.connect(getGain());
  osc.start(ctx.currentTime);
  osc.stop(ctx.currentTime + duration);
}

function playNoise(duration: number, volume: number = 0.1) {
  const ctx = getContext();
  const bufferSize = ctx.sampleRate * duration;
  const buffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
  const data = buffer.getChannelData(0);
  for (let i = 0; i < bufferSize; i++) {
    data[i] = (Math.random() * 2 - 1) * volume;
  }
  const source = ctx.createBufferSource();
  source.buffer = buffer;
  const gain = ctx.createGain();
  gain.gain.setValueAtTime(volume, ctx.currentTime);
  gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + duration);
  source.connect(gain);
  gain.connect(getGain());
  source.start(ctx.currentTime);
}

const soundMap: Record<SoundCue, () => void> = {
  ambientPeaceful: () => playTone(220, 0.8, "sine"),
  ambientDark: () => playTone(110, 1.0, "sawtooth"),
  ambientTense: () => playTone(165, 0.6, "triangle"),
  ambientSacred: () => playTone(330, 1.0, "sine"),
  combatHit: () => {
    playTone(200, 0.1, "square");
    playNoise(0.15, 0.2);
  },
  combatMiss: () => playTone(150, 0.15, "triangle"),
  combatVictory: () => {
    playTone(440, 0.15, "square");
    setTimeout(() => playTone(554, 0.15, "square"), 150);
    setTimeout(() => playTone(659, 0.3, "square"), 300);
  },
  itemPickup: () => {
    playTone(523, 0.1, "square");
    setTimeout(() => playTone(659, 0.15, "square"), 100);
  },
  itemDrop: () => playTone(330, 0.2, "triangle"),
  itemUse: () => playTone(440, 0.2, "sine"),
  doorUnlock: () => {
    playTone(262, 0.1, "square");
    setTimeout(() => playTone(330, 0.1, "square"), 100);
    setTimeout(() => playTone(392, 0.2, "square"), 200);
  },
  questComplete: () => {
    playTone(392, 0.15, "square");
    setTimeout(() => playTone(494, 0.15, "square"), 150);
    setTimeout(() => playTone(587, 0.15, "square"), 300);
    setTimeout(() => playTone(784, 0.4, "square"), 450);
  },
  questStart: () => {
    playTone(330, 0.2, "triangle");
    setTimeout(() => playTone(440, 0.3, "triangle"), 200);
  },
  playerDeath: () => {
    playTone(294, 0.3, "sawtooth");
    setTimeout(() => playTone(220, 0.3, "sawtooth"), 300);
    setTimeout(() => playTone(165, 0.5, "sawtooth"), 600);
  },
  npcGreeting: () => playTone(349, 0.2, "sine"),
  fleeSuccess: () => {
    playTone(392, 0.1, "square");
    setTimeout(() => playTone(494, 0.15, "square"), 100);
  },
  fleeFail: () => {
    playTone(294, 0.15, "square");
    setTimeout(() => playTone(220, 0.2, "square"), 150);
  },
};

export function playSoundCue(cue: SoundCue) {
  soundMap[cue]();
}

export function playSoundCues(cues: SoundCue[]) {
  for (const cue of cues) {
    playSoundCue(cue);
  }
}
