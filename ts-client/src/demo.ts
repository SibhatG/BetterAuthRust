/**
 * Demo file showing how to use the passwordless and accessibility features
 */

import { BetterAuthClient, accessibilityUtils } from './index';

// Create a client instance
const client = new BetterAuthClient('http://localhost:5000');

// Example: Enable accessibility features
function setupAccessibility() {
  // Apply default preferences
  accessibilityUtils.applyPreferences();

  // Setup keyboard shortcuts handler
  accessibilityUtils.setupKeyboardShortcuts({
    'toggle-high-contrast': () => {
      const preferences = accessibilityUtils.getPreferences();
      accessibilityUtils.updatePreferences({
        high_contrast: !preferences.high_contrast
      });
      console.log('High contrast mode:', accessibilityUtils.getPreferences().high_contrast);
    },
    'toggle-large-text': () => {
      const preferences = accessibilityUtils.getPreferences();
      accessibilityUtils.updatePreferences({
        large_text: !preferences.large_text
      });
      console.log('Large text mode:', accessibilityUtils.getPreferences().large_text);
    },
    'toggle-reduced-motion': () => {
      const preferences = accessibilityUtils.getPreferences();
      accessibilityUtils.updatePreferences({
        reduced_motion: !preferences.reduced_motion
      });
      console.log('Reduced motion mode:', accessibilityUtils.getPreferences().reduced_motion);
    },
    'toggle-screen-reader': () => {
      const preferences = accessibilityUtils.getPreferences();
      accessibilityUtils.updatePreferences({
        screen_reader_optimized: !preferences.screen_reader_optimized
      });
      console.log('Screen reader mode:', accessibilityUtils.getPreferences().screen_reader_optimized);
    }
  });

  // Add voice command support
  if (accessibilityUtils.getPreferences().voice_commands_enabled) {
    accessibilityUtils.initVoiceCommands((command) => {
      const action = accessibilityUtils.processVoiceCommand(command);
      console.log('Voice command received:', command, 'Action:', action);
      
      // Handle actions
      if (action === 'toggle-high-contrast') {
        const preferences = accessibilityUtils.getPreferences();
        accessibilityUtils.updatePreferences({
          high_contrast: !preferences.high_contrast
        });
      } else if (action === 'toggle-large-text') {
        const preferences = accessibilityUtils.getPreferences();
        accessibilityUtils.updatePreferences({
          large_text: !preferences.large_text
        });
      } else if (action === 'toggle-reduced-motion') {
        const preferences = accessibilityUtils.getPreferences();
        accessibilityUtils.updatePreferences({
          reduced_motion: !preferences.reduced_motion
        });
      } else if (action === 'login') {
        // Focus the login form
        const loginInput = document.getElementById('username-or-email');
        if (loginInput) {
          (loginInput as HTMLInputElement).focus();
        }
      }
    });
  }

  // Add accessibility stylesheet to document
  const style = document.createElement('style');
  style.textContent = accessibilityUtils.getAccessibilityStylesheet();
  document.head.appendChild(style);
}

// Example: Create an accessible passwordless login form
function createAccessibleLoginForm() {
  // Create a container element if it doesn't exist
  let container = document.getElementById('login-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'login-container';
    document.body.appendChild(container);
  }

  // Create the login form
  client.createAccessiblePasswordlessLoginForm(
    'login-container',
    (message) => {
      console.log('Login success:', message);
      // Handle successful login (e.g., redirect to dashboard)
    },
    (error) => {
      console.error('Login error:', error);
      // Handle login error
    }
  );
}

// Example: Register a new user with passwordless authentication
async function registerWithPasswordless(username: string, email: string, deviceName?: string) {
  try {
    const result = await client.passwordless.startPasswordlessRegistration(
      username,
      email,
      deviceName
    );
    console.log('Registration result:', result);
    return result;
  } catch (error) {
    console.error('Registration error:', error);
    throw error;
  }
}

// Example: Login with passwordless authentication
async function loginWithPasswordless(usernameOrEmail: string) {
  try {
    const result = await client.passwordless.startPasswordlessLogin(usernameOrEmail);
    console.log('Login result:', result);
    return result;
  } catch (error) {
    console.error('Login error:', error);
    throw error;
  }
}

// Example: Update accessibility preferences
function updateAccessibilityPreferences() {
  // Get current preferences
  const currentPrefs = accessibilityUtils.getPreferences();
  console.log('Current preferences:', currentPrefs);

  // Update preferences
  accessibilityUtils.updatePreferences({
    high_contrast: true,
    large_text: true
  });

  console.log('Updated preferences:', accessibilityUtils.getPreferences());
}

// Initialize everything when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  // Setup accessibility features
  setupAccessibility();

  // Create login form
  createAccessibleLoginForm();

  // Add shortcut information for screen readers
  const info = document.createElement('div');
  info.innerHTML = `
    <h3>Keyboard Shortcuts</h3>
    <ul>
      <li>Alt+Shift+C: Toggle high contrast mode</li>
      <li>Alt+Shift+L: Toggle large text mode</li>
      <li>Alt+Shift+M: Toggle reduced motion mode</li>
      <li>Alt+Shift+S: Toggle screen reader optimizations</li>
    </ul>
    <p>You can also use voice commands like "high contrast" or "large text" if voice commands are enabled.</p>
  `;
  document.body.appendChild(info);
});

// Export functions for use in HTML
if (typeof window !== 'undefined') {
  (window as any).auth = {
    register: registerWithPasswordless,
    login: loginWithPasswordless,
    updateAccessibility: updateAccessibilityPreferences
  };
}