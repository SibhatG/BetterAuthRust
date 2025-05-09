/**
 * Hybrid encryption service for handling encryption operations
 */
import { ApiClient } from './api-client';
import { HybridKeyPair, EncryptRequest, EncryptResponse, DecryptRequest, DecryptResponse, RotateKeysResponse } from '../types';
export declare class HybridEncryptionService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Get public keys for the current user
     */
    getPublicKeys(): Promise<HybridKeyPair>;
    /**
     * Encrypt data
     */
    encrypt(request: EncryptRequest): Promise<EncryptResponse>;
    /**
     * Decrypt data
     */
    decrypt(request: DecryptRequest): Promise<DecryptResponse>;
    /**
     * Rotate encryption keys
     */
    rotateKeys(): Promise<RotateKeysResponse>;
    /**
     * Delete encryption keys
     */
    deleteKeys(): Promise<boolean>;
    /**
     * Encrypt a token
     */
    encryptToken(token: string): Promise<string>;
    /**
     * Decrypt a token
     */
    decryptToken(encryptedToken: string): Promise<string>;
}
