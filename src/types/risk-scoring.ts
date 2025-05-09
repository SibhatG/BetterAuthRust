/**
 * Type definitions for risk scoring models
 */

export interface GeoLocation {
  latitude: number;
  longitude: number;
  country: string;
  city: string;
}

export interface LoginRecord {
  timestamp: string;
  ip_address: string;
  location?: GeoLocation;
  device_id: string;
  user_agent: string;
  success: boolean;
}

export interface RiskFactor {
  name: string;
  description: string;
  weight: number;
}

export enum RiskAction {
  Allow = 'Allow',
  RequireMfa = 'RequireMfa',
  Block = 'Block',
}

export interface RiskAnalysisResult {
  score: number;
  factors: RiskFactor[];
  action: RiskAction;
}

export interface RiskAnalysisResponse {
  risk_score: number;
  risk_factors: RiskFactor[];
  recommended_action: string;
}