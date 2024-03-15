import { API_URL } from "@/environment";
import { createContext } from "react";
import { flow, makeAutoObservable } from "mobx";
import axios, { AxiosError, AxiosResponse } from "axios";
import { User } from "@/types/user";

export enum AuthStatus {
  Authenticated,
  Unauthenticated,
  Pending,
}

export class UserStore {
  authStatus: AuthStatus = AuthStatus.Unauthenticated;
  user: User | null = null;

  setUser(user: User) {
    this.user = user;
    this.authStatus = AuthStatus.Authenticated;
  }

  fetchUser = flow(function* (this: UserStore) {
    try {
      this.authStatus = AuthStatus.Pending;

      const response: AxiosResponse<User> = yield axios.get<User>(`${API_URL}/user`, { withCredentials: true });

      this.user = response.data;
      this.authStatus = AuthStatus.Authenticated;
    } catch (e) {
      const error = e as AxiosError;
      if (error.response?.status === 401) {
        this.authStatus = AuthStatus.Unauthenticated;
      }
    }
  });

  requestLogout = flow(function* (this: UserStore, allDevices) {
    yield axios.delete(`${API_URL}/user?all=${allDevices}`, { withCredentials: true });

    this.logout();
  });

  constructor() {
    makeAutoObservable(this);
  }

  logout() {
    this.authStatus = AuthStatus.Unauthenticated;
    this.user = null;
  }
}

export const userStoreInstance = new UserStore();
export const UserStoreContext = createContext<UserStore>(userStoreInstance);
