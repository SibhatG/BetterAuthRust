/**
 * Hybrid encryption service for handling encryption operations
 */

import { ApiClient } from './api-client';
import { 
  HybridKeyPair,
  HybridEncryptedData,
  EncryptRequest,
  EncryptResponse,
  DecryptRequest,
  DecryptResponse,
  RotateKeysResponse
} from '../types';

export class HybridEncryptionService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Get public keys for the current user
   */
  public async getPublicKeys(): Promise<HybridKeyPair> {
    return this.apiClient.get<HybridKeyPair>('/api/encryption/keys');
  }

  /**
   * Encrypt data
   */
  public async encrypt(request: EncryptRequest): Promise<EncryptResponse> {
    return this.apiClient.post<EncryptResponse>('/api/encryption/encrypt', request);
  }

  /**
   * Decrypt data
   */
  public async decrypt(request: DecryptRequest): Promise<DecryptResponse> {
    return this.apiClient.post<DecryptResponse>('/api/encryption/decrypt', request);
  }

  /**
   * Rotate encryption keys
   */
  public async rotateKeys(): Promise<RotateKeysResponse> {
    return this.apiClient.post<RotateKeysResponse>('/api/encryption/rotate-keys', {});
  }

  /**
   * Delete encryption keys
   */
  public async deleteKeys(): Promise<boolean> {
    return this.apiClient.delete<boolean>('/api/encryption/keys');
  }

  /**
   * Encrypt a token
   */
  public async encryptToken(token: string): Promise<string> {
    return this.apiClient.post<string>('/api/encryption/token/encrypt', { token });
  }

  /**
   * Decrypt a token
   */
  public async decryptToken(encryptedToken: string): Promise<string> {
    return this.apiClient.post<string>('/api/encryption/token/decrypt', { encrypted_token: encryptedToken });
  }
}