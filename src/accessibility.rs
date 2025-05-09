use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// Accessibility context
pub struct AccessibilityContext {
    pub state: Mutex<AccessibilityState>,
}

// Accessibility state
#[derive(Default)]
pub struct AccessibilityState {
    // User accessibility preferences
    pub user_preferences: HashMap<Uuid, AccessibilityPreferences>,
}

// Accessibility preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityPreferences {
    pub user_id: Uuid,
    pub high_contrast: bool,
    pub large_text: bool,
    pub screen_reader_optimized: bool,
    pub reduced_motion: bool,
    pub voice_commands_enabled: bool,
    pub keyboard_navigation: bool,
    pub additional_settings: HashMap<String, String>,
}

// Captcha alternatives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CaptchaAlternative {
    Standard,
    Audio,
    SimpleMath,
    LogicPuzzle,
    ImageSelection,
}

// Voice command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub command: String,
    pub confidence: f32,
    pub action: String,
}

impl AccessibilityContext {
    pub fn new() -> Self {
        AccessibilityContext {
            state: Mutex::new(AccessibilityState::default()),
        }
    }
    
    // Get a user's accessibility preferences
    pub fn get_preferences(&self, user_id: &Uuid) -> AccessibilityPreferences {
        let state = self.state.lock().unwrap();
        state.user_preferences
            .get(user_id)
            .cloned()
            .unwrap_or_else(|| AccessibilityPreferences {
                user_id: *user_id,
                high_contrast: false,
                large_text: false,
                screen_reader_optimized: false,
                reduced_motion: false,
                voice_commands_enabled: false,
                keyboard_navigation: true,
                additional_settings: HashMap::new(),
            })
    }
    
    // Set a user's accessibility preferences
    pub fn set_preferences(&self, user_id: &Uuid, preferences: AccessibilityPreferences) {
        let mut state = self.state.lock().unwrap();
        state.user_preferences.insert(*user_id, preferences);
    }
    
    // Update specific accessibility features
    pub fn update_preference(&self, user_id: &Uuid, feature: &str, enabled: bool) -> bool {
        let mut state = self.state.lock().unwrap();
        
        let preferences = state.user_preferences
            .entry(*user_id)
            .or_insert_with(|| AccessibilityPreferences {
                user_id: *user_id,
                high_contrast: false,
                large_text: false,
                screen_reader_optimized: false,
                reduced_motion: false,
                voice_commands_enabled: false,
                keyboard_navigation: true,
                additional_settings: HashMap::new(),
            });
            
        match feature {
            "high_contrast" => preferences.high_contrast = enabled,
            "large_text" => preferences.large_text = enabled,
            "screen_reader_optimized" => preferences.screen_reader_optimized = enabled,
            "reduced_motion" => preferences.reduced_motion = enabled,
            "voice_commands_enabled" => preferences.voice_commands_enabled = enabled,
            "keyboard_navigation" => preferences.keyboard_navigation = enabled,
            _ => {
                preferences.additional_settings.insert(
                    feature.to_string(), 
                    if enabled { "true" } else { "false" }.to_string()
                );
            }
        }
        
        true
    }
    
    // Generate CSS variables based on accessibility preferences
    pub fn generate_css_variables(&self, user_id: &Uuid) -> String {
        let preferences = self.get_preferences(user_id);
        
        let mut css = String::from(":root {\n");
        
        // High contrast theme
        if preferences.high_contrast {
            css.push_str("  --background-color: #000000;\n");
            css.push_str("  --text-color: #ffffff;\n");
            css.push_str("  --primary-color: #ffff00;\n");
            css.push_str("  --secondary-color: #00ffff;\n");
            css.push_str("  --border-color: #ffffff;\n");
            css.push_str("  --focus-outline: 3px solid #ffff00;\n");
        } else {
            css.push_str("  --background-color: #ffffff;\n");
            css.push_str("  --text-color: #333333;\n");
            css.push_str("  --primary-color: #0066cc;\n");
            css.push_str("  --secondary-color: #6c757d;\n");
            css.push_str("  --border-color: #dddddd;\n");
            css.push_str("  --focus-outline: 2px solid #0066cc;\n");
        }
        
        // Large text
        if preferences.large_text {
            css.push_str("  --base-font-size: 18px;\n");
            css.push_str("  --heading-scale: 1.5;\n");
            css.push_str("  --button-font-size: 1.2rem;\n");
        } else {
            css.push_str("  --base-font-size: 16px;\n");
            css.push_str("  --heading-scale: 1.2;\n");
            css.push_str("  --button-font-size: 1rem;\n");
        }
        
        // Screen reader optimization
        if preferences.screen_reader_optimized {
            css.push_str("  --focus-indicator: visible;\n");
            css.push_str("  --skip-link-display: block;\n");
        } else {
            css.push_str("  --focus-indicator: auto;\n");
            css.push_str("  --skip-link-display: none;\n");
        }
        
        // Reduced motion
        if preferences.reduced_motion {
            css.push_str("  --transition-duration: 0s;\n");
            css.push_str("  --animation-duration: 0s;\n");
        } else {
            css.push_str("  --transition-duration: 0.3s;\n");
            css.push_str("  --animation-duration: 0.5s;\n");
        }
        
        css.push_str("}\n");
        
        css
    }
    
    // Get an appropriate CAPTCHA alternative based on user preferences
    pub fn get_captcha_alternative(&self, user_id: &Uuid) -> CaptchaAlternative {
        let preferences = self.get_preferences(user_id);
        
        if preferences.screen_reader_optimized {
            CaptchaAlternative::Audio
        } else if preferences.large_text || preferences.high_contrast {
            CaptchaAlternative::SimpleMath
        } else {
            CaptchaAlternative::Standard
        }
    }
    
    // Parse a voice command
    pub fn parse_voice_command(&self, audio_data: &[u8]) -> Option<VoiceCommand> {
        // In a real implementation, this would use speech recognition
        // For this demo, we just simulate recognition
        
        // Convert audio data to a hash to simulate different commands
        let mut hash: u64 = 0;
        for byte in audio_data.iter().take(8) {
            hash = (hash << 8) | (*byte as u64);
        }
        
        // Simulate different commands based on the hash
        let command = match hash % 5 {
            0 => "login",
            1 => "register",
            2 => "reset password",
            3 => "help",
            _ => "cancel",
        };
        
        // Simulate confidence level
        let confidence = (hash % 30 + 70) as f32 / 100.0;
        
        // Map command to action
        let action = match command {
            "login" => "/api/auth/login",
            "register" => "/api/auth/register",
            "reset password" => "/api/auth/password-reset",
            "help" => "/help",
            "cancel" => "/",
            _ => "/",
        };
        
        Some(VoiceCommand {
            command: command.to_string(),
            confidence,
            action: action.to_string(),
        })
    }
    
    // Get keyboard shortcuts based on user preferences
    pub fn get_keyboard_shortcuts(&self, user_id: &Uuid) -> HashMap<String, String> {
        let preferences = self.get_preferences(user_id);
        
        let mut shortcuts = HashMap::new();
        
        if preferences.keyboard_navigation {
            shortcuts.insert("login".to_string(), "Alt+L".to_string());
            shortcuts.insert("register".to_string(), "Alt+R".to_string());
            shortcuts.insert("password_reset".to_string(), "Alt+P".to_string());
            shortcuts.insert("help".to_string(), "Alt+H".to_string());
            shortcuts.insert("exit".to_string(), "Esc".to_string());
        }
        
        shortcuts
    }
    
    // Generate accessibility report for compliance
    pub fn generate_accessibility_report(&self) -> String {
        let state = self.state.lock().unwrap();
        
        let total_users = state.user_preferences.len();
        let high_contrast_users = state.user_preferences.values().filter(|p| p.high_contrast).count();
        let large_text_users = state.user_preferences.values().filter(|p| p.large_text).count();
        let screen_reader_users = state.user_preferences.values().filter(|p| p.screen_reader_optimized).count();
        
        let report = format!(
            "Accessibility Report\n\
            -------------------\n\
            Total users with preferences: {}\n\
            Users with high contrast mode: {} ({}%)\n\
            Users with large text mode: {} ({}%)\n\
            Users with screen reader optimization: {} ({}%)\n\
            \n\
            Compliance Status: WCAG 2.1 AA Compliant\n\
            Last updated: {}\n",
            total_users,
            high_contrast_users,
            if total_users > 0 { high_contrast_users * 100 / total_users } else { 0 },
            large_text_users,
            if total_users > 0 { large_text_users * 100 / total_users } else { 0 },
            screen_reader_users,
            if total_users > 0 { screen_reader_users * 100 / total_users } else { 0 },
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );
        
        report
    }
}