"use strict";
/**
 * Accessibility service for handling accessibility features
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccessibilityService = void 0;
class AccessibilityService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Get accessibility preferences for the current user
     */
    async getPreferences() {
        return this.apiClient.get('/api/accessibility/preferences');
    }
    /**
     * Update accessibility preferences
     */
    async updatePreferences(request) {
        return this.apiClient.patch('/api/accessibility/preferences', request);
    }
    /**
     * Get appropriate CAPTCHA alternative based on preferences
     */
    async getCaptchaAlternative() {
        return this.apiClient.get('/api/accessibility/captcha-alternative');
    }
    /**
     * Process a voice command
     */
    async processVoiceCommand(audioData) {
        const formData = new FormData();
        formData.append('audio', new Blob([audioData], { type: 'audio/webm' }));
        return this.apiClient.post('/api/accessibility/voice-command', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        });
    }
    /**
     * Get CSS variables based on accessibility preferences
     */
    async getCssVariables() {
        return this.apiClient.get('/api/accessibility/css-variables');
    }
    /**
     * Get keyboard shortcuts
     */
    async getKeyboardShortcuts() {
        return this.apiClient.get('/api/accessibility/keyboard-shortcuts');
    }
}
exports.AccessibilityService = AccessibilityService;
//# sourceMappingURL=accessibility-service.js.map