/**
 * Type definitions for breach detection models
 */
export interface BreachRecord {
    breach_date: string;
    source: string;
    data_types: string[];
    description: string;
}
export declare enum BreachAction {
    None = "None",
    PasswordReset = "PasswordReset",
    AccountLockout = "AccountLockout"
}
export interface BreachCheckResult {
    is_breached: boolean;
    breaches: BreachRecord[];
    password_compromised: boolean;
    action_required: BreachAction;
}
