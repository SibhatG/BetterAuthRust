/**
 * Type definitions for accessibility features
 */

export interface AccessibilityPreferences {
  high_contrast: boolean;
  large_text: boolean;
  screen_reader_optimized: boolean;
  reduced_motion: boolean;
  voice_commands_enabled: boolean;
  keyboard_navigation: boolean;
  additional_settings?: Record<string, string>;
}

export enum CaptchaAlternative {
  Standard = 'Standard',
  Audio = 'Audio',
  SimpleMath = 'SimpleMath',
  LogicPuzzle = 'LogicPuzzle',
  ImageSelection = 'ImageSelection',
}

export interface VoiceCommand {
  command: string;
  confidence: number;
  action: string;
}

export interface UpdateAccessibilityPreferencesRequest {
  high_contrast?: boolean;
  large_text?: boolean;
  screen_reader_optimized?: boolean;
  reduced_motion?: boolean;
  voice_commands_enabled?: boolean;
  keyboard_navigation?: boolean;
  additional_settings?: Record<string, string>;
}