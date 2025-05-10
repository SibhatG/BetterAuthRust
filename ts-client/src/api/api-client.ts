/**
 * API client for communicating with the Better Auth backend
 */
import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';

export class ApiClient {
  private readonly client: AxiosInstance;
  private authToken: string | null = null;

  constructor(baseURL: string) {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add auth token to requests if available
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
  public setAuthToken(token: string): void {
    this.authToken = token;
  }

  /**
   * Clear the authentication token
   */
  public clearAuthToken(): void {
    this.authToken = null;
  }

  /**
   * Make a GET request
   */
  public async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    try {
      const response = await this.client.get<T>(url, config);
      return response.data;
    } catch (error) {
      this.handleApiError(error);
      throw error;
    }
  }

  /**
   * Make a POST request
   */
  public async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    try {
      const response = await this.client.post<T>(url, data, config);
      return response.data;
    } catch (error) {
      this.handleApiError(error);
      throw error;
    }
  }

  /**
   * Make a PUT request
   */
  public async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    try {
      const response = await this.client.put<T>(url, data, config);
      return response.data;
    } catch (error) {
      this.handleApiError(error);
      throw error;
    }
  }

  /**
   * Make a PATCH request
   */
  public async patch<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    try {
      const response = await this.client.patch<T>(url, data, config);
      return response.data;
    } catch (error) {
      this.handleApiError(error);
      throw error;
    }
  }

  /**
   * Make a DELETE request
   */
  public async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    try {
      const response = await this.client.delete<T>(url, config);
      return response.data;
    } catch (error) {
      this.handleApiError(error);
      throw error;
    }
  }

  /**
   * Handle API errors
   */
  private handleApiError(error: any): void {
    if (axios.isAxiosError(error) && error.response) {
      const status = error.response.status;
      const data = error.response.data;

      // Handle different error types
      if (status === 401) {
        // Unauthorized - clear token
        this.clearAuthToken();
      }

      // Add more detailed error info to the error object
      error.details = data;
    }
  }
}