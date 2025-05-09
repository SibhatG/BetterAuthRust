/**
 * Type definitions for hybrid encryption models
 */
export interface HybridKeyPair {
    user_id: string;
    rsa_public_key: string;
    kyber_public_key: string;
    created_at: string;
}
export interface HybridEncryptedData {
    rsa_encrypted_key: string;
    kyber_encrypted_key: string;
    encrypted_data: string;
    nonce: string;
    algorithm: string;
}
export interface EncryptRequest {
    data: string;
}
export interface EncryptResponse {
    encrypted_data: HybridEncryptedData;
}
export interface DecryptRequest {
    encrypted_data: HybridEncryptedData;
}
export interface DecryptResponse {
    data: string;
}
export interface RotateKeysResponse {
    key_pair: HybridKeyPair;
}
