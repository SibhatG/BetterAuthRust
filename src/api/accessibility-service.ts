/**
 * Accessibility service for handling accessibility features
 */

import { ApiClient } from './api-client';
import { 
  AccessibilityPreferences,
  UpdateAccessibilityPreferencesRequest,
  CaptchaAlternative,
  VoiceCommand
} from '../types';

export class AccessibilityService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Get accessibility preferences for the current user
   */
  public async getPreferences(): Promise<AccessibilityPreferences> {
    return this.apiClient.get<AccessibilityPreferences>('/api/accessibility/preferences');
  }

  /**
   * Update accessibility preferences
   */
  public async updatePreferences(
    request: UpdateAccessibilityPreferencesRequest
  ): Promise<AccessibilityPreferences> {
    return this.apiClient.patch<AccessibilityPreferences>('/api/accessibility/preferences', request);
  }

  /**
   * Get appropriate CAPTCHA alternative based on preferences
   */
  public async getCaptchaAlternative(): Promise<CaptchaAlternative> {
    return this.apiClient.get<CaptchaAlternative>('/api/accessibility/captcha-alternative');
  }

  /**
   * Process a voice command
   */
  public async processVoiceCommand(audioData: ArrayBuffer): Promise<VoiceCommand> {
    const formData = new FormData();
    formData.append('audio', new Blob([audioData], { type: 'audio/webm' }));
    
    return this.apiClient.post<VoiceCommand>('/api/accessibility/voice-command', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    });
  }

  /**
   * Get CSS variables based on accessibility preferences
   */
  public async getCssVariables(): Promise<string> {
    return this.apiClient.get<string>('/api/accessibility/css-variables');
  }

  /**
   * Get keyboard shortcuts
   */
  public async getKeyboardShortcuts(): Promise<Record<string, string>> {
    return this.apiClient.get<Record<string, string>>('/api/accessibility/keyboard-shortcuts');
  }
}