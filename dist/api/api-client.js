"use strict";
/**
 * API client for communicating with the Better Auth backend
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ApiClient = void 0;
const axios_1 = __importDefault(require("axios"));
class ApiClient {
    constructor(baseURL) {
        this.authToken = null;
        this.client = axios_1.default.create({
            baseURL,
            headers: {
                'Content-Type': 'application/json',
            },
        });
        // Add request interceptor to include auth token when available
        this.client.interceptors.request.use((config) => {
            if (this.authToken) {
                config.headers.Authorization = `Bearer ${this.authToken}`;
            }
            return config;
        });
    }
    /**
     * Set the authentication token for subsequent requests
     */
    setAuthToken(token) {
        this.authToken = token;
    }
    /**
     * Clear the authentication token
     */
    clearAuthToken() {
        this.authToken = null;
    }
    /**
     * Make a GET request
     */
    async get(url, config) {
        try {
            const response = await this.client.get(url, config);
            return response.data;
        }
        catch (error) {
            this.handleApiError(error);
            throw error;
        }
    }
    /**
     * Make a POST request
     */
    async post(url, data, config) {
        try {
            const response = await this.client.post(url, data, config);
            return response.data;
        }
        catch (error) {
            this.handleApiError(error);
            throw error;
        }
    }
    /**
     * Make a PUT request
     */
    async put(url, data, config) {
        try {
            const response = await this.client.put(url, data, config);
            return response.data;
        }
        catch (error) {
            this.handleApiError(error);
            throw error;
        }
    }
    /**
     * Make a PATCH request
     */
    async patch(url, data, config) {
        try {
            const response = await this.client.patch(url, data, config);
            return response.data;
        }
        catch (error) {
            this.handleApiError(error);
            throw error;
        }
    }
    /**
     * Make a DELETE request
     */
    async delete(url, config) {
        try {
            const response = await this.client.delete(url, config);
            return response.data;
        }
        catch (error) {
            this.handleApiError(error);
            throw error;
        }
    }
    /**
     * Handle API errors
     */
    handleApiError(error) {
        if (axios_1.default.isAxiosError(error) && error.response) {
            const { status, data } = error.response;
            // Log the error
            console.error(`API Error (${status}):`, data);
            // Handle authentication errors
            if (status === 401) {
                this.clearAuthToken();
            }
            // Convert the error to a more friendly format
            if (data && 'message' in data) {
                error.message = data.message;
            }
        }
    }
}
exports.ApiClient = ApiClient;
//# sourceMappingURL=api-client.js.map