/**
 * API client for communicating with the Better Auth backend
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { ErrorResponse } from '../types';

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