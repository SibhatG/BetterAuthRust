/**
 * API client for communicating with the Better Auth backend
 */
import { AxiosRequestConfig } from 'axios';
export declare class ApiClient {
    private readonly client;
    private authToken;
    constructor(baseURL: string);
    /**
     * Set the authentication token for subsequent requests
     */
    setAuthToken(token: string): void;
    /**
     * Clear the authentication token
     */
    clearAuthToken(): void;
    /**
     * Make a GET request
     */
    get<T>(url: string, config?: AxiosRequestConfig): Promise<T>;
    /**
     * Make a POST request
     */
    post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T>;
    /**
     * Make a PUT request
     */
    put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T>;
    /**
     * Make a PATCH request
     */
    patch<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T>;
    /**
     * Make a DELETE request
     */
    delete<T>(url: string, config?: AxiosRequestConfig): Promise<T>;
    /**
     * Handle API errors
     */
    private handleApiError;
}
