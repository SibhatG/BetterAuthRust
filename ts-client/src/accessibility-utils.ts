/**
 * Accessibility utilities for the Better Auth client
 */

import { AccessibilityPreferences } from './types/accessibility';

/**
 * Class to handle client-side accessibility features
 */
export class AccessibilityUtils {
  private static instance: AccessibilityUtils;
  private preferences: AccessibilityPreferences;
  private readonly storageKey = 'better_auth_accessibility_prefs';

  /**
   * Private constructor for singleton pattern
   */
  private constructor() {
    // Default preferences
    this.preferences = {
      high_contrast: false,
      large_text: false,
      screen_reader_optimized: false,
      reduced_motion: false,
      voice_commands_enabled: false,
      keyboard_navigation: true,
    };

    // Load preferences from local storage if available
    this.loadPreferences();
  }

  /**
   * Get the singleton instance
   */
  public static getInstance(): AccessibilityUtils {
    if (!AccessibilityUtils.instance) {
      AccessibilityUtils.instance = new AccessibilityUtils();
    }
    return AccessibilityUtils.instance;
  }

  /**
   * Load preferences from localStorage
   */
  private loadPreferences(): void {
    try {
      const savedPrefs = localStorage.getItem(this.storageKey);
      if (savedPrefs) {
        this.preferences = { ...this.preferences, ...JSON.parse(savedPrefs) };
        this.applyPreferences();
      }
    } catch (error) {
      console.error('Failed to load accessibility preferences:', error);
    }
  }

  /**
   * Save preferences to localStorage
   */
  private savePreferences(): void {
    try {
      localStorage.setItem(this.storageKey, JSON.stringify(this.preferences));
    } catch (error) {
      console.error('Failed to save accessibility preferences:', error);
    }
  }

  /**
   * Get current preferences
   */
  public getPreferences(): AccessibilityPreferences {
    return { ...this.preferences };
  }

  /**
   * Update preferences
   */
  public updatePreferences(newPrefs: Partial<AccessibilityPreferences>): void {
    this.preferences = { ...this.preferences, ...newPrefs };
    this.savePreferences();
    this.applyPreferences();
  }

  /**
   * Apply the current preferences to the UI
   */
  public applyPreferences(): void {
    const { 
      high_contrast, 
      large_text, 
      screen_reader_optimized, 
      reduced_motion,
      keyboard_navigation
    } = this.preferences;

    const html = document.documentElement;

    // High contrast mode
    if (high_contrast) {
      html.classList.add('high-contrast');
    } else {
      html.classList.remove('high-contrast');
    }

    // Large text
    if (large_text) {
      html.classList.add('large-text');
    } else {
      html.classList.remove('large-text');
    }

    // Screen reader optimizations
    if (screen_reader_optimized) {
      html.classList.add('screen-reader');
    } else {
      html.classList.remove('screen-reader');
    }

    // Reduced motion
    if (reduced_motion) {
      html.classList.add('reduced-motion');
    } else {
      html.classList.remove('reduced-motion');
    }

    // Keyboard navigation
    if (keyboard_navigation) {
      html.classList.add('keyboard-nav');
    } else {
      html.classList.remove('keyboard-nav');
    }

    // Apply CSS variables
    this.applyCssVariables();
  }

  /**
   * Apply CSS variables based on preferences
   */
  private applyCssVariables(): void {
    const root = document.documentElement;
    const { high_contrast, large_text } = this.preferences;

    // High contrast variables
    if (high_contrast) {
      root.style.setProperty('--text-color', '#ffffff');
      root.style.setProperty('--background-color', '#000000');
      root.style.setProperty('--primary-color', '#ffff00');
      root.style.setProperty('--secondary-color', '#00ffff');
      root.style.setProperty('--error-color', '#ff6666');
      root.style.setProperty('--success-color', '#66ff66');
      root.style.setProperty('--border-color', '#ffffff');
    } else {
      root.style.setProperty('--text-color', '');
      root.style.setProperty('--background-color', '');
      root.style.setProperty('--primary-color', '');
      root.style.setProperty('--secondary-color', '');
      root.style.setProperty('--error-color', '');
      root.style.setProperty('--success-color', '');
      root.style.setProperty('--border-color', '');
    }

    // Large text variables
    if (large_text) {
      root.style.setProperty('--base-font-size', '18px');
      root.style.setProperty('--heading-font-size', '24px');
      root.style.setProperty('--button-font-size', '18px');
      root.style.setProperty('--input-font-size', '18px');
    } else {
      root.style.setProperty('--base-font-size', '');
      root.style.setProperty('--heading-font-size', '');
      root.style.setProperty('--button-font-size', '');
      root.style.setProperty('--input-font-size', '');
    }
  }

  /**
   * Get CSS stylesheet for accessibility
   */
  public getAccessibilityStylesheet(): string {
    return `
      /* High Contrast Mode */
      .high-contrast {
        --text-color: #ffffff;
        --background-color: #000000;
        --primary-color: #ffff00;
        --secondary-color: #00ffff;
        --error-color: #ff6666;
        --success-color: #66ff66;
        --border-color: #ffffff;
      }
      
      .high-contrast body {
        background-color: var(--background-color);
        color: var(--text-color);
      }
      
      .high-contrast button,
      .high-contrast input,
      .high-contrast select,
      .high-contrast textarea {
        background-color: #333333;
        color: var(--text-color);
        border: 2px solid var(--border-color);
      }
      
      .high-contrast a {
        color: var(--primary-color);
        text-decoration: underline;
      }
      
      .high-contrast .error {
        color: var(--error-color);
        font-weight: bold;
      }
      
      /* Large Text Mode */
      .large-text {
        --base-font-size: 18px;
        --heading-font-size: 24px;
        --button-font-size: 18px;
        --input-font-size: 18px;
      }
      
      .large-text body {
        font-size: var(--base-font-size);
      }
      
      .large-text h1, .large-text h2, .large-text h3, .large-text h4 {
        font-size: var(--heading-font-size);
      }
      
      .large-text button {
        font-size: var(--button-font-size);
        padding: 12px 20px;
      }
      
      .large-text input, .large-text select, .large-text textarea {
        font-size: var(--input-font-size);
        padding: 12px;
      }
      
      /* Screen Reader Optimizations */
      .screen-reader .sr-only {
        display: block;
        position: static;
        width: auto;
        height: auto;
        margin: 0;
        clip: auto;
        clip-path: none;
        overflow: visible;
      }
      
      /* Reduced Motion */
      .reduced-motion * {
        transition: none !important;
        animation: none !important;
      }
      
      /* Keyboard Navigation */
      .keyboard-nav :focus {
        outline: 3px solid #4d90fe !important;
        outline-offset: 2px !important;
      }
    `;
  }

  /**
   * Initialize voice commands
   */
  public initVoiceCommands(callback: (command: string) => void): void {
    if (!this.preferences.voice_commands_enabled) {
      return;
    }

    // Check if browser supports speech recognition
    const SpeechRecognition = (window as any).SpeechRecognition || 
                             (window as any).webkitSpeechRecognition;
    
    if (!SpeechRecognition) {
      console.warn('Speech recognition not supported in this browser');
      return;
    }

    try {
      const recognition = new SpeechRecognition();
      recognition.continuous = true;
      recognition.interimResults = false;
      recognition.lang = 'en-US';

      recognition.onresult = (event: any) => {
        const last = event.results.length - 1;
        const command = event.results[last][0].transcript.trim().toLowerCase();
        
        console.log('Voice command received:', command);
        callback(command);
      };

      recognition.onerror = (event: any) => {
        console.error('Voice recognition error:', event.error);
      };

      recognition.start();
      console.log('Voice commands enabled');
    } catch (error) {
      console.error('Failed to initialize voice commands:', error);
    }
  }

  /**
   * Process a voice command
   */
  public processVoiceCommand(command: string): string | null {
    const normalizedCommand = command.toLowerCase().trim();
    
    // Simple voice command mapping
    const commandMap: Record<string, string> = {
      'login': 'login',
      'sign in': 'login',
      'register': 'register',
      'sign up': 'register',
      'log out': 'logout',
      'sign out': 'logout',
      'reset password': 'reset-password',
      'high contrast': 'toggle-high-contrast',
      'large text': 'toggle-large-text',
      'reduce motion': 'toggle-reduced-motion',
    };

    // Find matching command
    for (const [trigger, action] of Object.entries(commandMap)) {
      if (normalizedCommand.includes(trigger)) {
        return action;
      }
    }

    return null;
  }

  /**
   * Add keyboard shortcuts for accessibility
   */
  public setupKeyboardShortcuts(handlers: Record<string, () => void>): void {
    if (!this.preferences.keyboard_navigation) {
      return;
    }

    document.addEventListener('keydown', (event) => {
      // Alt+Shift keyboard shortcuts
      if (event.altKey && event.shiftKey) {
        switch (event.key) {
          case 'C': // Alt+Shift+C for high contrast
            if (handlers['toggle-high-contrast']) {
              handlers['toggle-high-contrast']();
              event.preventDefault();
            }
            break;
          case 'L': // Alt+Shift+L for large text
            if (handlers['toggle-large-text']) {
              handlers['toggle-large-text']();
              event.preventDefault();
            }
            break;
          case 'M': // Alt+Shift+M for reduced motion
            if (handlers['toggle-reduced-motion']) {
              handlers['toggle-reduced-motion']();
              event.preventDefault();
            }
            break;
          case 'S': // Alt+Shift+S for screen reader optimizations
            if (handlers['toggle-screen-reader']) {
              handlers['toggle-screen-reader']();
              event.preventDefault();
            }
            break;
        }
      }
    });
  }
}

// Export singleton instance
export const accessibilityUtils = AccessibilityUtils.getInstance();