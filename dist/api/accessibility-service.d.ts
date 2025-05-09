/**
 * Accessibility service for handling accessibility features
 */
import { ApiClient } from './api-client';
import { AccessibilityPreferences, UpdateAccessibilityPreferencesRequest, CaptchaAlternative, VoiceCommand } from '../types';
export declare class AccessibilityService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Get accessibility preferences for the current user
     */
    getPreferences(): Promise<AccessibilityPreferences>;
    /**
     * Update accessibility preferences
     */
    updatePreferences(request: UpdateAccessibilityPreferencesRequest): Promise<AccessibilityPreferences>;
    /**
     * Get appropriate CAPTCHA alternative based on preferences
     */
    getCaptchaAlternative(): Promise<CaptchaAlternative>;
    /**
     * Process a voice command
     */
    processVoiceCommand(audioData: ArrayBuffer): Promise<VoiceCommand>;
    /**
     * Get CSS variables based on accessibility preferences
     */
    getCssVariables(): Promise<string>;
    /**
     * Get keyboard shortcuts
     */
    getKeyboardShortcuts(): Promise<Record<string, string>>;
}
